//! Minimal example: compose multiple Lucide icon SVGs into a single SVG.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_lucide::Lucide;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.lucide_composed_svg";

/// Target height for each glyph in the composed SVG (in SVG user units).
const GLYPH_HEIGHT: f64 = 200.0;

/// Builds the sample text from icon codepoints resolved at runtime.
fn sample_text() -> String {
    ["lucide-house-symbolic", "lucide-heart-symbolic", "lucide-star-symbolic", "lucide-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Lucide::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_composed_svg_text::<Lucide>(APP_ID, "Lucide Icons - Composed SVG Text", &sample_text(), "icon", GLYPH_HEIGHT)
}
