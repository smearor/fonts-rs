//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! With `embed-fonts`, `include_bytes!` bakes the variant's TTF into
//! the binary - no `resources/` directory needed at runtime.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```

use fonts_rs_fluent::Fluent;
use fonts_rs_fluent::variant;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.fluent_embed_render";

/// Sample text built from icon codepoints (PUA chars).
fn sample_text() -> String {
    let prefix = variant::GLYPH_PREFIX;
    ["home", "heart", "star", "alert"]
        .into_iter()
        .filter_map(|stem| Fluent::codepoint(format!("{prefix}-{stem}-symbolic")).map(|cp| cp.as_char()))
        .collect()
}

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<Fluent>(APP_ID, "Fluent UI System Icons - ab_glyph Software Rendering", &sample_text(), FONT_SIZE)
}
