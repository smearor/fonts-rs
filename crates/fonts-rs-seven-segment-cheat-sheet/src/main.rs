//! Seven Segment cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_seven_segment_cheat_sheet::SevenSegmentSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<SevenSegmentSheet>()
}
