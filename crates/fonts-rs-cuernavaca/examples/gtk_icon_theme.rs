//! Minimal GTK example: use GtkIconTheme to look up Cuernavaca glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_cuernavaca::Cuernavaca;
use fonts_rs_cuernavaca::constants;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [
        (constants::CUERNAVACA_K, "King"),
        (constants::CUERNAVACA_Q, "Queen"),
        (constants::CUERNAVACA_R, "Rook"),
        (constants::CUERNAVACA_B, "Bishop"),
    ];

    run_gtk_icon_theme::<Cuernavaca>("io.smearor.fonts_rs.cuernavaca_icon_theme", "Cuernavaca — GtkIconTheme", &icons)
}
