//! Minimal example: compose multiple Redacted glyph SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_redacted::RedactedConfig;
use fonts_rs_redacted::all_glyphs;
use fonts_rs_redacted::naming::Redacted;
use fonts_rs_redacted::variant;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.redacted_composed_svg";

const SAMPLE_TEXT: &str = "HELLO";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let char_to_name: Vec<(char, &str)> = all_glyphs().collect();

    run_composed_svg_text::<Redacted, RedactedConfig>(
        APP_ID,
        "Redacted — Composed SVG Text",
        variant::GRESOURCE_PREFIX,
        SAMPLE_TEXT,
        "redacted text",
        &char_to_name,
        GLYPH_HEIGHT,
    )
}
