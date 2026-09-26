//! Minimal GTK example: load font from GResource and render dice face text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_dicefont::Dicefont;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.dicefont_gresource";

/// Sample text: d6-1, d6-2, d6-3 (PUA codepoints U+F131–U+F133).
const SAMPLE_TEXT: &str = "\u{F131}\u{F132}\u{F133}";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Dicefont>(APP_ID, "Dicefont - GTK GResource", SAMPLE_TEXT)
}
