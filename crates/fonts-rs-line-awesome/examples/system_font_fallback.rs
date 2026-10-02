//! Minimal example: system font fallback when the Line Awesome TTFs are NOT loaded.
//!
//! Counterpart to `gtk_gresource`: with no `install_pango` call the
//! CSS `font-family: 'la-solid-900'` misses, so FA5 PUA codepoints
//! fall back to whatever system font claims them (e.g. Nerd Fonts) -
//! usually wrong artwork, since other PUA fonts collide on the same
//! codepoint range.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_line_awesome::GLYPH_PREFIX;
use fonts_rs_line_awesome::LineAwesome;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.line_awesome_fallback";

/// Sample text built from icon codepoints (PUA chars).
fn sample_text() -> String {
    let prefix = GLYPH_PREFIX;
    ["heart", "star", "bell", "camera-solid"]
        .into_iter()
        .filter_map(|stem| LineAwesome::codepoint(format!("{prefix}-{stem}-symbolic")).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<LineAwesome>(
        APP_ID,
        "Line Awesome - System Font Fallback",
        &sample_text(),
        "Font NOT loaded - system font fallback is used. Icon glyphs may render as other artwork or not at all.",
    )
}
