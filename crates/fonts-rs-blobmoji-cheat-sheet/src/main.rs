//! Blobmoji cheat sheet application binary.

use fonts_rs_blobmoji_cheat_sheet::BlobmojiSheet;
use fonts_rs_cheat_sheet::run;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<BlobmojiSheet>()
}
