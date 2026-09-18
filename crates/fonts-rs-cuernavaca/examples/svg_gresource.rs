//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_cuernavaca::CuernavacaConfig;
use fonts_rs_cuernavaca::all_glyphs;
use fonts_rs_cuernavaca::naming::Cuernavaca;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.cuernavaca_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<(char, &str)> = all_glyphs().take(4).collect();

    run_svg_gresource::<Cuernavaca, CuernavacaConfig>(APP_ID, "Cuernavaca — SVG GResource", CuernavacaConfig::GRESOURCE_PREFIX, &glyphs)
}
