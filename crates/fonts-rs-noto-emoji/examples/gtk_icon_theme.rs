//! Minimal GTK example: use GtkIconTheme to look up Noto Emoji glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_noto_emoji::NotoEmoji;
use fonts_rs_noto_emoji::constants;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [
        (constants::NOTO_EMOJI_GRINNING_FACE, "Grinning Face"),
        (constants::NOTO_EMOJI_SMILING_FACE, "Smiling Face"),
        (constants::NOTO_EMOJI_THUMBS_UP, "Thumbs Up"),
        (constants::NOTO_EMOJI_FIRE, "Fire"),
    ];

    run_gtk_icon_theme::<NotoEmoji>("io.smearor.fonts_rs.noto_emoji_icon_theme", "Noto Emoji — GtkIconTheme", &icons)
}
