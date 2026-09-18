//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_seven_segment::SevenSegmentConfig;
use fonts_rs_seven_segment::all_glyphs;
use fonts_rs_seven_segment::naming::SevenSegment;
use fonts_rs_seven_segment::variant;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<SevenSegment, SevenSegmentConfig>("DSEG7 — SVG Glyphs", variant::GRESOURCE_PREFIX, &glyphs, "seven_segment_svg_html.html")
}
