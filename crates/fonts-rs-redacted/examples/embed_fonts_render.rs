//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```

use fonts_rs_redacted::Redacted;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.redacted_embed_render";

const SAMPLE_TEXT: &str = "HELLO";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<Redacted>(APP_ID, "Redacted - ab_glyph Software Rendering", SAMPLE_TEXT, FONT_SIZE)
}
