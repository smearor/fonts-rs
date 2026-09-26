//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_eva::Eva;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    run_svg_html::<Eva>("Eva Icons - SVG Glyphs", "eva_svg_html.html")
}
