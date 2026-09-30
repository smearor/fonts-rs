//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! The TTF path of the hybrid set: the bundled
//! `FluentSystemIcons-{Regular,Filled}.ttf` is embedded as a base64
//! data URI and the icons render as text via `font-family`.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_fluent::Fluent;
use fonts_rs_fluent::variant;
use fonts_rs_test::run_css_class_html;

/// Sample text built from icon codepoints (PUA chars).
fn sample_text() -> String {
    let prefix = variant::GLYPH_PREFIX;
    ["home", "heart", "star", "alert"]
        .into_iter()
        .filter_map(|stem| Fluent::codepoint(format!("{prefix}-{stem}-symbolic")).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<()> {
    run_css_class_html::<Fluent>("icons", &sample_text(), "Fluent UI System Icons - @font-face", "fluent_css_class.html")
}
