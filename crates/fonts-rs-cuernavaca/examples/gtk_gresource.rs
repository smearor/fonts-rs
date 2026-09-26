//! Minimal GTK example: load font from GResource and render chess piece text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_cuernavaca::Cuernavaca;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.cuernavaca_gresource";

/// Sample text: King, Queen, Rook, Bishop, Knight, Pawn.
const SAMPLE_TEXT: &str = "KQRBNP";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Cuernavaca>(APP_ID, "Cuernavaca - GTK GResource", SAMPLE_TEXT)
}
