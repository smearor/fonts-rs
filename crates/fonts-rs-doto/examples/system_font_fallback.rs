//! System font fallback example for Doto.
//!
//! Run with:
//! ```sh
//! cargo run --example system_font_fallback --features gtk
//! ```

use fonts_rs_doto::Doto;
use fonts_rs_test::run_system_font_fallback;

const APP_ID: &str = "io.smearor.fonts_rs.doto_fallback";

const SAMPLE_TEXT: &str = "HELLO";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_system_font_fallback::<Doto>(
        APP_ID,
        "Doto — System Font Fallback",
        SAMPLE_TEXT,
        "Font NOT loaded — system font fallback is used. The text is not rendered as dot-matrix.",
    )
}
