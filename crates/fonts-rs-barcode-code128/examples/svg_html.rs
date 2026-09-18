//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_barcode_code128::Code128;
use fonts_rs_barcode_code128::Code128Definition;
use fonts_rs_barcode_code128::all_glyphs;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<Code128, Code128Definition>(
        "Libre Barcode Code 128 — SVG Glyphs",
        Code128Definition::GRESOURCE_PREFIX,
        &glyphs,
        "barcode_code128_svg_html.html",
    )
}
