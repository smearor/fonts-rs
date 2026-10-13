//! Minimal example: compose multiple Blobmoji glyph SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_blobmoji_bitmap_noflags::Blobmoji;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.blobmoji_composed_svg";

/// Sample text: 🎉🚀😍🔥
const SAMPLE_TEXT: &str = "🎉🚀😍🔥";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<Blobmoji>(APP_ID, "Blobmoji - Composed SVG Text", SAMPLE_TEXT, "emoji", GLYPH_HEIGHT)
}
