// build.rs: Eva Icons build - pre-rendered SVGs + webfont TTF.
//
// Both upstream styles are exported in one build via `CombinedSource`:
// `resources/svg/fill` -> `eva-{name}-symbolic`,
// `resources/svg/outline` -> `eva-{name}-outline-symbolic` (the
// upstream `-outline` filename/CSS suffix is kept as the name suffix).
// Both styles share the single `Eva-Icons.ttf`; its fill and outline
// glyphs live at different codepoints, resolved per glyph from the
// upstream stylesheet `resources/metadata/eva-icons.css`
// (`content: "\eXXX"` rules).
//
// Keyword metadata comes from upstream `icons-tags.ts` (the search tags
// powering the official eva-icons.com cheat sheet), vendored as
// `resources/metadata/icons-tags.json`. Style categories ("fill" /
// "outline") are derived from the glyph name; Eva ships no aliases.

use std::path::Path;

use fonts_rs_generator::CodepointStrategy;
use fonts_rs_generator::CombinedSource;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::GlyphSource;
use fonts_rs_generator::MetadataGenerator;
use fonts_rs_generator::SvgDirSource;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::hash_str;
use fonts_rs_generator::hash_svg_dir;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::CodePoint;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GlyphEntry;
use fonts_rs_model::GlyphNameMap;

#[path = "src/definition.rs"]
mod definition;

use definition::EvaConfig;

/// Export pipeline version.
///
/// Mixed into `FontBuild::extra_hash`. Bump when the export pipeline
/// itself changes in ways the hash cannot see - e.g. adding or
/// modifying `SvgPostProcessor`s - otherwise a matching hash would
/// reuse a stale export.
const EXPORT_PIPELINE_VERSION: u64 = 1;

/// Parses `eva-icons.css` into a name → codepoint map.
///
/// The stylesheet carries one rule per icon class:
///
/// ```css
/// .eva-activity::before {
///     content: "\ea01";
/// }
/// ```
///
/// Outline icons use the `-outline` class suffix (`eva-activity-outline`),
/// matching the `resources/svg/outline/{name}-outline.svg` filenames, so
/// the map is keyed by the SVG filename stem (`CodepointStrategy::
/// FromNameMap` looks up `ParsedGlyph::raw_name`). Non-icon utility
/// classes (`.eva`, `.eva-lg`, `.eva-2x`, …) carry no `::before` rule
/// and are skipped.
fn parse_codepoint_map(css: &str) -> GlyphNameMap {
    let mut map = GlyphNameMap::new();
    for rule in css.split(".eva-").skip(1) {
        let Some((name, body)) = rule.split_once("::before") else { continue };
        let Some(content) = body.split("content:").nth(1) else { continue };
        let Some(hex) = content.trim_start().strip_prefix('"').and_then(|content| content.strip_prefix('\\')) else {
            continue;
        };
        let Some(hex) = hex.split('"').next() else { continue };
        let Ok(value) = u32::from_str_radix(hex.trim(), 16) else { continue };
        let Some(ch) = char::from_u32(value) else { continue };
        map.insert(name.trim().to_string(), CodePoint::from(ch));
    }
    map
}

/// `MetadataGenerator` backed by upstream `icons-tags.json` (the search
/// tags from `icons-tags.ts` in the akveo/eva-icons website repo).
///
/// Maps each exported glyph's `eva-{name}-symbolic` name back to its
/// upstream icon entry; the tag list becomes the keywords. Eva ships
/// no categories or aliases, so only the keyword map is populated.
struct EvaMetadataGenerator {
    /// Parsed `icons-tags.json` object keyed by upstream icon name.
    tags: serde_json::Map<String, serde_json::Value>,
}

impl EvaMetadataGenerator {
    /// Returns the upstream icon name for a glyph name by stripping
    /// the `eva-` prefix and `-symbolic` suffix; outline glyphs keep
    /// their `-outline` marker so `icons-tags.json` lookups hit the
    /// upstream outline entries.
    fn upstream_key(glyph_name: &str) -> &str {
        glyph_name
            .strip_prefix("eva-")
            .and_then(|name| name.strip_suffix("-symbolic"))
            .unwrap_or(glyph_name)
    }
}

