//! Minimal example: compose multiple DSEG7 glyph SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_seven_segment::SevenSegmentConfig;
use fonts_rs_seven_segment::all_glyphs;
use fonts_rs_seven_segment::naming::SevenSegment;
use fonts_rs_seven_segment::variant;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.seven_segment_composed_svg";

const SAMPLE_TEXT: &str = "12345678";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let char_to_name: Vec<(char, &str)> = all_glyphs().collect();

    run_composed_svg_text::<SevenSegment, SevenSegmentConfig>(
        APP_ID,
        "DSEG7 — Composed SVG Text",
        variant::GRESOURCE_PREFIX,
        SAMPLE_TEXT,
        "7-segment display",
        &char_to_name,
        GLYPH_HEIGHT,
    )
}
