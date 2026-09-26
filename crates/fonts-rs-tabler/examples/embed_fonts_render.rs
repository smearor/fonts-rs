//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```
//!
//! The Tabler webfont carries real `glyf` outlines, so ab_glyph renders
//! monochrome silhouettes under either variant.

use fonts_rs_tabler::Tabler;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.tabler_embed_render";

const FONT_SIZE: f32 = 64.0;

/// Builds the sample text from icon codepoints resolved at runtime -
/// codepoints differ between the outline and filled variants.
fn sample_text() -> String {
    ["tabler-home-symbolic", "tabler-heart-symbolic", "tabler-star-symbolic", "tabler-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Tabler::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<Tabler>(APP_ID, "Tabler Icons - ab_glyph Software Rendering", &sample_text(), FONT_SIZE)
}
