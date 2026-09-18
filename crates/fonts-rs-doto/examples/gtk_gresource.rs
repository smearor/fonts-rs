//! GTK GResource example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_doto::naming::Doto;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.doto_gresource";

const FONT_FAMILY: &str = "Doto";

const SAMPLE_TEXT: &str = "HELLO";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Doto>(APP_ID, "Doto — GTK GResource", FONT_FAMILY, SAMPLE_TEXT)
}
