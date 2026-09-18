//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_redacted::RedactedConfig;
use fonts_rs_redacted::all_glyphs;
use fonts_rs_redacted::naming::Redacted;
use fonts_rs_redacted::variant;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<Redacted, RedactedConfig>("Redacted — SVG Glyphs", variant::GRESOURCE_PREFIX, &glyphs, "redacted_svg_html.html")
}
