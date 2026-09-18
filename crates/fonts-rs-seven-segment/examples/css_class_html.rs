//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_seven_segment::naming::SevenSegment;
use fonts_rs_test::run_css_class_html;

const FONT_FAMILY: &str = "DSEG7 Classic";

const SAMPLE_TEXT: &str = "12345678";

fn main() -> miette::Result<()> {
    run_css_class_html::<SevenSegment>(FONT_FAMILY, "dseg7", "7-segment display", SAMPLE_TEXT, "DSEG7 — CSS Class", "seven_segment_css_class.html")
}
