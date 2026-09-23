//! Minimal GTK example: use GtkIconTheme to look up EAN13 glyph SVGs by icon name.
//!
//! Demonstrates the GtkIconTheme rendering path:
//! 1. `register_glyphs()` — registers SVG icon GResource bundle
//! 2. `GtkIconTheme` — resolves icon names to `IconPaintable`s
//! 3. `GtkImage::from_icon_name()` — displays each glyph as an icon
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_barcode_ean13::Ean13;
use fonts_rs_barcode_ean13::constants;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [
        (constants::GUARD_NORMAL, "guard-normal"),
        (constants::ZERO_COMPATIBILITY, "zero-compatibility"),
        (constants::ONE_COMPATIBILITY, "one-compatibility"),
        (constants::TWO_COMPATIBILITY, "two-compatibility"),
    ];

    run_gtk_icon_theme::<Ean13>("io.smearor.fonts_rs.barcode_ean13_icon_theme", "EAN13 — GtkIconTheme", &icons)
}
