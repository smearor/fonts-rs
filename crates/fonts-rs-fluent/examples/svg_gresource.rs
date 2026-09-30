//! Minimal GTK example: load Fluent SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_fluent::Fluent;
use fonts_rs_fluent::variant;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.fluent_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let prefix = variant::GLYPH_PREFIX;
    let glyphs: Vec<_> = ["home", "heart", "star", "alert"]
        .into_iter()
        .filter_map(|stem| {
            let name = format!("{prefix}-{stem}-symbolic");
            Fluent::codepoint(&name).map(|cp| (cp, name))
        })
        .collect();

    run_svg_gresource::<Fluent>(APP_ID, "Fluent UI System Icons - SVG GResource", &glyphs)
}
