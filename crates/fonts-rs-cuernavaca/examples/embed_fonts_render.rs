//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```

use fonts_rs_cuernavaca::fonts;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.cuernavaca_embed_render";

/// Sample text: King, Queen, Rook, Bishop, Knight, Pawn.
const SAMPLE_TEXT: &str = "KQRBNP";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let font = fonts::font().expect("embedded font should be available with --features embed-fonts");

    run_embed_fonts_render(APP_ID, "Cuernavaca — ab_glyph Software Rendering", font, SAMPLE_TEXT, FONT_SIZE)
}
