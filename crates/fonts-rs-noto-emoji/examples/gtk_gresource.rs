//! Minimal GTK example: load font from GResource and render emoji text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_noto_emoji::NotoEmoji;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.noto_emoji_gresource";

/// Sample text: 🎉🚀😍🔥
const SAMPLE_TEXT: &str = "🎉🚀😍🔥";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<NotoEmoji>(APP_ID, "Noto Emoji — GTK GResource", SAMPLE_TEXT)
}
