//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_dicefont::DicefontConfig;
use fonts_rs_dicefont::all_glyphs;
use fonts_rs_dicefont::naming::Dicefont;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<Dicefont, DicefontConfig>("Dicefont — SVG Glyphs", DicefontConfig::GRESOURCE_PREFIX, &glyphs, "dicefont_svg_html.html")
}
