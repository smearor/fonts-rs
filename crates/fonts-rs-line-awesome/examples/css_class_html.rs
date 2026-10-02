//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! The TTF path of the hybrid set: the bundled `la-solid-900.ttf` is
//! embedded as a base64 data URI and the icons render as text via
//! `font-family` - the default face covers regular and solid glyphs
//! (they share the FA5 codepoints); brand glyphs need the
//! `la-brands-400` face, which `HtmlExport` does not embed.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_line_awesome::GLYPH_PREFIX;
use fonts_rs_line_awesome::LineAwesome;
use fonts_rs_test::run_css_class_html;

/// Sample text built from icon codepoints (PUA chars).
fn sample_text() -> String {
    let prefix = GLYPH_PREFIX;
    ["heart", "star", "bell", "camera-solid"]
        .into_iter()
        .filter_map(|stem| LineAwesome::codepoint(format!("{prefix}-{stem}-symbolic")).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<()> {
    run_css_class_html::<LineAwesome>("icons", &sample_text(), "Line Awesome - @font-face", "line_awesome_css_class.html")
}
