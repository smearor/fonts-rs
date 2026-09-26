//! Minimal GTK example: use GtkIconTheme to look up Lucide icon SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_lucide::Lucide;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons: Vec<(String, &str)> = [
        ("lucide-house-symbolic", "House"),
        ("lucide-heart-symbolic", "Heart"),
        ("lucide-star-symbolic", "Star"),
        ("lucide-settings-symbolic", "Settings"),
        ("lucide-bell-symbolic", "Bell"),
        ("lucide-search-symbolic", "Search"),
    ]
    .into_iter()
    .filter_map(|(name, label)| Lucide::codepoint(name).map(|_| (name.to_string(), label)))
    .collect();

    run_gtk_icon_theme::<Lucide>("io.smearor.fonts_rs.lucide_icon_theme", "Lucide Icons - GtkIconTheme", &icons)
}
