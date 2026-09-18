//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_noto_emoji::NotoEmojiConfig;
use fonts_rs_noto_emoji::all_glyphs;
use fonts_rs_noto_emoji::naming::NotoEmoji;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.noto_emoji_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<(char, &str)> = all_glyphs().take(4).collect();

    run_svg_gresource::<NotoEmoji, NotoEmojiConfig>(APP_ID, "Noto Emoji — SVG GResource", NotoEmojiConfig::GRESOURCE_PREFIX, &glyphs)
}
