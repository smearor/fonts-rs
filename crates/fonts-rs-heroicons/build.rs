// build.rs: Heroicons build - fontless SVG-only icon set.
//
// All four upstream styles are exported in one build via
// `CombinedSource`, partitioned by a `-{style}` name infix:
//
// - `outline` -> `heroicons-{name}-symbolic` (24x24 stroke-based;
//   `SymbolicClasses` adds the GTK symbolic classes that keep stroke
//   details visible under symbolic recoloring; unsalted `HashedPua`
//   keeps the long-standing BMP PUA codepoint allocation)
// - `solid`   -> `heroicons-{name}-solid-symbolic` (24x24 filled)
// - `mini`    -> `heroicons-{name}-mini-symbolic` (20x20 filled)
// - `micro`   -> `heroicons-{name}-micro-symbolic` (16x16 filled)
//
// Codepoints are stable PUA allocations via `CodepointStrategy::HashedPua`
// (upstream ships no codepoint table and no font). The non-outline
// styles share a PUA-B allocator (U+100000–U+10FFFD) and salt the
// hash input with their style - per-name stable and fully disjoint
// from outline's BMP PUA allocation, so upstream additions never
// move style assignments.
//
// Metadata: `resources/metadata/keywords.json` carries the search
// keywords used by heroicons.com (extracted from the site's JS bundle),
// `resources/metadata/aliases.json` maps Heroicons v1 names to their
// v2 canonical names (from the iconify heroicons set's alias table).
// Style categories are derived from the glyph name.
//
// Change detection: with no font file, `extra_hash` carries the whole
// hash - all four SVG directories and both metadata JSONs.

use std::cell::RefCell;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use fonts_rs_generator::CodepointStrategy;
use fonts_rs_generator::CombinedSource;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::GlyphSource;
use fonts_rs_generator::MetadataGenerator;
use fonts_rs_generator::PuaAllocator;
use fonts_rs_generator::SvgDirSource;
use fonts_rs_generator::SymbolicClasses;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::hash_str;
use fonts_rs_generator::hash_svg_dir;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GlyphEntry;

#[path = "src/definition.rs"]
mod definition;

use definition::HeroiconsConfig;

/// Non-outline styles in directory order. Outline is the default
/// style: it keeps plain `heroicons-{name}-symbolic` names and the
/// unsalted codepoint allocation.
const STYLES: &[&str] = &["solid", "mini", "micro"];

/// PUA-B allocator shared across the three non-outline style
/// sources so hash collisions resolve across styles, not just within
/// one directory. Disjoint from outline's BMP PUA arena.
type SharedAllocator = Rc<RefCell<PuaAllocator>>;

/// `SvgDirSource` for the outline directory: plain
/// `heroicons-{name}-symbolic` names and the unsalted (stable)
/// `HashedPua` codepoint allocation in the BMP PUA.
fn outline_source(dir: &Path) -> SvgDirSource<'static> {
    SvgDirSource::new(dir)
        .codepoint_strategy(CodepointStrategy::HashedPua)
        .name_resolver(|parsed| Some(format!("{}-symbolic", parsed.raw_name)))
        .name_prefix(HeroiconsConfig::FONT_FAMILY_NAME)
        .post_process(SymbolicClasses)
}

/// `SvgDirSource` for a non-outline style directory:
/// `heroicons-{name}-{style}-symbolic` names and a style-salted
/// `HashedPua` input allocating into shared PUA-B, so the same icon
/// cannot collide with another style's codepoint.
fn style_source(dir: &Path, style: &'static str, allocator: &SharedAllocator) -> SvgDirSource<'static> {
    SvgDirSource::new(dir)
        .codepoint_strategy(CodepointStrategy::HashedPua)
        .pua_salt(style)
        .pua_allocator(allocator.clone())
        .name_resolver(move |parsed| Some(format!("{}-{style}-symbolic", parsed.raw_name)))
        .name_prefix(HeroiconsConfig::FONT_FAMILY_NAME)
        .post_process(SymbolicClasses)
}

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
    /// `heroicons-` prefix, `-symbolic` suffix and the `-{style}`
    /// infix of non-outline styles.
    fn upstream_key(glyph_name: &str) -> &str {
        let stem = glyph_name
            .strip_prefix("heroicons-")
            .and_then(|name| name.strip_suffix("-symbolic"))
            .unwrap_or(glyph_name);
        STYLES.iter().find_map(|style| stem.strip_suffix(&format!("-{style}"))).unwrap_or(stem)
    }

    /// Returns the style label for a glyph name ("outline" for the
    /// unsuffixed default style).
    fn style_label(glyph_name: &str) -> &'static str {
        let stem = Self::upstream_keyless_stem(glyph_name);
        STYLES.iter().find(|style| stem.ends_with(&format!("-{style}"))).map_or("outline", |style| match *style {
            "solid" => "solid",
            "mini" => "mini",
            _ => "micro",
        })
    }

    /// Stem without `heroicons-` prefix and `-symbolic` suffix.
    fn upstream_keyless_stem(glyph_name: &str) -> &str {
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

    /// Every glyph gets its style as category - upstream ships no
    /// categories, so the style is the only label.
    fn categories_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        vec![Self::style_label(&entry.name).to_string()]
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
    eprintln!("build.rs: combined outline+solid+mini+micro export");

    let metadata_dir = Path::new(build_constants::RESOURCES_DIR).join("metadata");
    let keywords_json = std::fs::read_to_string(metadata_dir.join("keywords.json")).map_err(|e| miette::miette!("Failed to read keywords.json: {e}"))?;
    let aliases_json = std::fs::read_to_string(metadata_dir.join("aliases.json")).map_err(|e| miette::miette!("Failed to read aliases.json: {e}"))?;

    let svg_base = Path::new(build_constants::RESOURCES_DIR).join("svg");
    let outline_dir = svg_base.join("outline");
    let style_dirs: Vec<PathBuf> = STYLES.iter().map(|style| svg_base.join(style)).collect();

    // GResource SVGs: copy/rename pre-rendered SVGs from all four
    // style directories into one export, partitioned by a `-{style}`
    // name infix (`heroicons-{name}-solid-symbolic`, …). Runs inside
    // FontBuild's export closure so metadata.json and
    // icons.gresource.xml land in OUT_DIR.
    let config = ExportConfig::<HeroiconsConfig>::new();
    let mut build = FontBuild::svg_only().extra_hash(EXPORT_PIPELINE_VERSION.to_string());
    for dir in std::iter::once(&outline_dir).chain(&style_dirs) {
        build = build.extra_hash(format!("{:016x}", hash_svg_dir(dir)));
    }
    build
        .extra_hash(format!("{:016x}", hash_str(&keywords_json)))
        .extra_hash(format!("{:016x}", hash_str(&aliases_json)))
        .rerun_if_changed(&svg_base)
        .run_svg(|resources_dir| {
            // `svg_for` routes by the longest matching marker
            // (`-{style}-symbolic` beats the "heroicons-" catch-all).
            // The style sources share one PUA-B allocator, disjoint
            // from outline's BMP PUA arena.
            let allocator: SharedAllocator = Rc::new(RefCell::new(PuaAllocator::pua_b()));
            let source = CombinedSource::new()
                .source("heroicons-", outline_source(&outline_dir))
                .source("-solid-symbolic", style_source(&style_dirs[0], "solid", &allocator))
                .source("-mini-symbolic", style_source(&style_dirs[1], "mini", &allocator))
                .source("-micro-symbolic", style_source(&style_dirs[2], "micro", &allocator));
            source.export(&config, resources_dir)
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
