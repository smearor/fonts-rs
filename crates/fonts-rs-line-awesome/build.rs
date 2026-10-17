// build.rs: Line Awesome hybrid build - pre-rendered SVGs + TTFs.
//
// GResource SVGs come from the vendored flat `resources/svg/`
// directory; the three upstream `la-{style}-{weight}.ttf` webfonts are
// bundled via the static `resources/font.gresource.xml` manifest for
// pango/html/render.
//
// Glyph names are the upstream file stems
// (`{name}.svg` -> `la-{name}-symbolic`,
// `{name}-solid.svg` -> `la-{name}-solid-symbolic`), normalized to
// kebab-case. Codepoints come from the vendored stem-keyed
// `codepoints.json` (generated from upstream `_variables.scss` at
// vendoring time) via `CodepointStrategy::FromNameMap`. All codepoints
// are Font Awesome 5 BMP PUA allocations (U+F000-U+F8FF); regular and
// solid style variants share a codepoint per icon name.
//
// `CurrentColor::inject_missing()` injects `fill="currentColor"` on
// the `<svg>` root: upstream paths carry no paint attribute at all, so
// rewriting alone would be a no-op. A post-export assertion fails the
// build if any emitted SVG lacks the injected fill.
//
// Change detection: `extra_hash` carries the whole hash - export
// pipeline version, SVG directory, codepoint map and aliases JSON.

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
use fonts_rs_generator::build_constants;
use fonts_rs_generator::hash_str;
use fonts_rs_generator::hash_svg_dir;
use fonts_rs_generator::normalize_to_kebab;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_generator::stem_keywords;
use fonts_rs_model::GlyphEntry;
use fonts_rs_model::GlyphNameMap;

#[path = "src/definition.rs"]
mod definition;

use definition::LineAwesomeConfig;

/// Export pipeline version - bump when export semantics change.
const EXPORT_PIPELINE_VERSION: u64 = 1;

/// Post-export sanity check: every emitted SVG must carry
/// `fill="currentColor"` (root injection) and no hardcoded hex fill
/// may survive.
fn assert_fill_current_color(resources_dir: &Path) -> Result<(), GlyphSourceError> {
    let icons_dir = LineAwesomeConfig::icons_dir(resources_dir);
    for entry in std::fs::read_dir(&icons_dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "svg") {
            let svg = std::fs::read_to_string(&path)?;
            if !svg.contains("fill=\"currentColor\"") {
                return Err(GlyphSourceError::PostProcess(format!(
                    "{}: missing fill=\"currentColor\" - inject_missing failed",
                    path.display()
                )));
            }
            if svg.contains("fill=\"#") {
                return Err(GlyphSourceError::PostProcess(format!(
                    "{}: hardcoded hex fill leaked through the post-processor",
                    path.display()
                )));
            }
        }
    }
    Ok(())
}

/// `MetadataGenerator` for the Line Awesome set.
///
/// Keywords are filename-derived: the kebab stem of each exported glyph
/// name splits into word tokens via the shared
/// [`stem_keywords`](fonts_rs_generator::stem_keywords) helper.
/// Categories carry the style: the `-solid` filename suffix marks solid
/// glyphs, a plain stem with an exported `{stem}-solid` sibling marks
/// regular, a plain stem without sibling marks brands (FA5 model).
/// Aliases come from the vendored `aliases.json` (FA4-compat `*-o` and
/// `fa-*` names).
struct LineAwesomeMetadataGenerator {
    /// Stems of all exported glyphs (`address-book`,
    /// `address-book-solid`, `500px`, ...) - used both for the style
    /// category pairing rule and for filtering aliases to exported
    /// canonical names.
    stems: HashSet<String>,
    /// `(alias, canonical)` pairs using full `la-*-symbolic` names.
    aliases: Vec<(String, String)>,
}

impl LineAwesomeMetadataGenerator {
    /// Returns the upstream kebab stem by stripping `la-` and
    /// `-symbolic` from an exported glyph name.
    fn upstream_key<'a>(&self, glyph_name: &'a str) -> &'a str {
        glyph_name
            .strip_prefix("la-")
            .and_then(|name| name.strip_suffix("-symbolic"))
            .unwrap_or(glyph_name)
    }
}

impl MetadataGenerator for LineAwesomeMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        stem_keywords(self.upstream_key(&entry.name))
    }

    fn categories_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        let stem = self.upstream_key(&entry.name);
        let style = if stem.ends_with("-solid") {
            "solid"
        } else if self.stems.contains(&format!("{stem}-solid")) {
            "regular"
        } else {
            "brands"
        };
        vec![style.to_string()]
    }

    fn aliases(&self) -> Vec<(String, String)> {
        self.aliases.clone()
    }
}

