//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_dicefont::DicefontConfig;
use fonts_rs_dicefont::all_glyphs;
use fonts_rs_dicefont::naming::Dicefont;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.dicefont_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<(char, &str)> = all_glyphs().take(4).collect();

    run_svg_gresource::<Dicefont, DicefontConfig>(APP_ID, "Dicefont — SVG GResource", DicefontConfig::GRESOURCE_PREFIX, &glyphs)
}
