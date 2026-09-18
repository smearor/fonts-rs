//! SVG GResource example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_doto::DotoConfig;
use fonts_rs_doto::all_glyphs;
use fonts_rs_doto::naming::Doto;
use fonts_rs_doto::variant;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.doto_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<(char, &str)> = all_glyphs().take(4).collect();

    run_svg_gresource::<Doto, DotoConfig>(APP_ID, "Doto — SVG GResource", variant::GRESOURCE_PREFIX, &glyphs)
}
