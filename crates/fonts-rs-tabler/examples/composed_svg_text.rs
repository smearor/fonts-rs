//! Minimal example: compose multiple Tabler icon SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_tabler::Tabler;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.tabler_composed_svg";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

/// Builds the sample text from icon codepoints resolved at runtime —
/// codepoints differ between the outline and filled variants.
fn sample_text() -> String {
    ["tabler-home-symbolic", "tabler-heart-symbolic", "tabler-star-symbolic", "tabler-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Tabler::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<Tabler>(APP_ID, "Tabler Icons — Composed SVG Text", &sample_text(), "icon", GLYPH_HEIGHT)
}
