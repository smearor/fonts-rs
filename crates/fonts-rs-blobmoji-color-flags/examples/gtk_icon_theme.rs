//! Minimal GTK example: use GtkIconTheme to look up Blobmoji glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_blobmoji_color_flags::Blobmoji;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    // Icon names are variant-prefixed (`blobmoji-{variant}-*`), so they
    // are resolved from codepoints instead of using the generated
    // constants - this keeps the example compiling under every variant.
    let icons: Vec<(String, &str)> = [
        ('\u{1F600}', "Grinning Face"),
        ('\u{1F60A}', "Smiling Face"),
        ('\u{1F44D}', "Thumbs Up"),
        ('\u{1F525}', "Fire"),
    ]
    .into_iter()
    .filter_map(|(ch, label)| Blobmoji::glyph_name(ch).map(|name| (name.as_ref().to_string(), label)))
    .collect();

    run_gtk_icon_theme::<Blobmoji>("io.smearor.fonts_rs.blobmoji_icon_theme", "Blobmoji - GtkIconTheme", &icons)
}
