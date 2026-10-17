//! Noto Emoji cheat sheet application binary.

use fonts_rs_cheat_sheet::run;
use fonts_rs_noto_emoji_cheat_sheet::NotoEmojiSheet;
use gtk4::glib;
use miette::Result;

fn main() -> Result<glib::ExitCode> {
    run::<NotoEmojiSheet>()
}
