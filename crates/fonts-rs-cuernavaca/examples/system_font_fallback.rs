//! Minimal example: system font fallback when the chess font is NOT loaded.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_cuernavaca::Cuernavaca;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.cuernavaca_fallback";

/// Sample text: King, Queen, Rook, Bishop, Knight, Pawn.
const SAMPLE_TEXT: &str = "KQRBNP";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<Cuernavaca>(
        APP_ID,
        "Cuernavaca — System Font Fallback",
        SAMPLE_TEXT,
        "Font NOT loaded — system font fallback is used. Chess piece glyphs will not render correctly.",
    )
}
