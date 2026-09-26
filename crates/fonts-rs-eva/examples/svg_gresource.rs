//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_eva::Eva;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.eva_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    // Icon names are variant-independent, but the fill and outline
    // variants ship slightly different sets — resolve through
    // `codepoint` so the example works under either variant.
    let glyphs: Vec<_> = ["eva-home-symbolic", "eva-heart-symbolic", "eva-star-symbolic", "eva-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Eva::codepoint(name).map(|cp| (cp, name.to_string())))
        .collect();

    run_svg_gresource::<Eva>(APP_ID, "Eva Icons — SVG GResource", &glyphs)
}
