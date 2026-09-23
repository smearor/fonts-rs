//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Demonstrates the SVG GResource rendering path:
//! 1. `register_glyphs()` — registers SVG icon GResource
//! 2. `gio::resources_lookup_data()` — loads SVG bytes for a specific glyph
//! 3. `gdk::Texture::from_bytes()` — rasterizes SVG to a GPU texture
//! 4. `Picture` widget — displays the texture in GTK
//!
//! This path uses pre-rendered SVG outlines (not the TTF font) to display
//! individual glyphs as scalable vector graphics.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_barcode_ean13::Ean13;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_ean13_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<_> = Ean13::all_glyphs().take(4).collect();

    run_svg_gresource::<Ean13>(APP_ID, "EAN13 — SVG GResource", &glyphs)
}
