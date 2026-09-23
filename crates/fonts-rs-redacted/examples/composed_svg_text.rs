//! Minimal example: compose multiple Redacted glyph SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_redacted::Redacted;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.redacted_composed_svg";

const SAMPLE_TEXT: &str = "HELLO";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<Redacted>(APP_ID, "Redacted — Composed SVG Text", SAMPLE_TEXT, "redacted text", GLYPH_HEIGHT)
}
