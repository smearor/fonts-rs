//! Minimal example: generate an HTML page with @font-face and CSS class names.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use fonts_rs_eva::Eva;
use fonts_rs_test::run_css_class_html;

/// Builds the sample text from icon codepoints resolved at runtime -
/// codepoints differ between the fill and outline variants.
fn sample_text() -> String {
    ["eva-home-symbolic", "eva-heart-symbolic", "eva-star-symbolic", "eva-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Eva::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<()> {
    run_css_class_html::<Eva>("icon", &sample_text(), "Eva Icons - CSS Class", "eva_css_class.html")
}
