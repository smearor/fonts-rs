// build.rs: Fluent UI System Icons hybrid build - pre-rendered SVGs + TTF.
//
// The style variant (regular | filled) is selected by Cargo feature;
// default is `regular`. GResource SVGs come from the pre-rendered
// `resources/svg/{style}-24/` slice; the matching
// `FluentSystemIcons-{Style}.ttf` is bundled for pango/html/render.
//
// Glyph names are the upstream icon names
// (`ic_fluent_{name}_{size}_{style}.svg` -> `fluent-{name}-symbolic` /
// `fluent-filled-{name}-symbolic`), normalized to kebab-case. Codepoints
// are the real upstream codepoints from the vendored
// `FluentSystemIcons-{Style}.json` maps (flat `{"ic_fluent_...": int}`
// maps), resolved per glyph via `CodepointStrategy::FromNameMap` -
// including the ~26% allocated in Supplementary PUA-A (> U+FFFF).
//
// `CurrentColor` rewrites the upstream hardcoded `fill="#212121"` so
// GTK symbolic recoloring works; a post-export assertion fails the
// build if any hardcoded hex fill survives.
//
// Metadata: `resources/metadata/hidden.json` lists deprecated upstream
// icons (kebab names) and is consumed as `name_filter` input - hidden
// icons are excluded from the export in every build. With the
// `metadata` feature enabled, `aliases.json` (iconify kebab->kebab
// rename table) plus filename-derived keywords feed the
// `MetadataGenerator` phf maps.
//
// Change detection: `extra_hash` carries the whole hash - variant name,
// export pipeline version, SVG directory, codepoint map JSON and the
// metadata JSONs.

use std::collections::HashSet;
use std::path::Path;

use fonts_rs_generator::CodepointStrategy;
use fonts_rs_generator::CurrentColor;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontFamilyConfigExt;
use fonts_rs_generator::GlyphSource;
use fonts_rs_generator::GlyphSourceError;
use fonts_rs_generator::MetadataGenerator;
use fonts_rs_generator::PaintTarget;
use fonts_rs_generator::SvgDirSource;
use fonts_rs_generator::VariantList;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::hash_str;
use fonts_rs_generator::hash_svg_dir;
use fonts_rs_generator::normalize_to_kebab;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_generator::stem_keywords;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::FontVariant;
use fonts_rs_model::GlyphEntry;
use fonts_rs_model::GlyphNameMap;

#[path = "src/definition.rs"]
mod definition;

#[path = "src/filename_parser.rs"]
mod filename_parser;

use definition::FluentConfig;
use filename_parser::FluentFilenameParser;

/// All Fluent variants and their mapping to TTF filenames.
///
/// If no `regular`/`filled` Cargo feature is active, `regular` is used.
const VARIANTS: VariantList<FluentConfig> = VariantList::new(&[
    FontVariant::file("regular", "FluentSystemIcons-Regular"),
    FontVariant::file("filled", "FluentSystemIcons-Filled"),
]);

/// Grid size of the vendored SVG slices (v1: only the 24px slice).
const GRID_SIZE: u16 = 24;

/// Export pipeline version.
///
/// Mixed into `FontBuild::extra_hash`. Bump when the export pipeline
/// itself changes in ways the hash cannot see - e.g. adding or
/// modifying `SvgPostProcessor`s - otherwise a matching hash would
/// reuse a stale export.
const EXPORT_PIPELINE_VERSION: u64 = 1;

/// Parses `FluentSystemIcons-{Style}.json` (flat
/// `{"ic_fluent_add_24_regular": 57994}` integer map) into the
/// `GlyphNameMap` for this slice.
///
/// `GlyphNameMap::Deserialize` accepts the flat int shape directly; the
/// remaining work is vendor-specific key normalization: keep only the
/// `_{size}_{style}` entries and strip the `ic_fluent_` prefix + suffix
/// so keys match `ParsedGlyph::raw_name` (`add`, `i_o_t`).
fn fluent_codepoint_map(json: &str, size: u16, style: &str) -> miette::Result<GlyphNameMap> {
    let suffix = format!("_{size}_{style}");
    let map: GlyphNameMap = serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse codepoint map: {e}"))?;
    let mut entries = GlyphNameMap::new();
    for (key, codepoint) in map {
        if !key.ends_with(&suffix) {
            continue;
        }
        let name = key
            .strip_prefix("ic_fluent_")
            .and_then(|stem| stem.strip_suffix(&suffix))
            .ok_or_else(|| miette::miette!("codepoint map key '{key}' does not match ic_fluent_*{suffix}"))?;
        entries.insert(name.to_string(), codepoint);
    }
    Ok(entries)
}

