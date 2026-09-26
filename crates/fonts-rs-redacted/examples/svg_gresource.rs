//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_redacted::Redacted;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.redacted_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<_> = Redacted::all_glyphs().take(4).collect();

    run_svg_gresource::<Redacted>(APP_ID, "Redacted - SVG GResource", &glyphs)
}
