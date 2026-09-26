//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```

use fonts_rs_barcode_code39::Code39;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_code39_embed_render";

const BARCODE_TEXT: &str = "*HELLO*";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<Code39>(APP_ID, "Code39 - ab_glyph Software Rendering", BARCODE_TEXT, FONT_SIZE)
}
