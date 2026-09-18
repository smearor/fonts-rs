//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_cuernavaca::CuernavacaConfig;
use fonts_rs_cuernavaca::all_glyphs;
use fonts_rs_cuernavaca::naming::Cuernavaca;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<Cuernavaca, CuernavacaConfig>("Cuernavaca — SVG Glyphs", CuernavacaConfig::GRESOURCE_PREFIX, &glyphs, "cuernavaca_svg_html.html")
}
