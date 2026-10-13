//! Minimal GTK example: load font from GResource and render emoji text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_blobmoji_bitmap_noflags::Blobmoji;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.blobmoji_gresource";

/// Sample text: 🎉🚀😍🔥
const SAMPLE_TEXT: &str = "🎉🚀😍🔥";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Blobmoji>(APP_ID, "Blobmoji - GTK GResource", SAMPLE_TEXT)
}
