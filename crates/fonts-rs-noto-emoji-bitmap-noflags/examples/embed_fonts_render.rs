//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```
//!
//! This example only exists in the renderable variants: the CBDT
//! strikes render in color via ab_glyph, and `mono` produces white
//! silhouettes. The `color-*` crates (COLRv1, empty `glyf` outlines)
//! ship no pixel-drawing example.

use fonts_rs_noto_emoji_bitmap_noflags::NotoEmoji;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.noto_emoji_embed_render";

/// Sample text: 🎉🚀😍🔥
const SAMPLE_TEXT: &str = "🎉🚀😍🔥";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<NotoEmoji>(APP_ID, "Noto Emoji - ab_glyph Software Rendering", SAMPLE_TEXT, FONT_SIZE)
}
