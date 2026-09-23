//! SVG HTML example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_doto::Doto;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    run_svg_html::<Doto>("Doto — SVG Glyphs", "doto_svg_html.html")
}
