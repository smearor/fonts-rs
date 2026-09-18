//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_cuernavaca::naming::Cuernavaca;
use fonts_rs_test::run_css_class_html;

const FONT_FAMILY: &str = "ChessCuernavaca";

/// Sample text: King, Queen, Rook, Bishop, Knight, Pawn.
const SAMPLE_TEXT: &str = "KQRBNP";

fn main() -> miette::Result<()> {
    run_css_class_html::<Cuernavaca>(
        FONT_FAMILY,
        "cuernavaca",
        "chess notation",
        SAMPLE_TEXT,
        "Cuernavaca — CSS Class",
        "cuernavaca_css_class.html",
    )
}
