//! Minimal GTK example: use GtkIconTheme to look up Tabler icon SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_tabler::Tabler;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    // Icon names are variant-independent (`tabler-{name}-symbolic`), but
    // the filled variant ships a subset - names are filtered through
    // `codepoint` so the example works under either variant.
    let icons: Vec<(String, &str)> = [
        ("tabler-home-symbolic", "Home"),
        ("tabler-heart-symbolic", "Heart"),
        ("tabler-star-symbolic", "Star"),
        ("tabler-settings-symbolic", "Settings"),
        ("tabler-bell-symbolic", "Bell"),
        ("tabler-search-symbolic", "Search"),
    ]
    .into_iter()
    .filter_map(|(name, label)| Tabler::codepoint(name).map(|_| (name.to_string(), label)))
    .collect();

    run_gtk_icon_theme::<Tabler>("io.smearor.fonts_rs.tabler_icon_theme", "Tabler Icons - GtkIconTheme", &icons)
}
