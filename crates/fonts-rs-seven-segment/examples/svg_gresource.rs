//! Minimal GTK example: load SVG glyphs from GResource and render as textures.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_gresource --features gtk
//! ```

use fonts_rs_seven_segment::SevenSegmentConfig;
use fonts_rs_seven_segment::all_glyphs;
use fonts_rs_seven_segment::naming::SevenSegment;
use fonts_rs_seven_segment::variant;
use fonts_rs_test::run_svg_gresource;

const APP_ID: &str = "io.smearor.fonts_rs.seven_segment_svg_gresource";

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    let glyphs: Vec<(char, &str)> = all_glyphs().take(4).collect();

    run_svg_gresource::<SevenSegment, SevenSegmentConfig>(APP_ID, "DSEG7 — SVG GResource", variant::GRESOURCE_PREFIX, &glyphs)
}
