//! Minimal GTK example: use GtkIconTheme to look up Line Awesome icon SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_line_awesome::GLYPH_PREFIX;
use fonts_rs_line_awesome::LineAwesome;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let prefix = GLYPH_PREFIX;
    let icons: Vec<(String, &str)> = [
        ("heart", "Heart"),
        ("heart-solid", "Heart (solid)"),
        ("star", "Star"),
        ("bell", "Bell"),
        ("address-book", "Address Book"),
        ("500px", "500px (brand)"),
    ]
    .into_iter()
    .filter_map(|(stem, label)| {
        let name = format!("{prefix}-{stem}-symbolic");
        LineAwesome::codepoint(&name).map(|_| (name, label))
    })
    .collect();

    run_gtk_icon_theme::<LineAwesome>("io.smearor.fonts_rs.line_awesome_icon_theme", "Line Awesome - GtkIconTheme", &icons)
}
