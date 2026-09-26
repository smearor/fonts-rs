//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Lucide is fontless — the page renders its glyph grid from bundled
//! SVGs alone, no `@font-face` involved.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_lucide::Lucide;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    run_svg_html::<Lucide>("Lucide Icons - SVG Glyphs", "lucide_svg_html.html")
}
