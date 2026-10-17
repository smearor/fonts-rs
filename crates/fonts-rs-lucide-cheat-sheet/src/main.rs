//! Lucide Icons cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_lucide_cheat_sheet::LucideSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<LucideSheet>()
}
