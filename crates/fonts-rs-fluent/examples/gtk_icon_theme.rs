//! Minimal GTK example: use GtkIconTheme to look up Fluent icon SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_fluent::Fluent;
use fonts_rs_fluent::variant;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let prefix = variant::GLYPH_PREFIX;
    let icons: Vec<(String, &str)> = [
        ("home", "Home"),
        ("heart", "Heart"),
        ("star", "Star"),
        ("settings", "Settings"),
        ("alert", "Alert"),
        ("search", "Search"),
    ]
    .into_iter()
    .filter_map(|(stem, label)| {
        let name = format!("{prefix}-{stem}-symbolic");
        Fluent::codepoint(&name).map(|_| (name, label))
    })
    .collect();

    run_gtk_icon_theme::<Fluent>("io.smearor.fonts_rs.fluent_icon_theme", "Fluent UI System Icons - GtkIconTheme", &icons)
}
