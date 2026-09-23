//! Minimal GTK example: use GtkIconTheme to look up DSEG14 glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_fourteen_segment::FourteenSegment;
use fonts_rs_fourteen_segment::constants;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [
        (constants::DSEG14_CLASSIC_REGULAR_ZERO, "0"),
        (constants::DSEG14_CLASSIC_REGULAR_ONE, "1"),
        (constants::DSEG14_CLASSIC_REGULAR_A, "A"),
        (constants::DSEG14_CLASSIC_REGULAR_B, "B"),
    ];

    run_gtk_icon_theme::<FourteenSegment>("io.smearor.fonts_rs.fourteen_segment_icon_theme", "DSEG14 — GtkIconTheme", &icons)
}
