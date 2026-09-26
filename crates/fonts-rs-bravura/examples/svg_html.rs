//! SVG HTML example for Bravura.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_bravura::Bravura;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    run_svg_html::<Bravura>("Bravura - SVG Glyphs", "bravura_svg_html.html")
}
