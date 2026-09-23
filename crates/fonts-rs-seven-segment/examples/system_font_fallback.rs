//! Minimal example: system font fallback when the seven-segment font is NOT loaded.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_seven_segment::SevenSegment;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.seven_segment_fallback";

const SAMPLE_TEXT: &str = "12345678";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<SevenSegment>(
        APP_ID,
        "DSEG7 — System Font Fallback",
        SAMPLE_TEXT,
        "Font NOT loaded — system font fallback is used. Seven-segment display glyphs will not render correctly.",
    )
}
