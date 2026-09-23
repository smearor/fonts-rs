//! Minimal embed-fonts example: render text with ab_glyph and display in GTK 4.
//!
//! Demonstrates the full software rendering pipeline:
//! 1. `fonts::font()` — returns `&FontVec` embedded via `include_bytes!`
//! 2. `ab_glyph` — rasterize glyphs into an RGBA pixel buffer
//! 3. `gdk::MemoryTexture` — wrap the pixel buffer as a GTK paintable
//! 4. `Picture` widget — display the rendered text in a GTK window
//!
//! No Pango font map involved — the font is rendered entirely in software.
//!
//! Run with:
//! ```sh
//! cargo run --example embed_fonts_render --features gtk,render,embed-fonts
//! ```

use fonts_rs_barcode_ean13::Ean13;
use fonts_rs_test::run_embed_fonts_render;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_ean13_embed_render";

/// EAN13-encoded string (guard + left group + centre + right group + guard).
const BARCODE_TEXT: &str = ":ABCDEFGHIJ*abcdefghij:";

const FONT_SIZE: f32 = 64.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_embed_fonts_render::<Ean13>(APP_ID, "EAN13 — ab_glyph Software Rendering", BARCODE_TEXT, FONT_SIZE)
}
