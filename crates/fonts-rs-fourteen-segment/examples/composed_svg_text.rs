//! Minimal example: compose multiple DSEG14 glyph SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_fourteen_segment::FourteenSegment;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.fourteen_segment_composed_svg";

const SAMPLE_TEXT: &str = "ABCDEF12";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<FourteenSegment>(APP_ID, "DSEG14 — Composed SVG Text", SAMPLE_TEXT, "14-segment display", GLYPH_HEIGHT)
}
