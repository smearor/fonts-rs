//! Minimal GTK example: load font from GResource and render placeholder text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_redacted::Redacted;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.redacted_gresource";

const SAMPLE_TEXT: &str = "HELLO";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Redacted>(APP_ID, "Redacted — GTK GResource", SAMPLE_TEXT)
}
