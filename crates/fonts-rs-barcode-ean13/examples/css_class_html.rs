//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Demonstrates the CSS class loading path:
//! 1. `register_glyphs()` - registers SVG icon GResource
//! 2. `register_font()` - registers TTF font GResource
//! 3. `load_font()` - extracts TTF to temp file
//! 4. Generate an HTML file with `@font-face` referencing the TTF and
//!    a `.barcode-ean13` CSS class using `font-family: 'LibreBarcodeEAN13Text'`
//! 5. Open in a browser - the browser loads the font and renders the barcode
//!
//! This shows how to use the font in a web context with CSS classes.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_barcode_ean13::Ean13;
use fonts_rs_test::run_css_class_html;

const BARCODE_TEXT: &str = ":ABCDEFGHIJ*abcdefghij:";

fn main() -> miette::Result<()> {
    run_css_class_html::<Ean13>("barcode", BARCODE_TEXT, "Libre Barcode EAN13 - CSS Class", "barcode_ean13_css_class.html")
}
