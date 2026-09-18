//! Minimal GTK example: use GtkIconTheme to look up DSEG7 glyph SVGs by icon name.
//!
//! Run with:
//! ```sh
//! cargo run --example gtk_icon_theme --features gtk
//! ```

use fonts_rs_seven_segment::SevenSegmentConfig;
use fonts_rs_seven_segment::constants;
use fonts_rs_seven_segment::naming::SevenSegment;
use fonts_rs_seven_segment::variant;
use fonts_rs_test::run_gtk_icon_theme;

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let icons = [
        (constants::DSEG7_CLASSIC_REGULAR_ZERO, "0"),
        (constants::DSEG7_CLASSIC_REGULAR_ONE, "1"),
        (constants::DSEG7_CLASSIC_REGULAR_TWO, "2"),
        (constants::DSEG7_CLASSIC_REGULAR_THREE, "3"),
    ];

    run_gtk_icon_theme::<SevenSegment, SevenSegmentConfig>(
        "io.smearor.fonts_rs.seven_segment_icon_theme",
        "DSEG7 — GtkIconTheme",
        variant::GRESOURCE_PREFIX,
        &icons,
    )
}
