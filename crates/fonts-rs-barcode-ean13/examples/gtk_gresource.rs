//! Minimal GTK example: load font from GResource and render barcode text.
//!
//! Demonstrates the full GTK loading path:
//! 1. `register_glyphs()` - registers SVG icon GResource
//! 2. `register_font()` - registers TTF font GResource
//! 3. `load_font()` - extracts TTF to temp file, adds to Pango font map
//! 4. CSS `font-family` - GTK/Pango renders text with the loaded font
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_barcode_ean13::Ean13;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_ean13_gresource";

/// Hardcoded EAN13-encoded string (guard + left group + centre + right group + guard).
const BARCODE_TEXT: &str = ":ABCDEFGHIJ*abcdefghij:";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Ean13>(APP_ID, "EAN13 - GTK GResource", BARCODE_TEXT)
}
