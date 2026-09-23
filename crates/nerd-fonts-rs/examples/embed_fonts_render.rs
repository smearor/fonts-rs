//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```

use fonts_rs_test::run_embed_fonts_render;
use nerd_fonts_rs::NerdFonts;

const APP_ID: &str = "io.smearor.fonts_rs.nerd_fonts_embed_render";

/// Sample text:  (U+F005 star),  (U+F004 heart),  (U+F06D fire),  (U+F135 rocket)
const SAMPLE_TEXT: &str = "\u{F005}\u{F004}\u{F06D}\u{F135}";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<NerdFonts>(APP_ID, "Nerd Fonts — ab_glyph Software Rendering", SAMPLE_TEXT, FONT_SIZE)
}
