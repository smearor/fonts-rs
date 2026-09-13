// build.rs: export Bravura glyphs and generate codepoint maps + constants.
//
// Bravura is a SMuFL-compliant OpenType music font with ~2600+ glyphs
// covering conventional music notation symbols. All glyphs are mapped
// to the Unicode Private Use Area (U+E000–U+F8FF) per the SMuFL spec.
//
// The Bravura OTF uses a post table version 3.0 (no PostScript glyph
// names), so glyph names come from the SMuFL specification's
// glyphnames.json metadata file instead of the font itself.

use std::path::Path;

use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::GlyphNameMap;

#[path = "src/definition.rs"]
mod definition;

use definition::BravuraConfig;

/// OTF font file name (relative to `resources/`).
pub const FONT_FILE: &str = "Bravura.otf";

/// SMuFL glyphnames.json file name (relative to `resources/`).
pub const GLYPHNAMES_FILE: &str = "glyphnames.json";

/// Parse SMuFL glyphnames.json into a [`GlyphNameMap`].
///
/// The JSON format is: `{ "glyphName": { "codepoint": "U+E050", "description": "..." } }`.
/// Deserializes directly via serde, extracting only the `codepoint` field.
fn parse_glyphnames(json: &str) -> miette::Result<GlyphNameMap> {
    serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse glyphnames.json: {e}"))
}

fn main() -> miette::Result<()> {
    let font_path = Path::new(build_constants::RESOURCES_DIR).join(FONT_FILE);
    let glyphnames_path = Path::new(build_constants::RESOURCES_DIR).join(GLYPHNAMES_FILE);

    eprintln!("build.rs: exporting glyphs from {}", font_path.display());

    set_font_path_env("BRAVURA_FONT_PATH", &font_path)?;

    // Parse SMuFL glyph names
    let glyphnames_json = std::fs::read_to_string(&glyphnames_path).map_err(|e| miette::miette!("Failed to read {}: {e}", glyphnames_path.display()))?;
    let name_map = parse_glyphnames(&glyphnames_json)?;
    eprintln!("build.rs: parsed {} SMuFL glyph names", name_map.len());

    let config = ExportConfig::<BravuraConfig>::new();

    FontBuild::new(&font_path)
        .run(|font_path, resources_dir| config.export_glyphs_by_name_map(font_path, resources_dir, &name_map))
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;

    Ok(())
}
