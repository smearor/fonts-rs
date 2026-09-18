//! Minimal example: system font fallback when the barcode font is NOT loaded.
//!
//! Demonstrates what happens when `load_font()` is not called:
//! - CSS references `font-family: 'LibreBarcodeEAN13Text'`
//! - Pango cannot find the font family
//! - GTK falls back to the system font
//! - The barcode text is displayed but NOT rendered as a valid barcode
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_ean13_fallback";

const FONT_FAMILY: &str = "LibreBarcodeEAN13Text";

const BARCODE_TEXT: &str = ":ABCDEFGHIJ*abcdefghij:";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback(
        APP_ID,
        "EAN13 — System Font Fallback",
        FONT_FAMILY,
        BARCODE_TEXT,
        "Font NOT loaded — system font fallback is used. The text is not a valid barcode.",
    )
}
