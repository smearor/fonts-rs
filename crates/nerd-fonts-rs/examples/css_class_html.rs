//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_test::run_css_class_html;
use nerd_fonts_rs::NerdFonts;

const FONT_FAMILY: &str = "Symbols Nerd Font";

/// Sample text:  (U+F005 star),  (U+F004 heart),  (U+F06D fire),  (U+F135 rocket)
const SAMPLE_TEXT: &str = "\u{F005}\u{F004}\u{F06D}\u{F135}";

fn main() -> miette::Result<()> {
    run_css_class_html::<NerdFonts>(
        FONT_FAMILY,
        "nerd-fonts",
        "icon",
        SAMPLE_TEXT,
        "Nerd Fonts — CSS Class",
        "nerd_fonts_css_class.html",
    )
}
