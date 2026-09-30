//! Minimal GTK example: render icon text via Pango with the bundled TTF.
//!
//! The hybrid set ships `FluentSystemIcons-{Regular,Filled}.ttf`:
//! `install_pango` registers it in the Pango font map so
//! `font-family: 'FluentSystemIcons-*'` resolves the PUA codepoints.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_fluent::Fluent;
use fonts_rs_fluent::variant;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.fluent_gtk_gresource";

/// Sample text built from icon codepoints (PUA chars).
fn sample_text() -> String {
    let prefix = variant::GLYPH_PREFIX;
    ["home", "heart", "star", "alert"]
        .into_iter()
        .filter_map(|stem| Fluent::codepoint(format!("{prefix}-{stem}-symbolic")).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Fluent>(APP_ID, "Fluent UI System Icons - Pango Text", &sample_text())
}
