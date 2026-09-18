//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_redacted::naming::Redacted;
use fonts_rs_test::run_css_class_html;

const FONT_FAMILY: &str = "Redacted";

const SAMPLE_TEXT: &str = "HELLO";

fn main() -> miette::Result<()> {
    run_css_class_html::<Redacted>(
        FONT_FAMILY,
        "redacted",
        "redacted text",
        SAMPLE_TEXT,
        "Redacted — CSS Class",
        "redacted_css_class.html",
    )
}
