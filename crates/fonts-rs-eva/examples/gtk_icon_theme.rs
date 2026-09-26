//! Minimal GTK example: use GtkIconTheme to look up Eva icon SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_eva::Eva;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    // Icon names are variant-independent (`eva-{name}-symbolic`), but
    // the variants ship slightly different sets - names are filtered
    // through `codepoint` so the example works under either variant.
    let icons: Vec<(String, &str)> = [
        ("eva-home-symbolic", "Home"),
        ("eva-heart-symbolic", "Heart"),
        ("eva-star-symbolic", "Star"),
        ("eva-settings-symbolic", "Settings"),
        ("eva-bell-symbolic", "Bell"),
        ("eva-search-symbolic", "Search"),
    ]
    .into_iter()
    .filter_map(|(name, label)| Eva::codepoint(name).map(|_| (name.to_string(), label)))
    .collect();

    run_gtk_icon_theme::<Eva>("io.smearor.fonts_rs.eva_icon_theme", "Eva Icons - GtkIconTheme", &icons)
}
