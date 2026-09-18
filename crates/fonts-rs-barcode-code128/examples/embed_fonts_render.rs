//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```

use fonts_rs_barcode_code128::fonts;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_code128_embed_render";

const BARCODE_TEXT: &str = "HELLO";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let font = fonts::font().expect("embedded font should be available with --features embed-fonts");

    run_embed_fonts_render(APP_ID, "Code128 — ab_glyph Software Rendering", font, BARCODE_TEXT, FONT_SIZE)
}
