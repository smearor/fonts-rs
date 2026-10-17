//! The `fonts-rs` binary - a multi-family cheat sheet.
//!
//! `cargo install fonts-rs --features gui` installs a single browser
//! window multiplexed over every fonts-rs family via
//! [`fonts_rs_cheat_sheet::run_multi`]; the header dropdown switches
//! families in place. Each family's hooks come from its
//! `*-cheat-sheet` library binding (`XSheet`), bridged to the dyn
//! surface via [`FamilyAdapter`].

#[cfg(feature = "sheet-blobmoji")]
use fonts_rs_blobmoji_cheat_sheet::BlobmojiSheet;
#[cfg(feature = "sheet-bravura")]
use fonts_rs_bravura_cheat_sheet::BravuraSheet;
use fonts_rs_cheat_sheet::DynCheatSheetFamily;
#[allow(unused_imports)] // used only by the `sheet-*` cfg'd pushes
use fonts_rs_cheat_sheet::FamilyAdapter;
use fonts_rs_cheat_sheet::run_multi;
#[cfg(feature = "sheet-cuernavaca")]
use fonts_rs_cuernavaca_cheat_sheet::CuernavacaSheet;
#[cfg(feature = "sheet-dicefont")]
use fonts_rs_dicefont_cheat_sheet::DicefontSheet;
#[cfg(feature = "sheet-doto")]
use fonts_rs_doto_cheat_sheet::DotoSheet;
#[cfg(feature = "sheet-eva")]
use fonts_rs_eva_cheat_sheet::EvaSheet;
#[cfg(feature = "sheet-fluent")]
use fonts_rs_fluent_cheat_sheet::FluentSheet;
#[cfg(feature = "sheet-fourteen-segment")]
use fonts_rs_fourteen_segment_cheat_sheet::FourteenSegmentSheet;
#[cfg(feature = "sheet-heroicons")]
use fonts_rs_heroicons_cheat_sheet::HeroiconsSheet;
#[cfg(feature = "sheet-line-awesome")]
use fonts_rs_line_awesome_cheat_sheet::LineAwesomeSheet;
#[cfg(feature = "sheet-lucide")]
use fonts_rs_lucide_cheat_sheet::LucideSheet;
#[cfg(feature = "sheet-noto-emoji")]
use fonts_rs_noto_emoji_cheat_sheet::NotoEmojiSheet;
#[cfg(feature = "sheet-seven-segment")]
use fonts_rs_seven_segment_cheat_sheet::SevenSegmentSheet;
#[cfg(feature = "sheet-tabler")]
use fonts_rs_tabler_cheat_sheet::TablerSheet;
use gtk4::glib;
use miette::Result;
#[cfg(feature = "sheet-nerd-fonts")]
use nerd_fonts_cheat_sheet::NerdFontsSheet;
use std::boxed::Box;

fn main() -> Result<glib::ExitCode> {
    #[allow(unused_mut)] // pushes are cfg'd per `sheet-*` feature
    let mut families: Vec<Box<dyn DynCheatSheetFamily>> = Vec::new();
    #[cfg(feature = "sheet-blobmoji")]
    families.push(Box::new(FamilyAdapter::<BlobmojiSheet>::new()));
    #[cfg(feature = "sheet-bravura")]
    families.push(Box::new(FamilyAdapter::<BravuraSheet>::new()));
    #[cfg(feature = "sheet-cuernavaca")]
    families.push(Box::new(FamilyAdapter::<CuernavacaSheet>::new()));
    #[cfg(feature = "sheet-dicefont")]
    families.push(Box::new(FamilyAdapter::<DicefontSheet>::new()));
    #[cfg(feature = "sheet-doto")]
    families.push(Box::new(FamilyAdapter::<DotoSheet>::new()));
    #[cfg(feature = "sheet-eva")]
    families.push(Box::new(FamilyAdapter::<EvaSheet>::new()));
    #[cfg(feature = "sheet-fluent")]
    families.push(Box::new(FamilyAdapter::<FluentSheet>::new()));
    #[cfg(feature = "sheet-fourteen-segment")]
    families.push(Box::new(FamilyAdapter::<FourteenSegmentSheet>::new()));
    #[cfg(feature = "sheet-heroicons")]
    families.push(Box::new(FamilyAdapter::<HeroiconsSheet>::new()));
    #[cfg(feature = "sheet-line-awesome")]
    families.push(Box::new(FamilyAdapter::<LineAwesomeSheet>::new()));
    #[cfg(feature = "sheet-lucide")]
    families.push(Box::new(FamilyAdapter::<LucideSheet>::new()));
    #[cfg(feature = "sheet-noto-emoji")]
    families.push(Box::new(FamilyAdapter::<NotoEmojiSheet>::new()));
    #[cfg(feature = "sheet-seven-segment")]
    families.push(Box::new(FamilyAdapter::<SevenSegmentSheet>::new()));
    #[cfg(feature = "sheet-tabler")]
    families.push(Box::new(FamilyAdapter::<TablerSheet>::new()));
    #[cfg(feature = "sheet-nerd-fonts")]
    families.push(Box::new(FamilyAdapter::<NerdFontsSheet>::new()));
    run_multi(families)
}
