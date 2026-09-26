//! Minimal GTK example: load font from GResource and render icon text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_tabler::Tabler;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.tabler_gresource";

/// Builds the sample text from icon codepoints resolved at runtime —
/// codepoints differ between the outline and filled variants.
fn sample_text() -> String {
    ["tabler-home-symbolic", "tabler-heart-symbolic", "tabler-star-symbolic", "tabler-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Tabler::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Tabler>(APP_ID, "Tabler Icons — GTK GResource", &sample_text())
}
