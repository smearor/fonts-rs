//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_barcode_code39::Code39;
use fonts_rs_barcode_code39::Code39Definition;
use fonts_rs_barcode_code39::all_glyphs;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_code39_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<(char, &str)> = all_glyphs().take(4).collect();

    run_svg_gresource::<Code39, Code39Definition>(APP_ID, "Code39 — SVG GResource", Code39Definition::GRESOURCE_PREFIX, &glyphs)
}
