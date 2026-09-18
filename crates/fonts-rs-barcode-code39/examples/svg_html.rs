//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_barcode_code39::Code39;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    run_svg_html::<Code39>("Libre Barcode Code 39 — SVG Glyphs", "barcode_code39_svg_html.html")
}
