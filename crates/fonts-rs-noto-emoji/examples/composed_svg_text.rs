//! Minimal example: compose multiple Noto Emoji glyph SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_noto_emoji::NotoEmojiConfig;
use fonts_rs_noto_emoji::all_glyphs;
use fonts_rs_noto_emoji::naming::NotoEmoji;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.noto_emoji_composed_svg";

/// Sample text: 🎉🚀😍🔥
const SAMPLE_TEXT: &str = "🎉🚀😍🔥";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let char_to_name: Vec<(char, &str)> = all_glyphs().collect();

    run_composed_svg_text::<NotoEmoji, NotoEmojiConfig>(
        APP_ID,
        "Noto Emoji — Composed SVG Text",
        NotoEmojiConfig::GRESOURCE_PREFIX,
        SAMPLE_TEXT,
        "emoji",
        &char_to_name,
        GLYPH_HEIGHT,
    )
}
