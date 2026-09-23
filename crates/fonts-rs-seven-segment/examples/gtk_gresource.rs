//! Minimal GTK example: load font from GResource and render seven-segment text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_seven_segment::SevenSegment;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.seven_segment_gresource";

const SAMPLE_TEXT: &str = "12345678";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<SevenSegment>(APP_ID, "DSEG7 — GTK GResource", SAMPLE_TEXT)
}
