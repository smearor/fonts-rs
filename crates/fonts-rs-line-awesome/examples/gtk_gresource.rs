//! Minimal GTK example: render icon text via Pango with the bundled TTFs.
//!
//! The hybrid set ships `la-{regular-400,solid-900,brands-400}.ttf`:
//! `install_pango` registers all three faces in the Pango font map so
//! `font-family: 'la-solid-900'` (and the regular/brands family names)
//! resolve the FA5 PUA codepoints.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_line_awesome::GLYPH_PREFIX;
use fonts_rs_line_awesome::LineAwesome;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.line_awesome_gtk_gresource";

/// Sample text built from icon codepoints (PUA chars). Solid-style
/// variants render via `la-solid-900` - the default CSS family.
fn sample_text() -> String {
    let prefix = GLYPH_PREFIX;
    ["heart", "star", "bell", "camera-solid"]
        .into_iter()
        .filter_map(|stem| LineAwesome::codepoint(format!("{prefix}-{stem}-symbolic")).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<LineAwesome>(APP_ID, "Line Awesome - Pango Text", &sample_text())
}