impl MetadataGenerator for EvaMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        self.tags
            .get(Self::upstream_key(&entry.name))
            .and_then(serde_json::Value::as_array)
            .map(|tags| tags.iter().filter_map(|tag| tag.as_str().map(String::from)).collect())
            .unwrap_or_default()
    }

    /// Every glyph gets its style as category ("fill" / "outline") -
    /// upstream ships no categories, so the style is the only label.
    fn categories_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        vec![if Self::upstream_key(&entry.name).ends_with("-outline") {
            "outline".to_string()
        } else {
            "fill".to_string()
        }]
    }
}

fn main() -> miette::Result<()> {
    let font_path = Path::new(build_constants::RESOURCES_DIR).join("Eva-Icons.ttf");

    eprintln!("build.rs: combined fill+outline export -> {}", font_path.display());

    // Set TTF path for pixel-drawing (ab_glyph) at runtime.
    set_font_path_env("EVA_FONT_PATH", &font_path)?;

    let metadata_dir = Path::new(build_constants::RESOURCES_DIR).join("metadata");
    let css = std::fs::read_to_string(metadata_dir.join("eva-icons.css")).map_err(|e| miette::miette!("Failed to read eva-icons.css: {e}"))?;
    let tags_json = std::fs::read_to_string(metadata_dir.join("icons-tags.json")).map_err(|e| miette::miette!("Failed to read icons-tags.json: {e}"))?;

    let codepoint_map = parse_codepoint_map(&css);
    eprintln!("build.rs: {} codepoints in eva-icons.css", codepoint_map.len());

    let fill_dir = Path::new(build_constants::RESOURCES_DIR).join("svg").join("fill");
    let outline_dir = Path::new(build_constants::RESOURCES_DIR).join("svg").join("outline");

    // GResource SVGs: copy/rename pre-rendered SVGs from both style
    // directories into one export. The `-outline` filename suffix
    // stays in the glyph name (`eva-{name}-outline-symbolic`), so
    // fill and outline partition the namespace. Runs inside
    // FontBuild's export closure so metadata.json and
    // icons.gresource.xml land in OUT_DIR where the GResource
    // compilation and code generation expect them.
    let config = ExportConfig::<EvaConfig>::new();
    FontBuild::new(&font_path)
        .compile_font_gresource()
        .gresource_prefix(EvaConfig::GRESOURCE_PREFIX)
        .extra_hash(EXPORT_PIPELINE_VERSION.to_string())
        .extra_hash(format!("{:016x}", hash_svg_dir(&fill_dir)))
        .extra_hash(format!("{:016x}", hash_svg_dir(&outline_dir)))
        .extra_hash(format!("{:016x}", hash_str(&css)))
        .extra_hash(format!("{:016x}", hash_str(&tags_json)))
        .run(|_font_path, resources_dir| {
            let source = |dir: &Path| {
                SvgDirSource::new(dir)
                    .codepoint_strategy(CodepointStrategy::FromNameMap)
                    .codepoint_map(&codepoint_map)
                    .name_resolver(|parsed| Some(format!("{}-symbolic", parsed.raw_name)))
                    .name_prefix(EvaConfig::FONT_FAMILY_NAME)
            };
            CombinedSource::new()
                // More specific marker first: outline names contain
                // "-outline", every name contains "eva-".
                .source("-outline", source(&outline_dir))
                .source("eva-", source(&fill_dir))
                .export(&config, resources_dir)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    // Generate metadata (keywords + style categories - Eva ships no
    // aliases) if enabled.
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating icon metadata (keywords + style categories)...");

        let tags_value: serde_json::Value = serde_json::from_str(&tags_json).map_err(|e| miette::miette!("Failed to parse icons-tags.json: {e}"))?;
        let tags = tags_value.as_object().cloned().unwrap_or_default();

        let metadata_json = std::fs::read_to_string(build_constants::metadata_path()).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;

        EvaMetadataGenerator { tags }
            .run(&entries)
            .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
