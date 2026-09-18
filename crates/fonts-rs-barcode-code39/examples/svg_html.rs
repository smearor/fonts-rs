//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_barcode_code39::Code39;
use fonts_rs_barcode_code39::Code39Definition;
use fonts_rs_barcode_code39::all_glyphs;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<Code39, Code39Definition>(
        "Libre Barcode Code 39 — SVG Glyphs",
        Code39Definition::GRESOURCE_PREFIX,
        &glyphs,
        "barcode_code39_svg_html.html",
    )
}
