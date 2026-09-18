//! SVG GResource example for Bravura.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_bravura::BravuraConfig;
use fonts_rs_bravura::all_glyphs;
use fonts_rs_bravura::naming::Bravura;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.bravura_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<(char, &str)> = all_glyphs().take(4).collect();

    run_svg_gresource::<Bravura, BravuraConfig>(APP_ID, "Bravura — SVG GResource", BravuraConfig::GRESOURCE_PREFIX, &glyphs)
}
