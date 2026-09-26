//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts,bitmap-flags
//! ```
//!
//! `bitmap-flags` is a required feature: only the CBDT strikes render
//! in color via ab_glyph. Under `color-*` (COLRv1, the default) the
//! font's `glyf` outlines are empty — ab_glyph has no COLR support and
//! would render a black box. Under `mono` the glyphs render as white
//! silhouettes.

use fonts_rs_noto_emoji::NotoEmoji;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.noto_emoji_embed_render";

/// Sample text: 🎉🚀😍🔥
const SAMPLE_TEXT: &str = "🎉🚀😍🔥";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<NotoEmoji>(APP_ID, "Noto Emoji — ab_glyph Software Rendering", SAMPLE_TEXT, FONT_SIZE)
}
