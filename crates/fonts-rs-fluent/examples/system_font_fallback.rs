//! Minimal example: system font fallback when the Fluent TTF is NOT loaded.
//!
//! Counterpart to `gtk_gresource`: with no `install_pango` call the
//! CSS `font-family` misses, so PUA codepoints fall back to whatever
//! system font claims them (e.g. Nerd Fonts) - usually wrong artwork.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_fluent::Fluent;
use fonts_rs_fluent::variant;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.fluent_fallback";

/// Sample text built from icon codepoints (PUA chars).
fn sample_text() -> String {
    let prefix = variant::GLYPH_PREFIX;
    ["home", "heart", "star", "alert"]
        .into_iter()
        .filter_map(|stem| Fluent::codepoint(format!("{prefix}-{stem}-symbolic")).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<Fluent>(
        APP_ID,
        "Fluent UI System Icons - System Font Fallback",
        &sample_text(),
        "Font NOT loaded - system font fallback is used. Icon glyphs may render as other artwork or not at all.",
    )
}
