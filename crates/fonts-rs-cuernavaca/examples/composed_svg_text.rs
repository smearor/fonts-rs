//! Minimal example: compose multiple Cuernavaca glyph SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_cuernavaca::Cuernavaca;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.cuernavaca_composed_svg";

/// Sample text: King, Queen, Rook, Bishop, Knight, Pawn.
const SAMPLE_TEXT: &str = "KQRBNP";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<Cuernavaca>(APP_ID, "Cuernavaca — Composed SVG Text", SAMPLE_TEXT, "chess notation", GLYPH_HEIGHT)
}
