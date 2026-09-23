//! Minimal GTK example: use GtkIconTheme to look up Dicefont glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_dicefont::Dicefont;
use fonts_rs_dicefont::constants;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [
        (constants::DICEFONT_D6_1, "d6-1"),
        (constants::DICEFONT_D6_2, "d6-2"),
        (constants::DICEFONT_D6_3, "d6-3"),
        (constants::DICEFONT_D6_4, "d6-4"),
    ];

    run_gtk_icon_theme::<Dicefont>("io.smearor.fonts_rs.dicefont_icon_theme", "Dicefont — GtkIconTheme", &icons)
}
