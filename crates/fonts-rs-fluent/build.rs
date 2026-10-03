// build.rs: Fluent UI System Icons hybrid build - pre-rendered SVGs + TTF.
//
// Both upstream styles are exported in one build via `CombinedSource`:
// `resources/svg/regular-24` -> `fluent-{name}-symbolic`,
// `resources/svg/filled-24` -> `fluent-filled-{name}-symbolic`.
// Glyph names are the upstream icon names
// (`ic_fluent_{name}_{size}_{style}.svg`), normalized to kebab-case.
//
// Regular codepoints are the real upstream values from the vendored
// `FluentSystemIcons-Regular.json` map (flat `{"ic_fluent_...": int}`
// map) via `CodepointStrategy::FromNameMap`. Upstream allocates both
// fonts into the *same* PUA space, so the filled style cannot reuse
// its map: filled codepoints are hashed (`CodepointStrategy::HashedPua`,
// salted `"fluent-filled"`) into Supplementary PUA-B (U+100000–
// U+10FFFD) - per-name stable and fully disjoint from the regular
// map, so upstream additions never move filled assignments.
// Filled glyphs therefore render via the icon theme / SVG path only;
// the single bundled `FluentSystemIcons-Regular.ttf` covers regular.
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
// `MetadataGenerator` phf maps; style categories are name-derived.
//
// Change detection: `extra_hash` carries the whole hash - export
// pipeline version, both SVG directories, the regular codepoint map
// JSON and the metadata JSONs.

use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;

use fonts_rs_generator::CodepointStrategy;
use fonts_rs_generator::CombinedSource;
use fonts_rs_generator::CurrentColor;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontFamilyConfigExt;
use fonts_rs_generator::GlyphSource;
use fonts_rs_generator::GlyphSourceError;
use fonts_rs_generator::MetadataGenerator;
use fonts_rs_generator::PaintTarget;
use fonts_rs_generator::PuaAllocator;
use fonts_rs_generator::SvgDirSource;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::hash_str;
use fonts_rs_generator::hash_svg_dir;
use fonts_rs_generator::normalize_to_kebab;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_generator::stem_keywords;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GlyphEntry;
use fonts_rs_model::GlyphNameMap;

#[path = "src/definition.rs"]
mod definition;

#[path = "src/filename_parser.rs"]
mod filename_parser;

use definition::FluentConfig;
use filename_parser::FluentFilenameParser;

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

/// `MetadataGenerator` for the Fluent set.
///
/// Keywords are filename-derived: the kebab stem of each exported
/// glyph name splits into word tokens (see [`stem_keywords`]). Aliases
/// come from the vendored `aliases.json` (iconify kebab->kebab rename
/// table). Categories carry the style ("regular" / "filled"); Fluent
/// ships no categories upstream, so the style is the only label.
struct FluentMetadataGenerator {
    /// `(alias, canonical)` pairs using full `fluent-*-symbolic` names.
    aliases: Vec<(String, String)>,
}

impl FluentMetadataGenerator {
    /// Returns the upstream kebab stem by stripping the
    /// `fluent-filled-` / `fluent-` prefix and `-symbolic` from an
    /// exported glyph name.
    fn upstream_key<'a>(&self, glyph_name: &'a str) -> &'a str {
        glyph_name
            .strip_prefix("fluent-filled-")
            .or_else(|| glyph_name.strip_prefix("fluent-"))
            .and_then(|name| name.strip_suffix("-symbolic"))
            .unwrap_or(glyph_name)
    }
}

