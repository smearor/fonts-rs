//! Minimal GTK example: use GtkIconTheme to look up Code 128 glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_barcode_code128::Code128;
use fonts_rs_barcode_code128::constants;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [(constants::H, "H"), (constants::E, "E"), (constants::L, "L"), (constants::O, "O")];

    run_gtk_icon_theme::<Code128>("io.smearor.fonts_rs.barcode_code128_icon_theme", "Code128 - GtkIconTheme", &icons)
}
