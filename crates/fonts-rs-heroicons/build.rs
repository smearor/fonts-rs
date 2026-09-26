// build.rs: Heroicons build - fontless SVG-only icon set.
//
// The variant (outline | solid | mini | micro) is selected by Cargo
// feature; default is `outline`. Glyph names are variant-independent
// (`heroicons-{name}-symbolic`) - the variant only selects which SVG
// set ships:
//
// - `outline` (default): 24x24 stroke-based (`stroke="currentColor"`,
//   `fill="none"`) - `SymbolicClasses` adds the GTK symbolic classes
//   (`transparent-fill`/`foreground-stroke`) that keep stroke details
//   visible under symbolic recoloring.
// - `solid`:   24x24 filled (`fill="currentColor"`).
// - `mini`:    20x20 filled.
// - `micro`:   16x16 filled.
//
// Codepoints are stable PUA allocations via `CodepointStrategy::HashedPua`
// (upstream ships no codepoint table and no font).
//
// Metadata: `resources/metadata/keywords.json` carries the search
// keywords used by heroicons.com (extracted from the site's JS bundle),
// `resources/metadata/aliases.json` maps Heroicons v1 names to their
// v2 canonical names (from the iconify heroicons set's alias table).
//
// Change detection: with no font file, `extra_hash` carries the whole
// hash - variant name, SVG directory and both metadata JSONs.

use std::path::Path;

use fonts_rs_generator::CodepointStrategy;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::GlyphSource;
use fonts_rs_generator::MetadataGenerator;
use fonts_rs_generator::SvgDirSource;
use fonts_rs_generator::SymbolicClasses;
use fonts_rs_generator::VariantList;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::hash_str;
use fonts_rs_generator::hash_svg_dir;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::FontVariant;
use fonts_rs_model::GlyphEntry;

#[path = "src/definition.rs"]
mod definition;

use definition::HeroiconsConfig;

/// All Heroicons variants.
///
/// Fontless set: `FontVariant::axes(name, &[])` marks the variant as
/// file-less - only the name matters for feature detection.
const VARIANTS: VariantList<HeroiconsConfig> = VariantList::new(&[
    FontVariant::axes("outline", &[]),
    FontVariant::axes("solid", &[]),
    FontVariant::axes("mini", &[]),
    FontVariant::axes("micro", &[]),
]);

/// Export pipeline version.
///
/// Mixed into `FontBuild::extra_hash`. Bump when the export pipeline
/// itself changes in ways the hash cannot see - e.g. adding or
/// modifying `SvgPostProcessor`s - otherwise a matching hash would
/// reuse a stale export.
const EXPORT_PIPELINE_VERSION: u64 = 1;

/// `MetadataGenerator` backed by the vendored `keywords.json`
/// (heroicons.com search keywords) and `aliases.json` (v1→v2 names).
///
/// Maps each exported glyph's `heroicons-{name}-symbolic` name back to
/// its upstream icon entry. Heroicons ships no categories, so only
/// keywords and aliases are populated.
struct HeroiconsMetadataGenerator {
    /// Parsed `keywords.json` object keyed by upstream icon name.
    keywords: serde_json::Map<String, serde_json::Value>,
    /// `(alias, canonical)` pairs using full `heroicons-*-symbolic`
    /// glyph names.
    aliases: Vec<(String, String)>,
}

impl HeroiconsMetadataGenerator {
    /// Returns the upstream icon name for a glyph name by stripping the
    /// `heroicons-` prefix and `-symbolic` suffix.
    fn upstream_key(glyph_name: &str) -> &str {
        glyph_name
            .strip_prefix("heroicons-")
            .and_then(|name| name.strip_suffix("-symbolic"))
            .unwrap_or(glyph_name)
    }
}

impl MetadataGenerator for HeroiconsMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        self.keywords
            .get(Self::upstream_key(&entry.name))
            .and_then(serde_json::Value::as_array)
            .map(|items| items.iter().filter_map(|item| item.as_str().map(String::from)).collect())
            .unwrap_or_default()
    }

    fn categories_for(&self, _entry: &GlyphEntry<String>) -> Vec<String> {
        Vec::new()
    }

    fn aliases(&self) -> Vec<(String, String)> {
        self.aliases.clone()
    }
}

/// Parses `aliases.json` into `(alias, canonical)` pairs using full
/// `heroicons-*-symbolic` glyph names.
fn parse_aliases(json: &str) -> miette::Result<Vec<(String, String)>> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse aliases.json: {e}"))?;
    let aliases = value.as_object().ok_or_else(|| miette::miette!("aliases.json is not an object"))?;
    Ok(aliases
        .iter()
        .filter_map(|(alias, canonical)| {
            canonical
                .as_str()
                .map(|canonical| (format!("heroicons-{alias}-symbolic"), format!("heroicons-{canonical}-symbolic")))
        })
        .collect())
}

fn main() -> miette::Result<()> {
    let variant = VARIANTS.detect_and_get_active_variant(0)?;
    let style = variant.as_str().to_string();

    eprintln!("build.rs: active variant: {style}");

    let metadata_dir = Path::new(build_constants::RESOURCES_DIR).join("metadata");
    let keywords_json = std::fs::read_to_string(metadata_dir.join("keywords.json")).map_err(|e| miette::miette!("Failed to read keywords.json: {e}"))?;
    let aliases_json = std::fs::read_to_string(metadata_dir.join("aliases.json")).map_err(|e| miette::miette!("Failed to read aliases.json: {e}"))?;

    let svg_dir = Path::new(build_constants::RESOURCES_DIR).join("svg").join(&style);

    // GResource SVGs: copy/rename pre-rendered SVGs from
    // resources/svg/{style}/ to `heroicons-{name}-symbolic.svg` under
    // scalable/glyphs. Runs inside FontBuild's export closure so
    // metadata.json and icons.gresource.xml land in OUT_DIR.
    let config = ExportConfig::<HeroiconsConfig>::new();
    FontBuild::svg_only()
        .extra_hash(style.clone())
        .extra_hash(EXPORT_PIPELINE_VERSION.to_string())
        .extra_hash(format!("{:016x}", hash_svg_dir(&svg_dir)))
        .extra_hash(format!("{:016x}", hash_str(&keywords_json)))
        .extra_hash(format!("{:016x}", hash_str(&aliases_json)))
        .rerun_if_changed(&svg_dir)
        .run_svg(|resources_dir| {
            SvgDirSource::new(&svg_dir)
                .codepoint_strategy(CodepointStrategy::HashedPua)
                .name_resolver(|parsed| Some(format!("{}-symbolic", parsed.raw_name)))
                .name_prefix(HeroiconsConfig::FONT_FAMILY_NAME)
                .post_process(SymbolicClasses)
                .export(&config, resources_dir)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    // Generate metadata (keywords + aliases) if enabled.
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating icon metadata (keywords + aliases)...");

        let keywords_value: serde_json::Value = serde_json::from_str(&keywords_json).map_err(|e| miette::miette!("Failed to parse keywords.json: {e}"))?;
        let keywords = keywords_value.as_object().cloned().unwrap_or_default();
        let aliases = parse_aliases(&aliases_json)?;

        let metadata_json = std::fs::read_to_string(build_constants::metadata_path()).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;

        HeroiconsMetadataGenerator { keywords, aliases }
            .run(&entries)
            .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
