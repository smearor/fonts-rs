//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_tabler::Tabler;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.tabler_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    // Icon names are variant-independent, but the filled variant ships a
    // subset - resolve through `codepoint` so the example works under
    // either variant.
    let glyphs: Vec<_> = ["tabler-home-symbolic", "tabler-heart-symbolic", "tabler-star-symbolic", "tabler-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Tabler::codepoint(name).map(|cp| (cp, name.to_string())))
        .collect();

    run_svg_gresource::<Tabler>(APP_ID, "Tabler Icons - SVG GResource", &glyphs)
}
