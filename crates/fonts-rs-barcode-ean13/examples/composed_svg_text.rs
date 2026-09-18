//! Minimal example: compose multiple EAN13 glyph SVGs into a single SVG barcode.
//!
//! Demonstrates the composed SVG text rendering path:
//! 1. `register_glyphs()` — registers SVG icon GResource
//! 2. `gio::resources_lookup_data()` — loads individual glyph SVGs
//! 3. Extract `viewBox` and inner content from each glyph SVG
//! 4. Compose into a single SVG with nested `<svg>` elements side by side
//! 5. Render as `gdk::Texture` and display in a `Picture` widget
//!
//! This path renders a complete barcode from pre-generated SVG glyph
//! outlines — no TTF font, no Pango, no ab_glyph required.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_barcode_ean13::Ean13;
use fonts_rs_barcode_ean13::Ean13Definition;
use fonts_rs_barcode_ean13::all_glyphs;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_ean13_composed_svg";

/// EAN13-encoded string (guard + left group + centre + right group + guard).
const BARCODE_TEXT: &str = ":ABCDEFGHIJ*abcdefghij:";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let char_to_name: Vec<(char, &str)> = all_glyphs().collect();

    run_composed_svg_text::<Ean13, Ean13Definition>(
        APP_ID,
        "EAN13 — Composed SVG Text",
        Ean13Definition::GRESOURCE_PREFIX,
        BARCODE_TEXT,
        "barcode",
        &char_to_name,
        GLYPH_HEIGHT,
    )
}
