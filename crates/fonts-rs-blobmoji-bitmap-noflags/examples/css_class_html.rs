//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

// Note: the bitmap variants embed a CBDT font. Browsers reject CBDT
// web fonts in their sanitizer - Firefox needs the pref
// `gfx.downloadable_fonts.keep_color_bitmaps = true` (default: false,
// Mozilla bug 1418818), desktop Chrome has no CBDT support at all.
// For a working @font-face demo use a `color-*` variant (COLRv1).
use fonts_rs_blobmoji_bitmap_noflags::Blobmoji;
use fonts_rs_test::run_css_class_html;

/// Sample text: 🎉🚀😍🔥
const SAMPLE_TEXT: &str = "🎉🚀😍🔥";

fn main() -> miette::Result<()> {
    run_css_class_html::<Blobmoji>("emoji", SAMPLE_TEXT, "Blobmoji - CSS Class", "blobmoji_css_class.html")
}
