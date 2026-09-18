//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Demonstrates the SVG HTML rendering path:
//! 1. `register_glyphs()` — registers SVG icon GResource
//! 2. `gio::resources_lookup_data()` — loads SVG bytes for each glyph
//! 3. Generate an HTML file with inline `<svg>` elements (no external font needed)
//! 4. Open in a browser — SVGs render as scalable vector graphics
//!
//! This path uses pre-rendered SVG outlines embedded directly in HTML,
//! without requiring the TTF font or @font-face.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_barcode_ean13::Ean13;
use fonts_rs_barcode_ean13::Ean13Definition;
use fonts_rs_barcode_ean13::all_glyphs;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<Ean13, Ean13Definition>("Libre Barcode EAN13 — SVG Glyphs", Ean13Definition::GRESOURCE_PREFIX, &glyphs, "barcode_ean13_svg_html.html")
}
