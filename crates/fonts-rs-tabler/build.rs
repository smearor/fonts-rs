// build.rs: Tabler Icons build — pre-rendered SVGs + webfont TTF.
//
// The variant (outline | filled) is selected by Cargo feature; default
// is `outline`. Glyph names are variant-independent
// (`tabler-{name}-symbolic`) — the variant only selects which SVG set
// and TTF ship. Codepoints come from `resources/metadata/icons.json`
// (`styles.{variant}.unicode`): filled icons use different codepoints
// than their outline counterparts.

use std::path::Path;

use fonts_rs_generator::CodepointStrategy;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontVariantExt;
use fonts_rs_generator::GlyphSource;
use fonts_rs_generator::MetadataGenerator;
use fonts_rs_generator::SvgDirSource;
use fonts_rs_generator::VariantList;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::CodePoint;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::FontVariant;
use fonts_rs_model::GlyphEntry;
use fonts_rs_model::GlyphNameMap;

#[path = "src/definition.rs"]
mod definition;

use definition::TablerConfig;

/// All Tabler variants and their mapping to TTF filenames.
///
/// `outline` (default) is the classic stroke-based icon style covering
/// all ~5,100 icons; `filled` ships solid artwork for a subset of
/// ~1,000 icons.
const VARIANTS: VariantList<TablerConfig> =
    VariantList::new(&[FontVariant::file("outline", "tabler-icons"), FontVariant::file("filled", "tabler-icons-filled")]);

/// Export pipeline version.
///
/// Mixed into `FontBuild::extra_hash`. Bump when the export pipeline
/// itself changes in ways the hash cannot see — e.g. adding or
/// modifying `SvgPostProcessor`s — otherwise a matching hash would
/// reuse a stale export.
const EXPORT_PIPELINE_VERSION: u64 = 1;

/// FNV-1a hash over a string's bytes.
fn hash_str(content: &str) -> u64 {
    content
        .bytes()
        .fold(0xcbf29ce484222325_u64, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3))
}

/// FNV-1a hash over the sorted SVG filenames and file contents in
/// `svg_dir`.
///
/// Mixed into `FontBuild::extra_hash` so changes to `resources/svg/`
/// force a re-export even when the TTF file is unchanged — the default
/// hash only covers the font file.
fn hash_svg_dir(svg_dir: &Path) -> u64 {
    let mut names: Vec<String> = std::fs::read_dir(svg_dir)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names.iter().fold(0xcbf29ce484222325_u64, |hash, name| {
        let content = std::fs::read(svg_dir.join(name)).unwrap_or_default();
        name.bytes()
            .chain(content.iter().copied())
            .fold(hash, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3))
    })
}

/// Parses the `icons.json` object into a name → codepoint map for the
/// active style.
///
/// Each icon carries per-style `unicode` values; filled icons use
/// different codepoints than their outline counterparts. Icons without
/// an entry for the active style are skipped.
fn parse_codepoint_map(icons: &serde_json::Map<String, serde_json::Value>, style: &str) -> GlyphNameMap {
    let mut map = GlyphNameMap::new();
    for (name, icon) in icons {
        let Some(unicode) = icon
            .get("styles")
            .and_then(|styles| styles.get(style))
            .and_then(|style| style.get("unicode"))
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        let Ok(value) = u32::from_str_radix(unicode, 16) else { continue };
        let Some(ch) = char::from_u32(value) else { continue };
        map.insert(name.clone(), CodePoint::from(ch));
    }
    map
}

/// Parses `aliases.json` for the active style into `(alias, canonical)`
/// pairs using full `tabler-*-symbolic` glyph names.
fn parse_aliases(json: &str, style: &str) -> miette::Result<Vec<(String, String)>> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse aliases.json: {e}"))?;
    let aliases = value
        .get(style)
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| miette::miette!("aliases.json has no '{style}' table"))?;
    Ok(aliases
        .iter()
        .filter_map(|(alias, canonical)| {
            canonical
                .as_str()
                .map(|canonical| (format!("tabler-{alias}-symbolic"), format!("tabler-{canonical}-symbolic")))
        })
        .collect())
}

