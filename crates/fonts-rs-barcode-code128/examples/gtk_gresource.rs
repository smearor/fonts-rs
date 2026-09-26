//! Minimal GTK example: load font from GResource and render barcode text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_barcode_code128::Code128;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_code128_gresource";

const BARCODE_TEXT: &str = "HELLO";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Code128>(APP_ID, "Code128 - GTK GResource", BARCODE_TEXT)
}
