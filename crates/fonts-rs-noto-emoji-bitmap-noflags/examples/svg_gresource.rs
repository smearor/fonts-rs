//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_noto_emoji_bitmap_noflags::NotoEmoji;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.noto_emoji_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    // Pick expressive glyphs - the first entries of `all_glyphs()` are
    // the plain digit glyphs, which look monochrome by design.
    let glyphs: Vec<_> = ['\u{1F389}', '\u{1F680}', '\u{1F60D}', '\u{1F525}']
        .into_iter()
        .filter_map(|ch| NotoEmoji::glyph_name(ch).map(|name| (fonts_rs_model::CodePoint::from(ch), name.as_ref().to_string())))
        .collect();

    run_svg_gresource::<NotoEmoji>(APP_ID, "Noto Emoji - SVG GResource", &glyphs)
}
