//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_noto_emoji::NotoEmojiConfig;
use fonts_rs_noto_emoji::all_glyphs;
use fonts_rs_noto_emoji::naming::NotoEmoji;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    let glyphs: Vec<(char, &str)> = all_glyphs().collect();

    run_svg_html::<NotoEmoji, NotoEmojiConfig>("Noto Emoji — SVG Glyphs", NotoEmojiConfig::GRESOURCE_PREFIX, &glyphs, "noto_emoji_svg_html.html")
}
