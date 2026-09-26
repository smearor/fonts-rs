//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```
//!
//! The Eva webfont carries real `glyf` outlines, so ab_glyph renders
//! monochrome silhouettes under either variant.

use fonts_rs_eva::Eva;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.eva_embed_render";

const FONT_SIZE: f32 = 64.0;

/// Builds the sample text from icon codepoints resolved at runtime -
/// codepoints differ between the fill and outline variants.
fn sample_text() -> String {
    ["eva-home-symbolic", "eva-heart-symbolic", "eva-star-symbolic", "eva-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Eva::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<Eva>(APP_ID, "Eva Icons - ab_glyph Software Rendering", &sample_text(), FONT_SIZE)
}
