//! Minimal example: generate an HTML page with inline SVG glyphs from GResource.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use fonts_rs_noto_emoji_mono::NotoEmoji;
use fonts_rs_test::run_svg_html;

fn main() -> miette::Result<()> {
    run_svg_html::<NotoEmoji>("Noto Emoji - SVG Glyphs", "noto_emoji_svg_html.html")
}
