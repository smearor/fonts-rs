//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_noto_emoji::NotoEmoji;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.noto_emoji_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<_> = NotoEmoji::all_glyphs().take(4).collect();

    run_svg_gresource::<NotoEmoji>(APP_ID, "Noto Emoji — SVG GResource", &glyphs)
}