/// Returns `style` with an uppercase first letter (`"regular"` -> `"Regular"`).
fn title_case(style: &str) -> String {
    let mut chars = style.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

/// `MetadataGenerator` for the Fluent set.
///
/// Keywords are filename-derived: the kebab stem of each exported
/// glyph name splits into word tokens (see [`stem_keywords`]). Aliases
/// come from the vendored `aliases.json` (iconify kebab->kebab rename
/// table). Fluent ships no categories upstream, so the category map
/// stays empty.
struct FluentMetadataGenerator {
    /// Glyph name prefix for this variant (`fluent` / `fluent-filled`).
    glyph_prefix: String,
    /// `(alias, canonical)` pairs using full `fluent-*-symbolic` names.
    aliases: Vec<(String, String)>,
}

impl FluentMetadataGenerator {
    /// Returns the upstream kebab stem by stripping `{prefix}-` and
    /// `-symbolic` from an exported glyph name.
    fn upstream_key<'a>(&self, glyph_name: &'a str) -> &'a str {
        glyph_name
            .strip_prefix(&format!("{}-", self.glyph_prefix))
            .and_then(|name| name.strip_suffix("-symbolic"))
            .unwrap_or(glyph_name)
    }
}

impl MetadataGenerator for FluentMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        stem_keywords(self.upstream_key(&entry.name))
    }

    fn categories_for(&self, _entry: &GlyphEntry<String>) -> Vec<String> {
        Vec::new()
    }

    fn aliases(&self) -> Vec<(String, String)> {
        self.aliases.clone()
    }
}

/// Parses `aliases.json` (iconify kebab->kebab rename table) into
/// `(alias, canonical)` pairs with full `{prefix}-...-symbolic` glyph
/// names.
///
/// Entries whose canonical name is not exported are dropped - the
/// table covers all upstream sizes and styles, not just this slice.
fn parse_aliases(json: &str, glyph_prefix: &str, exported: &HashSet<&str>) -> miette::Result<Vec<(String, String)>> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse aliases.json: {e}"))?;
    let aliases = value.as_object().ok_or_else(|| miette::miette!("aliases.json is not an object"))?;
    Ok(aliases
        .iter()
        .filter_map(|(alias, canonical)| canonical.as_str().map(|canonical| (alias, canonical)))
        .filter(|(alias, canonical)| alias.as_str() != *canonical)
        .map(|(alias, canonical)| (format!("{glyph_prefix}-{alias}-symbolic"), format!("{glyph_prefix}-{canonical}-symbolic")))
        .filter(|(_, canonical)| exported.contains(canonical.as_str()))
        .collect())
}

/// Fails the build if any emitted GResource SVG still contains a
/// hardcoded `fill="#..."` attribute.
///
/// `CurrentColor` should have rewritten every upstream `fill="#212121"`;
/// `fill="none"` and `fill="currentColor"` remain legal. A surviving
/// hex fill means the post-processor lost coverage - report the
/// offending file.
fn assert_no_hex_fill(resources_dir: &Path) -> Result<(), GlyphSourceError> {
    let icons_dir = FluentConfig::icons_dir(resources_dir);
    for entry in std::fs::read_dir(&icons_dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "svg") {
            let svg = std::fs::read_to_string(&path)?;
            if svg.contains("fill=\"#") {
                return Err(GlyphSourceError::PostProcess(format!(
                    "{}: hardcoded hex fill survived the CurrentColor rewrite",
                    path.display()
                )));
            }
        }
    }
    Ok(())
}

