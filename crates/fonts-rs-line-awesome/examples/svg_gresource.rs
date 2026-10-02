//! Minimal GTK example: load Line Awesome SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_line_awesome::GLYPH_PREFIX;
use fonts_rs_line_awesome::LineAwesome;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.line_awesome_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let prefix = GLYPH_PREFIX;
    let glyphs: Vec<_> = ["heart", "star", "bell", "camera-solid"]
        .into_iter()
        .filter_map(|stem| {
            let name = format!("{prefix}-{stem}-symbolic");
            LineAwesome::codepoint(&name).map(|cp| (cp, name))
        })
        .collect();

    run_svg_gresource::<LineAwesome>(APP_ID, "Line Awesome - SVG GResource", &glyphs)
}