impl MetadataGenerator for FluentMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        stem_keywords(self.upstream_key(&entry.name))
    }

    /// Every glyph gets its style as category - upstream ships no
    /// categories, so the style is the only label.
    fn categories_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        vec![if entry.name.starts_with("fluent-filled-") { "filled".to_string() } else { "regular".to_string() }]
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
    // The TTFs are vendored under resources/fonts/ (matching the
    // upstream lock file layout). Only the regular font ships: its
    // codepoints are the real upstream values, while filled
    // codepoints are hashed into PUA-B (upstream allocates both
    // styles into the same PUA space, so the filled map is unusable).
    let font_path = Path::new(build_constants::RESOURCES_DIR).join("fonts").join("FluentSystemIcons-Regular.ttf");

    eprintln!("build.rs: combined regular+filled export -> {}", font_path.display());

    // Set TTF path for pixel-drawing (ab_glyph) at runtime.
    set_font_path_env("FLUENT_FONT_PATH", &font_path)?;

    // Vendored upstream codepoint map: flat int map keyed by
    // `ic_fluent_{name}_{size}_regular`. Normalized to the glyph stems
    // produced by FluentFilenameParser (e.g. `i_o_t`) so
    // `CodepointStrategy::FromNameMap` can resolve every regular SVG.
    // The filled map is *not* loaded: upstream allocates both styles
    // into the same PUA space, so filled codepoints are hashed into
    // PUA-B instead (per-name stable, disjoint from regular).
    let metadata_dir = Path::new(build_constants::RESOURCES_DIR).join("metadata");
    let map_path = metadata_dir.join("FluentSystemIcons-Regular.json");
    let map_json = std::fs::read_to_string(&map_path).map_err(|e| miette::miette!("Failed to read {}: {e}", map_path.display()))?;
    let regular_codepoints = fluent_codepoint_map(&map_json, GRID_SIZE, "regular")?;
    eprintln!("build.rs: {} codepoints for regular-{GRID_SIZE}", regular_codepoints.len());

    // Vendored metadata JSONs. `hidden.json` (kebab names) drives the
    // SvgDirSource name_filter; `aliases.json` is parsed lazily when
    // the `metadata` feature is enabled.
    let aliases_json = std::fs::read_to_string(metadata_dir.join("aliases.json")).map_err(|e| miette::miette!("Failed to read aliases.json: {e}"))?;
    let hidden_json = std::fs::read_to_string(metadata_dir.join("hidden.json")).map_err(|e| miette::miette!("Failed to read hidden.json: {e}"))?;
    let hidden_names: Vec<String> = serde_json::from_str(&hidden_json).map_err(|e| miette::miette!("Failed to parse hidden.json: {e}"))?;
    let hidden_stems: HashSet<String> = hidden_names.iter().map(|name| name.to_lowercase().replace('-', "_")).collect();
    eprintln!("build.rs: {} hidden icon(s) excluded", hidden_stems.len());

    let svg_base = Path::new(build_constants::RESOURCES_DIR).join("svg");
    let regular_dir = svg_base.join(format!("regular-{GRID_SIZE}"));
    let filled_dir = svg_base.join(format!("filled-{GRID_SIZE}"));

    // `ExportConfig::new()` keeps the base prefixes: names are
    // `fluent-{name}-symbolic` (regular) and
    // `fluent-filled-{name}-symbolic` (filled), resources live under
    // the un-suffixed `fluent` GResource prefix.
    let config = ExportConfig::<FluentConfig>::new();

    FontBuild::new(&font_path)
        .compile_font_gresource()
        .gresource_prefix(FluentConfig::GRESOURCE_PREFIX)
        .extra_hash(EXPORT_PIPELINE_VERSION.to_string())
        .extra_hash(format!("{:016x}", hash_svg_dir(&regular_dir)))
        .extra_hash(format!("{:016x}", hash_svg_dir(&filled_dir)))
        .extra_hash(format!("{:016x}", hash_str(&map_json)))
        .extra_hash(format!("{:016x}", hash_str(&hidden_json)))
        .extra_hash(format!("{:016x}", hash_str(&aliases_json)))
        .rerun_if_changed(&svg_base)
        .rerun_if_changed(&metadata_dir)
        .run(|_font_path, resources_dir| {
            let base = |dir: &Path, style: &'static str, prefix: &'static str| {
                SvgDirSource::new(dir)
                    .filename_parser(FluentFilenameParser::new(GRID_SIZE, style))
                    .name_resolver(|parsed| normalize_to_kebab(&parsed.raw_name).map(|stem| format!("{stem}-symbolic")))
                    .name_filter(|parsed| !hidden_stems.contains(&parsed.raw_name))
                    .name_prefix(prefix)
                    .post_process(CurrentColor::new(PaintTarget::Fill))
            };
            // `svg_for` routes by the longest matching marker:
            // "fluent-filled-" wins over the "fluent-" catch-all.
            let count = CombinedSource::new()
                .source(
                    "fluent-",
                    base(&regular_dir, "regular", "fluent")
                        .codepoint_map(&regular_codepoints)
                        .codepoint_strategy(CodepointStrategy::FromNameMap),
                )
                .source(
                    "fluent-filled-",
                    base(&filled_dir, "filled", "fluent-filled")
                        .codepoint_strategy(CodepointStrategy::HashedPua)
                        .pua_salt("fluent-filled")
                        .pua_allocator(Rc::new(RefCell::new(PuaAllocator::pua_b()))),
                )
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
        // Alias pairs are generated for both style prefixes so a
        // `fluent-filled-*` name resolves just like its regular pair.
        let mut aliases = parse_aliases(&aliases_json, "fluent", &exported)?;
        aliases.extend(parse_aliases(&aliases_json, "fluent-filled", &exported)?);
        eprintln!("build.rs: {} aliases resolve to exported glyphs", aliases.len());

        FluentMetadataGenerator { aliases }
            .run(&entries)
            .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
