//! Minimal GTK example: use GtkIconTheme to look up Noto Emoji glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_noto_emoji_color_flags::NotoEmoji;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    // Icon names are variant-prefixed (`noto-emoji-{variant}-*`), so they
    // are resolved from codepoints instead of using the generated
    // constants - this keeps the example compiling under every variant.
    let icons: Vec<(String, &str)> = [
        ('\u{1F600}', "Grinning Face"),
        ('\u{1F60A}', "Smiling Face"),
        ('\u{1F44D}', "Thumbs Up"),
        ('\u{1F525}', "Fire"),
    ]
    .into_iter()
    .filter_map(|(ch, label)| NotoEmoji::glyph_name(ch).map(|name| (name.as_ref().to_string(), label)))
    .collect();

    run_gtk_icon_theme::<NotoEmoji>("io.smearor.fonts_rs.noto_emoji_icon_theme", "Noto Emoji - GtkIconTheme", &icons)
}
