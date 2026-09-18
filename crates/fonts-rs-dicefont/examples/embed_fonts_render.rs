//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```

use fonts_rs_dicefont::fonts;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.dicefont_embed_render";

/// Sample text: d6-1, d6-2, d6-3 (PUA codepoints U+F131–U+F133).
const SAMPLE_TEXT: &str = "\u{F131}\u{F132}\u{F133}";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let font = fonts::font().expect("embedded font should be available with --features embed-fonts");

    run_embed_fonts_render(APP_ID, "Dicefont — ab_glyph Software Rendering", font, SAMPLE_TEXT, FONT_SIZE)
}
