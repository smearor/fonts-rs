//! Composed SVG text example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example composed_svg_text --features gtk
//! ```

use fonts_rs_doto::DotoConfig;
use fonts_rs_doto::all_glyphs;
use fonts_rs_doto::naming::Doto;
use fonts_rs_doto::variant;
use fonts_rs_test::run_composed_svg_text;

const APP_ID: &str = "io.smearor.fonts_rs.doto_composed_svg";

const SAMPLE_TEXT: &str = "HELLO";

const GLYPH_HEIGHT: f64 = 200.0;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let char_to_name: Vec<(char, &str)> = all_glyphs().collect();

    run_composed_svg_text::<Doto, DotoConfig>(
        APP_ID,
        "Doto — Composed SVG Text",
        variant::GRESOURCE_PREFIX,
        SAMPLE_TEXT,
        "dot matrix text",
        &char_to_name,
        GLYPH_HEIGHT,
    )
}