fn main() -> miette::Result<()> {
    let entry = VARIANTS.detect_and_get_active_variant(0)?;
    let style = entry.as_str();

    // The TTFs are vendored under resources/fonts/ (matching the
    // upstream lock file layout); `FontVariantExt::font_path` assumes a
    // flat resources/ layout, so the path is built explicitly.
    let font_file = entry.font_file().ok_or_else(|| miette::miette!("variant '{entry}' has no font file"))?;
    let font_path = Path::new(build_constants::RESOURCES_DIR)
        .join("fonts")
        .join(format!("{}.ttf", font_file.as_str()));

    eprintln!("build.rs: active variant: {style} -> {}", font_path.display());

    // Set TTF path for pixel-drawing (ab_glyph) at runtime.
    set_font_path_env("FLUENT_FONT_PATH", &font_path)?;

    // Vendored upstream codepoint map: flat int map keyed by
    // `ic_fluent_{name}_{size}_{style}`. Normalized to the glyph stems
    // produced by FluentFilenameParser (e.g. `i_o_t`) so
    // `CodepointStrategy::FromNameMap` can resolve every SVG.
    let metadata_dir = Path::new(build_constants::RESOURCES_DIR).join("metadata");
    let map_path = metadata_dir.join(format!("FluentSystemIcons-{}.json", title_case(style)));
    let map_json = std::fs::read_to_string(&map_path).map_err(|e| miette::miette!("Failed to read {}: {e}", map_path.display()))?;
    let codepoints = fluent_codepoint_map(&map_json, GRID_SIZE, style)?;
    eprintln!("build.rs: {} codepoints for {style}-{GRID_SIZE}", codepoints.len());

    // Vendored metadata JSONs. `hidden.json` (kebab names) drives the
    // SvgDirSource name_filter; `aliases.json` is parsed lazily when
    // the `metadata` feature is enabled.
    let aliases_json = std::fs::read_to_string(metadata_dir.join("aliases.json")).map_err(|e| miette::miette!("Failed to read aliases.json: {e}"))?;
    let hidden_json = std::fs::read_to_string(metadata_dir.join("hidden.json")).map_err(|e| miette::miette!("Failed to read hidden.json: {e}"))?;
    let hidden_names: Vec<String> = serde_json::from_str(&hidden_json).map_err(|e| miette::miette!("Failed to parse hidden.json: {e}"))?;
    let hidden_stems: HashSet<String> = hidden_names.iter().map(|name| name.to_lowercase().replace('-', "_")).collect();
    eprintln!("build.rs: {} hidden icon(s) excluded", hidden_stems.len());

    let svg_dir = Path::new(build_constants::RESOURCES_DIR).join("svg").join(format!("{style}-{GRID_SIZE}"));

    // `with_variant` yields "fluent-regular-{name}" for the default
    // variant - drop the style infix, keep "fluent-filled-" for filled.
    let mut config = ExportConfig::<FluentConfig>::with_variant(*entry);
    if style == "regular" {
        config.glyph_name_prefix = FluentConfig::FONT_FAMILY_NAME.to_string();
    }

    FontBuild::new(&font_path)
        .compile_font_gresource()
        .gresource_prefix(FluentConfig::GRESOURCE_PREFIX)
        .extra_hash(format!("{style}-{EXPORT_PIPELINE_VERSION}"))
        .extra_hash(format!("{:016x}", hash_svg_dir(&svg_dir)))
        .extra_hash(format!("{:016x}", hash_str(&map_json)))
        .extra_hash(format!("{:016x}", hash_str(&hidden_json)))
        .extra_hash(format!("{:016x}", hash_str(&aliases_json)))
        .rerun_if_changed(&svg_dir)
        .rerun_if_changed(&metadata_dir)
        .run(|_font_path, resources_dir| {
            let count = SvgDirSource::new(&svg_dir)
                .filename_parser(FluentFilenameParser::new(GRID_SIZE, style))
                .codepoint_map(&codepoints)
                .codepoint_strategy(CodepointStrategy::FromNameMap)
                .name_resolver(|parsed| normalize_to_kebab(&parsed.raw_name).map(|stem| format!("{stem}-symbolic")))
                .name_filter(move |parsed| !hidden_stems.contains(&parsed.raw_name))
                .name_prefix(&config.glyph_name_prefix)
                .post_process(CurrentColor::new(PaintTarget::Fill))
                .export(&config, resources_dir)?;
            assert_no_hex_fill(resources_dir)?;
            Ok::<usize, GlyphSourceError>(count)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    // Generate variant info for runtime use.
    config.write_variant_info()?;

    // Generate metadata (filename-derived keywords + vendored aliases)
    // if enabled. `entries` come from the freshly written metadata.json;
    // aliases only resolve to exported glyph names.
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating icon metadata (filename keywords + aliases)...");

        let metadata_json = std::fs::read_to_string(build_constants::metadata_path()).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;
        let exported: HashSet<&str> = entries.iter().map(|entry| entry.name.as_str()).collect();
        let aliases = parse_aliases(&aliases_json, &config.glyph_name_prefix, &exported)?;
        eprintln!("build.rs: {} aliases resolve to exported glyphs", aliases.len());

        FluentMetadataGenerator {
            glyph_prefix: config.glyph_name_prefix.clone(),
            aliases,
        }
        .run(&entries)
        .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
