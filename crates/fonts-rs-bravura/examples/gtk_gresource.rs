//! GTK GResource example for Bravura.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_bravura::Bravura;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.bravura_gresource";

const SAMPLE_TEXT: &str = "\u{E050}\u{E0A4}\u{E083}";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Bravura>(APP_ID, "Bravura — GTK GResource", SAMPLE_TEXT)
}
