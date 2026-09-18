//! SVG HTML example for Bravura.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_bravura::BravuraConfig;
use fonts_rs_bravura::all_glyphs;
use fonts_rs_bravura::naming::Bravura;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<Bravura, BravuraConfig>("Bravura — SVG Glyphs", BravuraConfig::GRESOURCE_PREFIX, &glyphs, "bravura_svg_html.html")
}
