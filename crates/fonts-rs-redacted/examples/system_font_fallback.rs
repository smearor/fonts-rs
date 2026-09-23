//! Minimal example: system font fallback when the placeholder font is NOT loaded.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_redacted::Redacted;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.redacted_fallback";

const SAMPLE_TEXT: &str = "HELLO";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<Redacted>(
        APP_ID,
        "Redacted — System Font Fallback",
        SAMPLE_TEXT,
        "Font NOT loaded — system font fallback is used. Placeholder bars will not render correctly.",
    )
}
