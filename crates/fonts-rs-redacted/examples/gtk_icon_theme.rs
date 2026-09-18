//! Minimal GTK example: use GtkIconTheme to look up Redacted glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_redacted::RedactedConfig;
use fonts_rs_redacted::constants;
use fonts_rs_redacted::naming::Redacted;
use fonts_rs_redacted::variant;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [
        (constants::REDACTED_REDACTED_H, "H"),
        (constants::REDACTED_REDACTED_E, "E"),
        (constants::REDACTED_REDACTED_L, "L"),
        (constants::REDACTED_REDACTED_O, "O"),
    ];

    run_gtk_icon_theme::<Redacted, RedactedConfig>(
        "io.smearor.fonts_rs.redacted_icon_theme",
        "Redacted — GtkIconTheme",
        variant::GRESOURCE_PREFIX,
        &icons,
    )
}
