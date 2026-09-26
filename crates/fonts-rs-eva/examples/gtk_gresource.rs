//! Minimal GTK example: load font from GResource and render icon text.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_gresource --features gtk
//! ```

use fonts_rs_eva::Eva;
use fonts_rs_test::run_gtk_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.eva_gresource";

/// Builds the sample text from icon codepoints resolved at runtime —
/// codepoints differ between the fill and outline variants.
fn sample_text() -> String {
    ["eva-home-symbolic", "eva-heart-symbolic", "eva-star-symbolic", "eva-bell-symbolic"]
        .into_iter()
        .filter_map(|name| Eva::codepoint(name).map(|cp| cp.as_char()))
        .collect()
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    run_gtk_gresource::<Eva>(APP_ID, "Eva Icons — GTK GResource", &sample_text())
}
