//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_barcode_code128::Code128;
use fonts_rs_test::run_css_class_html;

const BARCODE_TEXT: &str = "HELLO";

fn main() -> miette::Result<()> {
    run_css_class_html::<Code128>("barcode", BARCODE_TEXT, "Libre Barcode Code 128 - CSS Class", "barcode_code128_css_class.html")
}
