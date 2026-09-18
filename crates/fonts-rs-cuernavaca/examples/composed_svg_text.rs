//! Minimal example: compose multiple Cuernavaca glyph SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_cuernavaca::CuernavacaConfig;
use fonts_rs_cuernavaca::all_glyphs;
use fonts_rs_cuernavaca::naming::Cuernavaca;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.cuernavaca_composed_svg";

/// Sample text: King, Queen, Rook, Bishop, Knight, Pawn.
const SAMPLE_TEXT: &str = "KQRBNP";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let char_to_name: Vec<(char, &str)> = all_glyphs().collect();

    run_composed_svg_text::<Cuernavaca, CuernavacaConfig>(
        APP_ID,
        "Cuernavaca — Composed SVG Text",
        CuernavacaConfig::GRESOURCE_PREFIX,
        SAMPLE_TEXT,
        "chess notation",
        &char_to_name,
        GLYPH_HEIGHT,
    )
}
