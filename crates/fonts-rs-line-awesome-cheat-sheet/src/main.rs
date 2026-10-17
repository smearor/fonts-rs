//! Line Awesome cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_line_awesome_cheat_sheet::LineAwesomeSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<LineAwesomeSheet>()
}
