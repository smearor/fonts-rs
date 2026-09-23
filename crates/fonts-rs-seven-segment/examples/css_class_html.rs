//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_seven_segment::SevenSegment;
use fonts_rs_test::run_css_class_html;

const SAMPLE_TEXT: &str = "12345678";

fn main() -> miette::Result<()> {
    run_css_class_html::<SevenSegment>("7-segment display", SAMPLE_TEXT, "DSEG7 — CSS Class", "seven_segment_css_class.html")
}
