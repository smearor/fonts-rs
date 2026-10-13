//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_blobmoji_bitmap_noflags::Blobmoji;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.blobmoji_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<_> = Blobmoji::all_glyphs().take(4).collect();

    run_svg_gresource::<Blobmoji>(APP_ID, "Blobmoji - SVG GResource", &glyphs)
}
