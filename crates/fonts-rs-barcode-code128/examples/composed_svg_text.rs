//! Minimal example: compose multiple Code 128 glyph SVGs into a single SVG barcode.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_barcode_code128::Code128;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_code128_composed_svg";

const BARCODE_TEXT: &str = "HELLO";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<Code128>(APP_ID, "Code128 — Composed SVG Text", BARCODE_TEXT, "barcode", GLYPH_HEIGHT)
}
