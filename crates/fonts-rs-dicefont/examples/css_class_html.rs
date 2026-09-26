//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_dicefont::Dicefont;
use fonts_rs_test::run_css_class_html;

/// Sample text: d6-1, d6-2, d6-3 (PUA codepoints U+F131–U+F133).
const SAMPLE_TEXT: &str = "\u{F131}\u{F132}\u{F133}";

fn main() -> miette::Result<()> {
    run_css_class_html::<Dicefont>("dice glyph", SAMPLE_TEXT, "Dicefont - CSS Class", "dicefont_css_class.html")
}
