//! Minimal GTK example: use GtkIconTheme to look up Nerd Font glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_test::run_gtk_icon_theme;
use nerd_fonts_rs::NerdFonts;
use nerd_fonts_rs::icons::constants;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [
        (constants::NF_FA_GAMEPAD_SYMBOLIC, "Gamepad"),
        (constants::NF_FA_HEART_SYMBOLIC, "Heart"),
        (constants::NF_FA_STAR_SYMBOLIC, "Star"),
        (constants::NF_FA_FIRE_SYMBOLIC, "Fire"),
    ];

    run_gtk_icon_theme::<NerdFonts>("io.smearor.fonts_rs.nerd_fonts_icon_theme", "Nerd Fonts - GtkIconTheme", &icons)
}
