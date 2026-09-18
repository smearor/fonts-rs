//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_gresource;
use nerd_fonts_rs::NerdFonts;
use nerd_fonts_rs::NerdFontsConfig;
use nerd_fonts_rs::all_icons;

const APP_ID: &str = "io.smearor.fonts_rs.nerd_fonts_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<(char, &str)> = all_icons().take(4).collect();

    run_svg_gresource::<NerdFonts, NerdFontsConfig>(APP_ID, "Nerd Fonts — SVG GResource", NerdFontsConfig::GRESOURCE_PREFIX, &glyphs)
}
