//! Cuernavaca cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_cuernavaca_cheat_sheet::CuernavacaSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<CuernavacaSheet>()
}
