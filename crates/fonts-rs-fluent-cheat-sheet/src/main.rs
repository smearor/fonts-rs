//! Fluent Icons cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_fluent_cheat_sheet::FluentSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<FluentSheet>()
}
