//! GtkIconTheme example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_doto::Doto;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons: Vec<_> = Doto::all_glyphs().take(4).map(|(_, name)| (name.clone(), name)).collect();

    run_gtk_icon_theme::<Doto>("io.smearor.fonts_rs.doto_icon_theme", "Doto - GtkIconTheme", &icons)
}
