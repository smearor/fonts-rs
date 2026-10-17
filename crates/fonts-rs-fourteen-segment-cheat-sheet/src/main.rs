//! Fourteen Segment cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_fourteen_segment_cheat_sheet::FourteenSegmentSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<FourteenSegmentSheet>()
}
