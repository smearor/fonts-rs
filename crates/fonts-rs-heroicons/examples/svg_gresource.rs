//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_heroicons::Heroicons;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.heroicons_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<_> = [
        "heroicons-star-symbolic",
        "heroicons-heart-symbolic",
        "heroicons-bell-symbolic",
        "heroicons-sparkles-symbolic",
    ]
    .into_iter()
    .filter_map(|name| Heroicons::codepoint(name).map(|cp| (cp, name.to_string())))
    .collect();

    run_svg_gresource::<Heroicons>(APP_ID, "Heroicons - SVG GResource", &glyphs)
}
