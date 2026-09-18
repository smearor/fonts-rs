//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_html;
use nerd_fonts_rs::NerdFonts;
use nerd_fonts_rs::NerdFontsConfig;
use nerd_fonts_rs::all_icons;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_icons().collect();

    run_svg_html::<NerdFonts, NerdFontsConfig>(
        "Nerd Fonts — SVG Glyphs",
        NerdFontsConfig::GRESOURCE_PREFIX,
        &glyphs,
        "nerd_fonts_svg_html.html",
    )
}
