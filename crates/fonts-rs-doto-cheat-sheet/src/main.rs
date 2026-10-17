//! Doto cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_doto_cheat_sheet::DotoSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<DotoSheet>()
}
