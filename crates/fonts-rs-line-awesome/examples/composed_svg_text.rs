//! Minimal example: compose multiple Line Awesome icon SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_line_awesome::GLYPH_PREFIX;
use fonts_rs_line_awesome::LineAwesome;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.line_awesome_composed_svg";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

/// Builds the sample text from icon codepoints resolved at runtime.
fn sample_text() -> String {
    let prefix = GLYPH_PREFIX;
    ["heart", "star", "bell", "camera-solid"]
        .into_iter()
        .filter_map(|stem| LineAwesome::codepoint(format!("{prefix}-{stem}-symbolic")).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<LineAwesome>(APP_ID, "Line Awesome - Composed SVG Text", &sample_text(), "icon", GLYPH_HEIGHT)
}
