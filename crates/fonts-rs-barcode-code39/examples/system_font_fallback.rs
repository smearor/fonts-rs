//! Minimal example: system font fallback when the barcode font is NOT loaded.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_barcode_code39::Code39;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_code39_fallback";

const BARCODE_TEXT: &str = "*HELLO*";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<Code39>(
        APP_ID,
        "Code39 — System Font Fallback",
        BARCODE_TEXT,
        "Font NOT loaded — system font fallback is used. The text is not a valid barcode.",
    )
}
