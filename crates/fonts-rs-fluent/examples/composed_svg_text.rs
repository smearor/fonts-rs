//! Minimal example: compose multiple Fluent icon SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_fluent::Fluent;
use fonts_rs_fluent::variant;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.fluent_composed_svg";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

/// Builds the sample text from icon codepoints resolved at runtime.
fn sample_text() -> String {
    let prefix = variant::GLYPH_PREFIX;
    ["home", "heart", "star", "alert"]
        .into_iter()
        .filter_map(|stem| Fluent::codepoint(format!("{prefix}-{stem}-symbolic")).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<Fluent>(APP_ID, "Fluent UI System Icons - Composed SVG Text", &sample_text(), "icon", GLYPH_HEIGHT)
}
