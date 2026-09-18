//! SVG HTML example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_doto::DotoConfig;
use fonts_rs_doto::all_glyphs;
use fonts_rs_doto::naming::Doto;
use fonts_rs_doto::variant;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<Doto, DotoConfig>("Doto — SVG Glyphs", variant::GRESOURCE_PREFIX, &glyphs, "doto_svg_html.html")
}
