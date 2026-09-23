//! Minimal GTK example: load font from GResource and render fourteen-segment text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_fourteen_segment::FourteenSegment;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.fourteen_segment_gresource";

const SAMPLE_TEXT: &str = "ABCDEF12";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<FourteenSegment>(APP_ID, "DSEG14 — GTK GResource", SAMPLE_TEXT)
}
