//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_lucide::Lucide;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.lucide_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<_> = ["lucide-house-symbolic", "lucide-heart-symbolic", "lucide-star-symbolic", "lucide-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Lucide::codepoint(name).map(|cp| (cp, name.to_string())))
        .collect();

    run_svg_gresource::<Lucide>(APP_ID, "Lucide Icons - SVG GResource", &glyphs)
}
