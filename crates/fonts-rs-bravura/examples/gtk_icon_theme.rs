//! GtkIconTheme example for Bravura.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_bravura::BravuraConfig;
use fonts_rs_bravura::naming::Bravura;
use fonts_rs_bravura::constants;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [
        (constants::BRAVURA_GCLEF, "g-clef"),
        (constants::BRAVURA_NOTEHEADBLACK, "notehead-black"),
        (constants::BRAVURA_TIMESIG3, "time-sig-3"),
        (constants::BRAVURA_TIMESIG4, "time-sig-4"),
    ];

    run_gtk_icon_theme::<Bravura, BravuraConfig>(
        "io.smearor.fonts_rs.bravura_icon_theme",
        "Bravura — GtkIconTheme",
        BravuraConfig::GRESOURCE_PREFIX,
        &icons,
    )
}
