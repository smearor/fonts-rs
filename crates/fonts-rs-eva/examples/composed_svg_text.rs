//! Minimal example: compose multiple Eva icon SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_eva::Eva;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.eva_composed_svg";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

/// Builds the sample text from icon codepoints resolved at runtime —
/// codepoints differ between the fill and outline variants.
fn sample_text() -> String {
    ["eva-home-symbolic", "eva-heart-symbolic", "eva-star-symbolic", "eva-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Eva::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<Eva>(APP_ID, "Eva Icons — Composed SVG Text", &sample_text(), "icon", GLYPH_HEIGHT)
}
