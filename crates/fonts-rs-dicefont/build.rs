// build.rs: export Dicefont glyphs and generate codepoint maps + constants.
//
// Dicefont is a static icon font with die face glyphs for D2, D4, D6, D8,
// D10, D12, D20, and dot-d6 variants. All glyphs are in the Private Use
// Area (U+F100–U+F19B).

use std::fs;
use std::path::Path;

use fonts_rs_generator::FontBuild;
use fonts_rs_generator::build_config;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::CodePointRange;

/// PUA range (U+F000–U+FFFF) covering all dicefont glyphs.
const CODEPOINT_RANGES: &[CodePointRange] = &[CodePointRange::from_char('\u{F000}', '\u{FFFF}')];

/// TTF font file name (relative to `resources/`).
pub const FONT_FILE: &str = "dicefont.ttf";

fn main() -> miette::Result<()> {
    let font_path = format!("{}/{}", build_constants::RESOURCES_DIR, FONT_FILE);

    eprintln!("build.rs: exporting glyphs from {font_path}");

    set_font_path_env("DICEFONT_FONT_PATH", &font_path);

    let config = build_config("dicefont", "Dicefont", "dicefont", None, CODEPOINT_RANGES);

    let icons_dir = Path::new(build_constants::RESOURCES_DIR).join("scalable").join("glyphs");

    FontBuild::new(&font_path)
        .run(|font_path, resources_dir| {
            if icons_dir.exists() {
                fs::remove_dir_all(&icons_dir)?;
            }
            config.export_glyphs(font_path, resources_dir)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;

    Ok(())
}
