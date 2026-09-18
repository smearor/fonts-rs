//! Minimal example: system font fallback when the emoji font is NOT loaded.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.noto_emoji_fallback";

const FONT_FAMILY: &str = "Noto Emoji";

/// Sample text: 🎉🚀😍🔥
const SAMPLE_TEXT: &str = "🎉🚀😍🔥";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback(
        APP_ID,
        "Noto Emoji — System Font Fallback",
        FONT_FAMILY,
        SAMPLE_TEXT,
        "Font NOT loaded — system font fallback is used. Emoji glyphs may render as monochrome or not at all.",
    )
}
