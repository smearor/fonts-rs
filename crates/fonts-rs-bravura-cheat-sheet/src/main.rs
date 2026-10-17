//! Bravura cheat sheet application binary.

use fonts_rs_bravura_cheat_sheet::BravuraSheet;
use fonts_rs_cheat_sheet::run;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<BravuraSheet>()
}
