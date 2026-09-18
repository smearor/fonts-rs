//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_fourteen_segment::FourteenSegmentConfig;
use fonts_rs_fourteen_segment::all_glyphs;
use fonts_rs_fourteen_segment::naming::FourteenSegment;
use fonts_rs_fourteen_segment::variant;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<FourteenSegment, FourteenSegmentConfig>("DSEG14 — SVG Glyphs", variant::GRESOURCE_PREFIX, &glyphs, "fourteen_segment_svg_html.html")
}
