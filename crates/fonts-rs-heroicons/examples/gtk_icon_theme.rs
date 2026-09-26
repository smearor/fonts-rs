//! Minimal GTK example: use GtkIconTheme to look up Heroicons icon SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_heroicons::Heroicons;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons: Vec<(String, &str)> = [
        ("heroicons-star-symbolic", "Star"),
        ("heroicons-heart-symbolic", "Heart"),
        ("heroicons-bell-symbolic", "Bell"),
        ("heroicons-bookmark-symbolic", "Bookmark"),
        ("heroicons-sparkles-symbolic", "Sparkles"),
        ("heroicons-magnifying-glass-symbolic", "Search"),
    ]
    .into_iter()
    .filter_map(|(name, label)| Heroicons::codepoint(name).map(|_| (name.to_string(), label)))
    .collect();

    run_gtk_icon_theme::<Heroicons>("io.smearor.fonts_rs.heroicons_icon_theme", "Heroicons - GtkIconTheme", &icons)
}
