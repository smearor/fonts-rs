//! Minimal example: generate an HTML page with inline Fluent SVG glyphs from GResource.
//!
//! The hybrid set ships both paths: this page renders its glyph grid
//! from the bundled SVGs alone, no `@font-face` involved (see
//! `font_face_html` for the TTF path).
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_fluent::Fluent;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    run_svg_html::<Fluent>("Fluent UI System Icons - SVG Glyphs", "fluent_svg_html.html")
}
