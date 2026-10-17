//! Heroicons cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_heroicons_cheat_sheet::HeroiconsSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<HeroiconsSheet>()
}
