//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_tabler::Tabler;
use fonts_rs_test::run_css_class_html;

/// Builds the sample text from icon codepoints resolved at runtime -
/// codepoints differ between the outline and filled variants.
fn sample_text() -> String {
    ["tabler-home-symbolic", "tabler-heart-symbolic", "tabler-star-symbolic", "tabler-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Tabler::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<()> {
    run_css_class_html::<Tabler>("icon", &sample_text(), "Tabler Icons - CSS Class", "tabler_css_class.html")
}
