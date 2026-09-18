//! GtkIconTheme example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_doto::DotoConfig;
use fonts_rs_doto::all_glyphs;
use fonts_rs_doto::naming::Doto;
use fonts_rs_doto::variant;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons: Vec<(&str, &str)> = all_glyphs().take(4).map(|(_, name)| (name, name)).collect();

    run_gtk_icon_theme::<Doto, DotoConfig>("io.smearor.fonts_rs.doto_icon_theme", "Doto — GtkIconTheme", variant::GRESOURCE_PREFIX, &icons)
}
