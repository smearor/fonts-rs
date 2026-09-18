//! Minimal example: compose multiple Dicefont glyph SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_dicefont::DicefontConfig;
use fonts_rs_dicefont::all_glyphs;
use fonts_rs_dicefont::naming::Dicefont;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.dicefont_composed_svg";

/// Sample text: d6-1, d6-2, d6-3 (PUA codepoints U+F131–U+F133).
const SAMPLE_TEXT: &str = "\u{F131}\u{F132}\u{F133}";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let char_to_name: Vec<(char, &str)> = all_glyphs().collect();

    run_composed_svg_text::<Dicefont, DicefontConfig>(
        APP_ID,
        "Dicefont — Composed SVG Text",
        DicefontConfig::GRESOURCE_PREFIX,
        SAMPLE_TEXT,
        "dice glyph",
        &char_to_name,
        GLYPH_HEIGHT,
    )
}
