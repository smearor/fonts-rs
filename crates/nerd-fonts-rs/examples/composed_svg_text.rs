//! Minimal example: compose multiple Nerd Font glyph SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_composed_svg_text;
use nerd_fonts_rs::NerdFonts;
use nerd_fonts_rs::NerdFontsConfig;
use nerd_fonts_rs::all_icons;

const APP_ID: &str = "io.smearor.fonts_rs.nerd_fonts_composed_svg";

/// Sample text:  (U+F005 star),  (U+F004 heart),  (U+F06D fire),  (U+F135 rocket)
const SAMPLE_TEXT: &str = "\u{F005}\u{F004}\u{F06D}\u{F135}";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let char_to_name: Vec<(char, &str)> = all_icons().collect();

    run_composed_svg_text::<NerdFonts, NerdFontsConfig>(
        APP_ID,
        "Nerd Fonts — Composed SVG Text",
        NerdFontsConfig::GRESOURCE_PREFIX,
        SAMPLE_TEXT,
        "icon",
        &char_to_name,
        GLYPH_HEIGHT,
    )
}