/// `MetadataGenerator` backed by upstream `icons.json` and `aliases.json`.
///
/// Maps each exported glyph's `tabler-{name}-symbolic` name back to its
/// upstream icon entry: `tags` become keywords, `category` becomes the
/// category, and the active style's alias table becomes `(alias,
/// canonical)` glyph-name pairs.
struct TablerMetadataGenerator {
    /// Parsed `icons.json` object keyed by upstream icon name.
    icons: serde_json::Map<String, serde_json::Value>,
    /// `(alias, canonical)` pairs using full `tabler-*-symbolic` names.
    aliases: Vec<(String, String)>,
}

impl TablerMetadataGenerator {
    /// Returns the upstream `icons.json` key for a glyph name by
    /// stripping the `tabler-` prefix and `-symbolic` suffix.
    fn upstream_key(glyph_name: &str) -> &str {
        glyph_name
            .strip_prefix("tabler-")
            .and_then(|name| name.strip_suffix("-symbolic"))
            .unwrap_or(glyph_name)
    }
}

impl MetadataGenerator for TablerMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        self.icons
            .get(Self::upstream_key(&entry.name))
            .and_then(|icon| icon.get("tags"))
            .and_then(serde_json::Value::as_array)
            .map(|tags| tags.iter().filter_map(|tag| tag.as_str().map(String::from)).collect())
            .unwrap_or_default()
    }

    fn categories_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        self.icons
            .get(Self::upstream_key(&entry.name))
            .and_then(|icon| icon.get("category"))
            .and_then(serde_json::Value::as_str)
            .map(|category| vec![category.to_string()])
            .unwrap_or_default()
    }

    fn aliases(&self) -> Vec<(String, String)> {
        self.aliases.clone()
    }
}

fn main() -> miette::Result<()> {
    let entry = VARIANTS.detect_and_get_active_variant(0)?;
    let font_path = entry.font_path()?;
    let style = entry.as_str().to_string();

    eprintln!("build.rs: active variant: {style} -> {}", font_path.display());

    // Set TTF path for pixel-drawing (ab_glyph) at runtime.
    set_font_path_env("TABLER_FONT_PATH", &font_path)?;

    let metadata_dir = Path::new(build_constants::RESOURCES_DIR).join("metadata");
    let icons_json = std::fs::read_to_string(metadata_dir.join("icons.json")).map_err(|e| miette::miette!("Failed to read icons.json: {e}"))?;
    let icons_value: serde_json::Value = serde_json::from_str(&icons_json).map_err(|e| miette::miette!("Failed to parse icons.json: {e}"))?;
    let icons = icons_value.as_object().cloned().unwrap_or_default();

    let codepoint_map = parse_codepoint_map(&icons, &style);
    eprintln!("build.rs: {} codepoints for style '{style}'", codepoint_map.len());

    let svg_dir = Path::new(build_constants::RESOURCES_DIR).join("svg").join(&style);

    // GResource SVGs: copy/rename pre-rendered SVGs from
    // resources/svg/{style}/ using upstream icon names. Runs inside
    // FontBuild's export closure so metadata.json and
    // icons.gresource.xml land in OUT_DIR where the GResource
    // compilation and code generation expect them.
    let config = ExportConfig::<TablerConfig>::new();
    FontBuild::new(&font_path)
        .compile_font_gresource()
        .gresource_prefix(TablerConfig::GRESOURCE_PREFIX)
        .extra_hash(style.clone())
        .extra_hash(EXPORT_PIPELINE_VERSION.to_string())
        .extra_hash(format!("{:016x}", hash_svg_dir(&svg_dir)))
        .extra_hash(format!("{:016x}", hash_str(&icons_json)))
        .run(|_font_path, resources_dir| {
            SvgDirSource::new(&svg_dir)
                .codepoint_strategy(CodepointStrategy::FromNameMap)
                .codepoint_map(&codepoint_map)
                .name_resolver(|parsed| Some(format!("{}-symbolic", parsed.raw_name)))
                .name_prefix(TablerConfig::FONT_FAMILY_NAME)
                .export(&config, resources_dir)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    // Generate metadata (keywords + categories + aliases) if enabled.
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating icon metadata (keywords/categories/aliases)...");

        let aliases_json = std::fs::read_to_string(metadata_dir.join("aliases.json")).map_err(|e| miette::miette!("Failed to read aliases.json: {e}"))?;
        let aliases = parse_aliases(&aliases_json, &style)?;

        let metadata_json = std::fs::read_to_string(build_constants::metadata_path()).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;

        TablerMetadataGenerator { icons, aliases }
            .run(&entries)
            .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
