//! Minimal GTK example: use GtkIconTheme to look up Code 39 glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_barcode_code39::Code39;
use fonts_rs_barcode_code39::Code39Definition;
use fonts_rs_barcode_code39::constants;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [
        (constants::UNI002A, "uni002a"),
        (constants::UNI0048, "uni0048"),
        (constants::UNI0045, "uni0045"),
        (constants::UNI004C, "uni004c"),
    ];

    run_gtk_icon_theme::<Code39, Code39Definition>(
        "io.smearor.fonts_rs.barcode_code39_icon_theme",
        "Code39 — GtkIconTheme",
        Code39Definition::GRESOURCE_PREFIX,
        &icons,
    )
}
