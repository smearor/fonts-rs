//! Minimal example: system font fallback when the icon font is NOT loaded.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_tabler::Tabler;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.tabler_fallback";

/// Builds the sample text from icon codepoints resolved at runtime -
/// codepoints differ between the outline and filled variants.
fn sample_text() -> String {
    ["tabler-home-symbolic", "tabler-heart-symbolic", "tabler-star-symbolic", "tabler-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Tabler::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<Tabler>(
        APP_ID,
        "Tabler Icons - System Font Fallback",
        &sample_text(),
        "Font NOT loaded - system font fallback is used. Icon glyphs render as PUA placeholders or not at all.",
    )
}
