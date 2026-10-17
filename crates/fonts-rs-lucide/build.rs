//! Build script for fonts-rs-lucide.
//!
//! Lucide is a **fontless** icon set: upstream ships only pre-rendered
//! SVGs (`stroke="currentColor"`, `fill="none"`, 24×24 grid) - no
//! webfont. This crate therefore uses `FontBuild::svg_only()`:
//! GResource SVGs are the only artifact, codepoints are stable PUA
//! allocations via `CodepointStrategy::HashedPua`, and there is no
//! `pango`/`render`/`embed-fonts` capability.
//!
//! Metadata comes from upstream per-icon JSON files (`icons/{name}.json`
//! with `tags` + `categories`), vendored merged as
//! `resources/metadata/icons.json`; `resources/metadata/categories.json`
//! maps category slugs to display titles. Lucide ships no aliases.
//!
//! Change detection: with no font file, `extra_hash` carries the whole
//! hash - SVG directory and both metadata JSONs.

use std::path::Path;

use fonts_rs_generator::CodepointStrategy;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::GlyphSource;
use fonts_rs_generator::MetadataGenerator;
use fonts_rs_generator::SvgDirSource;
use fonts_rs_generator::SymbolicClasses;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::hash_str;
use fonts_rs_generator::hash_svg_dir;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GlyphEntry;

#[path = "src/definition.rs"]
mod definition;

use definition::LucideConfig;

/// Bump when the glyph export pipeline (naming, layout, post-processing)
/// changes in ways that require re-export of unchanged inputs.
const EXPORT_PIPELINE_VERSION: u32 = 2;

/// `MetadataGenerator` backed by merged upstream `icons.json`
/// (`icons/{name}.json`: `tags` + `categories` per icon).
///
/// Maps each exported glyph's `lucide-{name}-symbolic` name back to its
/// upstream icon entry. Lucide ships no aliases, so only keywords and
/// categories are populated.
struct LucideMetadataGenerator {
    /// Parsed `icons.json` object keyed by upstream icon name.
    icons: serde_json::Map<String, serde_json::Value>,
}

impl LucideMetadataGenerator {
    /// Looks up the upstream entry for an exported glyph name
    /// (`lucide-{name}-symbolic` → `{name}`).
    fn entry<'a>(&'a self, name: &'a str) -> Option<&'a serde_json::Value> {
        let upstream = name.strip_prefix("lucide-").and_then(|n| n.strip_suffix("-symbolic")).unwrap_or(name);
        self.icons.get(upstream)
    }

    /// Extracts a string-list field (`tags` / `categories`) from an
    /// upstream icon entry.
    fn string_list(&self, name: &str, field: &str) -> Vec<String> {
        self.entry(name)
            .and_then(|entry| entry.get(field))
            .and_then(serde_json::Value::as_array)
            .map(|items| items.iter().filter_map(|item| item.as_str().map(String::from)).collect())
            .unwrap_or_default()
    }
}

impl MetadataGenerator for LucideMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        self.string_list(&entry.name, "tags")
    }

    fn categories_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        self.string_list(&entry.name, "categories")
    }
}

fn main() -> miette::Result<()> {
    // Emit the vendored SVG directory to dependent build scripts
    // (custom icon sets via `fonts_rs_generator::IconSetBuild`).
    if let Some(manifest_dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        println!("cargo:svg_dir={}", Path::new(&manifest_dir).join("resources/svg").display());
    }

    let metadata_dir = Path::new(build_constants::RESOURCES_DIR).join("metadata");
    let icons_json = std::fs::read_to_string(metadata_dir.join("icons.json")).map_err(|e| miette::miette!("Failed to read icons.json: {e}"))?;
    let categories_json = std::fs::read_to_string(metadata_dir.join("categories.json")).map_err(|e| miette::miette!("Failed to read categories.json: {e}"))?;

    let svg_dir = Path::new(build_constants::RESOURCES_DIR).join("svg");

    // GResource SVGs: copy pre-rendered upstream SVGs to
    // `lucide-{name}-symbolic.svg` names under scalable/actions. Runs
    // inside FontBuild's export closure so metadata.json and
    // icons.gresource.xml land in OUT_DIR.
    let config = ExportConfig::<LucideConfig>::new();
    FontBuild::svg_only()
        .extra_hash(EXPORT_PIPELINE_VERSION.to_string())
        .extra_hash(format!("{:016x}", hash_svg_dir(&svg_dir)))
        .extra_hash(format!("{:016x}", hash_str(&icons_json)))
        .extra_hash(format!("{:016x}", hash_str(&categories_json)))
        .rerun_if_changed(&svg_dir)
        .run_svg(|resources_dir| {
            SvgDirSource::new(&svg_dir)
                .codepoint_strategy(CodepointStrategy::HashedPua)
                .name_resolver(|parsed| Some(format!("{}-symbolic", parsed.raw_name)))
                .name_prefix(LucideConfig::FONT_FAMILY_NAME)
                .post_process(SymbolicClasses)
                .export(&config, resources_dir)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    // Generate metadata (keywords + categories) if enabled.
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating icon metadata (keywords + categories)...");

        let icons_value: serde_json::Value = serde_json::from_str(&icons_json).map_err(|e| miette::miette!("Failed to parse icons.json: {e}"))?;
        let icons = icons_value.as_object().cloned().unwrap_or_default();

        let metadata_json = std::fs::read_to_string(build_constants::metadata_path()).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;

        LucideMetadataGenerator { icons }
            .run(&entries)
            .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
