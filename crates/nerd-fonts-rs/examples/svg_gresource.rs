//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_test::run_svg_gresource;
use nerd_fonts_rs::NerdFonts;

const APP_ID: &str = "io.smearor.fonts_rs.nerd_fonts_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<_> = NerdFonts::all_glyphs().take(4).collect();

    run_svg_gresource::<NerdFonts>(APP_ID, "Nerd Fonts - SVG GResource", &glyphs)
}
