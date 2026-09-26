//! Minimal example: system font fallback when the Nerd Font is NOT loaded.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_test::run_system_font_fallback;
use nerd_fonts_rs::NerdFonts;

const APP_ID: &str = "io.smearor.fonts_rs.nerd_fonts_fallback";

/// Sample text:  (U+F005 star),  (U+F004 heart),  (U+F06D fire),  (U+F135 rocket)
const SAMPLE_TEXT: &str = "\u{F005}\u{F004}\u{F06D}\u{F135}";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<NerdFonts>(
        APP_ID,
        "Nerd Fonts - System Font Fallback",
        SAMPLE_TEXT,
        "Font NOT loaded - system font fallback is used. Nerd Font icons will not render correctly.",
    )
}
