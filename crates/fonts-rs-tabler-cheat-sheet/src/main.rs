//! Tabler Icons cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_tabler_cheat_sheet::TablerSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<TablerSheet>()
}
