//! Minimal example: system font fallback when the fourteen-segment font is NOT loaded.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_fourteen_segment::FourteenSegment;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.fourteen_segment_fallback";

const SAMPLE_TEXT: &str = "ABCDEF12";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<FourteenSegment>(
        APP_ID,
        "DSEG14 - System Font Fallback",
        SAMPLE_TEXT,
        "Font NOT loaded - system font fallback is used. Fourteen-segment display glyphs will not render correctly.",
    )
}
