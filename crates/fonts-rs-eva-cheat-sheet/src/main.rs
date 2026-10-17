//! Eva Icons cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_eva_cheat_sheet::EvaSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<EvaSheet>()
}
