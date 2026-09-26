//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_barcode_code128::Code128;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_code128_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<_> = Code128::all_glyphs().take(4).collect();

    run_svg_gresource::<Code128>(APP_ID, "Code128 - SVG GResource", &glyphs)
}
