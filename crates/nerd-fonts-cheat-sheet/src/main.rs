//! Nerd Fonts cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use gtk4::glib;
use miette::Result;
use nerd_fonts_cheat_sheet::NerdFontsSheet;

fn main() -> Result<glib::ExitCode> {
    run::<NerdFontsSheet>()
}
