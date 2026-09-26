//! Embed-fonts render example for Bravura.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```

use fonts_rs_bravura::Bravura;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.bravura_embed_render";

const SAMPLE_TEXT: &str = "\u{E050}\u{E0A4}\u{E083}";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<Bravura>(APP_ID, "Bravura - ab_glyph Software Rendering", SAMPLE_TEXT, FONT_SIZE)
}
