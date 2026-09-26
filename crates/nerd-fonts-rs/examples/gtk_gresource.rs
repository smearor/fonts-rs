//! Minimal GTK example: load font from GResource and render Nerd Font text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_test::run_gtk_gresource;
use nerd_fonts_rs::NerdFonts;

const APP_ID: &str = "io.smearor.fonts_rs.nerd_fonts_gresource";

/// Sample text:  (U+F005 star),  (U+F004 heart),  (U+F06D fire),  (U+F135 rocket)
const SAMPLE_TEXT: &str = "\u{F005}\u{F004}\u{F06D}\u{F135}";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<NerdFonts>(APP_ID, "Nerd Fonts - GTK GResource", SAMPLE_TEXT)
}
