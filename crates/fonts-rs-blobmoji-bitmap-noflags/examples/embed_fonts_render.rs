//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```
//!
//! This example only exists in the renderable variants: the CBDT
//! strikes render in color via ab_glyph. The `color-*` crates (COLRv1,
//! empty `glyf` outlines) ship no pixel-drawing example - ab_glyph has
//! no COLR support and would render a black box.

use fonts_rs_blobmoji_bitmap_noflags::Blobmoji;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.blobmoji_embed_render";

/// Sample text: 🎉🚀😍🔥
const SAMPLE_TEXT: &str = "🎉🚀😍🔥";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<Blobmoji>(APP_ID, "Blobmoji - ab_glyph Software Rendering", SAMPLE_TEXT, FONT_SIZE)
}
