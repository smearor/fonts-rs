//! GTK GResource example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_doto::Doto;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.doto_gresource";

const SAMPLE_TEXT: &str = "HELLO";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Doto>(APP_ID, "Doto — GTK GResource", SAMPLE_TEXT)
}
