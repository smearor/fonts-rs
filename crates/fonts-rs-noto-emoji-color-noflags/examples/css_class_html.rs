//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_noto_emoji_color_noflags::NotoEmoji;
use fonts_rs_test::run_css_class_html;

/// Sample text: 🎉🚀😍🔥
const SAMPLE_TEXT: &str = "🎉🚀😍🔥";

fn main() -> miette::Result<()> {
    run_css_class_html::<NotoEmoji>("emoji", SAMPLE_TEXT, "Noto Emoji - CSS Class", "noto_emoji_css_class.html")
}
