//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! With `embed-fonts`, `include_bytes!` bakes all three TTFs
//! (`la-regular-400`, `la-solid-900`, `la-brands-400`) into the
//! binary - no `resources/` directory needed at runtime.
//! `LineAwesome::font()` returns the default (solid) face; per-style
//! faces are available via `LineAwesome::ab_glyph(LineAwesomeStyle::*)`.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```

use fonts_rs_line_awesome::GLYPH_PREFIX;
use fonts_rs_line_awesome::LineAwesome;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.line_awesome_embed_render";

/// Sample text built from icon codepoints (PUA chars).
fn sample_text() -> String {
    let prefix = GLYPH_PREFIX;
    ["heart", "star", "bell", "camera-solid"]
        .into_iter()
        .filter_map(|stem| LineAwesome::codepoint(format!("{prefix}-{stem}-symbolic")).map(|cp| cp.as_char()))
        .collect()
}

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<LineAwesome>(APP_ID, "Line Awesome - ab_glyph Software Rendering", &sample_text(), FONT_SIZE)
}