/// Parses `aliases.json` (FA4-compat `*-o` and `fa-*` kebab stems ->
/// kebab stem) into `(alias, canonical)` pairs with full
/// `la-*-symbolic` glyph names.
///
/// Entries whose canonical stem was not exported are dropped.
fn parse_aliases(json: &str, stems: &HashSet<String>) -> miette::Result<Vec<(String, String)>> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse aliases.json: {e}"))?;
    let aliases = value.as_object().ok_or_else(|| miette::miette!("aliases.json is not an object"))?;
    Ok(aliases
        .iter()
        .filter_map(|(alias, canonical)| canonical.as_str().map(|canonical| (alias, canonical)))
        .filter(|(alias, canonical)| alias.as_str() != *canonical)
        .filter(|(_, canonical)| stems.contains(*canonical))
        .map(|(alias, canonical)| (format!("la-{alias}-symbolic"), format!("la-{canonical}-symbolic")))
        .collect())
}

fn main() -> miette::Result<()> {
    // Emit the vendored SVG directory to dependent build scripts
    // (custom icon sets via `fonts_rs_generator::IconSetBuild`).
    if let Some(manifest_dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        println!("cargo:svg_dir={}", Path::new(&manifest_dir).join("resources/svg").display());
    }

    let resources = Path::new(build_constants::RESOURCES_DIR);
    let font_path = resources.join("fonts").join("la-solid-900.ttf");

    // Set the per-style TTF paths for pixel-drawing (ab_glyph) at
    // runtime. Solid is the default face (`LineAwesomeStyle::Solid`).
    set_font_path_env("LINE_AWESOME_REGULAR_FONT_PATH", &resources.join("fonts").join("la-regular-400.ttf"))?;
    set_font_path_env("LINE_AWESOME_SOLID_FONT_PATH", &font_path)?;
    set_font_path_env("LINE_AWESOME_BRANDS_FONT_PATH", &resources.join("fonts").join("la-brands-400.ttf"))?;

    // Vendored stem-keyed codepoint map: flat int map keyed by the
    // export's glyph stems so `CodepointStrategy::FromNameMap` can
    // resolve every SVG (both `adjust` and `adjust-solid` are keys).
    let metadata_dir = resources.join("metadata");
    let map_path = metadata_dir.join("codepoints.json");
    let map_json = std::fs::read_to_string(&map_path).map_err(|e| miette::miette!("Failed to read {}: {e}", map_path.display()))?;
    let codepoints: GlyphNameMap = serde_json::from_str(&map_json).map_err(|e| miette::miette!("Failed to parse codepoints.json: {e}"))?;
    eprintln!("build.rs: {} stem codepoints", codepoints.len());

    // Vendored alias table (FA4-compat + fa-* class names) - parsed
    // lazily when the `metadata` feature is enabled; hashed here so
    // alias changes trigger a re-export.
    let aliases_path = metadata_dir.join("aliases.json");
    let aliases_json = std::fs::read_to_string(&aliases_path).map_err(|e| miette::miette!("Failed to read {}: {e}", aliases_path.display()))?;

    let svg_dir = resources.join("svg");

    let config = ExportConfig::<LineAwesomeConfig>::new();

    FontBuild::new(&font_path)
        .compile_font_gresource()
        .extra_hash(format!("{EXPORT_PIPELINE_VERSION}"))
        .extra_hash(format!("{:016x}", hash_svg_dir(&svg_dir)))
        .extra_hash(format!("{:016x}", hash_str(&map_json)))
        .extra_hash(format!("{:016x}", hash_str(&aliases_json)))
        .rerun_if_changed(&svg_dir)
        .rerun_if_changed(&metadata_dir)
        .rerun_if_changed(resources.join("fonts"))
        .run(|_font_path, resources_dir| {
            let count = SvgDirSource::new(&svg_dir)
                .codepoint_map(&codepoints)
                .codepoint_strategy(CodepointStrategy::FromNameMap)
                .name_resolver(|parsed| normalize_to_kebab(&parsed.raw_name).map(|stem| format!("{stem}-symbolic")))
                .name_prefix(&config.glyph_name_prefix)
                .post_process(CurrentColor::new(PaintTarget::Fill).inject_missing())
                .export(&config, resources_dir)?;
            assert_fill_current_color(resources_dir)?;
            Ok::<usize, GlyphSourceError>(count)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    // Generate metadata (filename keywords, style categories, vendored
    // aliases) if enabled. `entries` come from the freshly written
    // metadata.json; the stem set covers every exported glyph name.
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating icon metadata (filename keywords + style categories + aliases)...");

        let metadata_json = std::fs::read_to_string(build_constants::metadata_path()).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;
        let stems: HashSet<String> = entries
            .iter()
            .filter_map(|entry| {
                entry
                    .name
                    .strip_prefix("la-")
                    .and_then(|name| name.strip_suffix("-symbolic"))
                    .map(str::to_string)
            })
            .collect();
        let aliases = parse_aliases(&aliases_json, &stems)?;
        eprintln!("build.rs: {} aliases resolve to exported glyphs", aliases.len());

        LineAwesomeMetadataGenerator { stems, aliases }
            .run(&entries)
            .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
