//! Dicefont cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_dicefont_cheat_sheet::DicefontSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<DicefontSheet>()
}
