//! System font fallback example for Bravura.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_bravura::Bravura;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.bravura_fallback";

const SAMPLE_TEXT: &str = "\u{E050}\u{E0A4}\u{E083}";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<Bravura>(
        APP_ID,
        "Bravura - System Font Fallback",
        SAMPLE_TEXT,
        "Font NOT loaded - system font fallback is used. PUA codepoints may not render.",
    )
}
