//! Composed SVG text example for Bravura.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_bravura::BravuraConfig;
use fonts_rs_bravura::all_glyphs;
use fonts_rs_bravura::naming::Bravura;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.bravura_composed_svg";

const SAMPLE_TEXT: &str = "\u{E050}\u{E0A4}\u{E083}";

const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let char_to_name: Vec<(char, &str)> = all_glyphs().collect();

    run_composed_svg_text::<Bravura, BravuraConfig>(
        APP_ID,
        "Bravura — Composed SVG Text",
        BravuraConfig::GRESOURCE_PREFIX,
        SAMPLE_TEXT,
        "music notation",
        &char_to_name,
        GLYPH_HEIGHT,
    )
}
