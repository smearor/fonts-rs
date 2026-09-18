//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_redacted::RedactedConfig;
use fonts_rs_redacted::all_glyphs;
use fonts_rs_redacted::naming::Redacted;
use fonts_rs_redacted::variant;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.redacted_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<(char, &str)> = all_glyphs().take(4).collect();

    run_svg_gresource::<Redacted, RedactedConfig>(APP_ID, "Redacted — SVG GResource", variant::GRESOURCE_PREFIX, &glyphs)
}
