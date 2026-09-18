//! CSS class HTML example for Bravura.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_bravura::naming::Bravura;
use fonts_rs_test::run_css_class_html;

const FONT_FAMILY: &str = "Bravura";

const SAMPLE_TEXT: &str = "\u{E050}\u{E0A4}\u{E083}";

fn main() -> miette::Result<()> {
    run_css_class_html::<Bravura>(
        FONT_FAMILY,
        "bravura",
        "music notation",
        SAMPLE_TEXT,
        "Bravura — CSS Class",
        "bravura_css_class.html",
    )
}
