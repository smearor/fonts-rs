//! Minimal example: system font fallback when the dice font is NOT loaded.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_dicefont::Dicefont;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.dicefont_fallback";

/// Sample text: d6-1, d6-2, d6-3 (PUA codepoints U+F131–U+F133).
const SAMPLE_TEXT: &str = "\u{F131}\u{F132}\u{F133}";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<Dicefont>(
        APP_ID,
        "Dicefont — System Font Fallback",
        SAMPLE_TEXT,
        "Font NOT loaded — system font fallback is used. The dice face glyphs will not render correctly.",
    )
}
