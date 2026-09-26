//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_fourteen_segment::FourteenSegment;
use fonts_rs_test::run_css_class_html;

const SAMPLE_TEXT: &str = "ABCDEF12";

fn main() -> miette::Result<()> {
    run_css_class_html::<FourteenSegment>("14-segment display", SAMPLE_TEXT, "DSEG14 - CSS Class", "fourteen_segment_css_class.html")
}
