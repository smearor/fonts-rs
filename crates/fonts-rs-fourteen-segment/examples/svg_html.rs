//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_fourteen_segment::FourteenSegment;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    run_svg_html::<FourteenSegment>("DSEG14 — SVG Glyphs", "fourteen_segment_svg_html.html")
}
