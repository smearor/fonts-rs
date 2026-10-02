//! Minimal example: generate an HTML page with inline Line Awesome SVG glyphs from GResource.
//!
//! The hybrid set ships both paths: this page renders its glyph grid
//! from the bundled SVGs alone, no `@font-face` involved (see
//! `css_class_html` for the TTF path).
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_line_awesome::LineAwesome;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    run_svg_html::<LineAwesome>("Line Awesome - SVG Glyphs", "line_awesome_svg_html.html")
}
