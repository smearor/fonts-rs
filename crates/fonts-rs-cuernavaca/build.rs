// build.rs: export Chess Cuernavaca glyphs and generate codepoint maps + constants.
//
// Cuernavaca is a chess piece font where uppercase letters represent
// white pieces and lowercase letters represent black pieces.
// The font covers ASCII letters and a few punctuation marks.

use fonts_rs_generator::FontBuild;
use fonts_rs_generator::build_config;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::ASCII_PRINTABLE_RANGE;

fn main() -> miette::Result<()> {
    let font_path = format!("{}/ChessCuernavaca.ttf", build_constants::RESOURCES_DIR);

    eprintln!("build.rs: exporting glyphs from {font_path}");

    set_font_path_env("CUERNAVACA_FONT_PATH", &font_path);

    let config = build_config("cuernavaca", "Cuernavaca", "cuernavaca", None, ASCII_PRINTABLE_RANGE);

    FontBuild::new(&font_path)
        .run(|font_path, resources_dir| config.export_glyphs(font_path, resources_dir))
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;

    Ok(())
}
