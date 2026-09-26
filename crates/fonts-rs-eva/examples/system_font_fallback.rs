//! Minimal example: system font fallback when the icon font is NOT loaded.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_eva::Eva;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.eva_fallback";

/// Builds the sample text from icon codepoints resolved at runtime -
/// codepoints differ between the fill and outline variants.
fn sample_text() -> String {
    ["eva-home-symbolic", "eva-heart-symbolic", "eva-star-symbolic", "eva-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Eva::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<Eva>(
        APP_ID,
        "Eva Icons - System Font Fallback",
        &sample_text(),
        "Font NOT loaded - system font fallback is used. Icon glyphs render as PUA placeholders or not at all.",
    )
}
