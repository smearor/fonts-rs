//! Composed SVG text example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_doto::Doto;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.doto_composed_svg";

const SAMPLE_TEXT: &str = "HELLO";

const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<Doto>(APP_ID, "Doto — Composed SVG Text", SAMPLE_TEXT, "dot matrix text", GLYPH_HEIGHT)
}
