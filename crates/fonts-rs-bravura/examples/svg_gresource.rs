//! SVG GResource example for Bravura.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_bravura::Bravura;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.bravura_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<_> = Bravura::all_glyphs().take(4).collect();

    run_svg_gresource::<Bravura>(APP_ID, "Bravura - SVG GResource", &glyphs)
}
