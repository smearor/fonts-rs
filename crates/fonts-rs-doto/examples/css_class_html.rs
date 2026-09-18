//! CSS class HTML example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_doto::naming::Doto;
use fonts_rs_test::run_css_class_html;

const FONT_FAMILY: &str = "Doto";

const SAMPLE_TEXT: &str = "HELLO";

fn main() -> miette::Result<()> {
    run_css_class_html::<Doto>(FONT_FAMILY, "doto", "dot matrix text", SAMPLE_TEXT, "Doto — CSS Class", "doto_css_class.html")
}
