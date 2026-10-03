//! Dicefont Cheat Sheet application.
//!
//! Browse all available Dicefont die face glyphs with a search filter.
//! Displays each glyph with its Unicode codepoint and glyph name.
//! Search covers glyph names, hex codepoints, and characters.
//! Click a glyph to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Dicefont hooks via [`CheatSheetFamily`].
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-dicefont-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_dicefont::Dicefont;
use fonts_rs_dicefont::DicefontName;
use fonts_rs_model::CodePoint;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

/// Dicefont hooks for the generic cheat sheet.
struct DicefontSheet;

impl CheatSheetFamily for DicefontSheet {
    const TITLE: &'static str = "Dicefont - Cheat Sheet";
    const NOUN: &'static str = "glyphs";
    const APP_ID: &'static str = "io.smearor.fonts_rs.dicefont_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by glyph name (e.g. 'dicefont-d6-1', 'dicefont-d20-20')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("dicefont-icon");
    const FAMILY_NAME: &'static str = "Dicefont";
    const FAMILY_KIND: FamilyKind = FamilyKind::SpecialFont;
    const LICENSE: &'static str = "OFL-1.1";

    type Name = DicefontName;

    fn init() -> Result<()> {
        Dicefont::install().into_diagnostic()?;
        // Register the bundled TTF with the global Pango font map -
        // without this, `font-family: 'dicefont'` falls back to any
        // installed font covering the PUA codepoints (e.g. Nerd Fonts).
        Dicefont::install_pango(&pangocairo::FontMap::default()).into_diagnostic()?;
        fonts_rs_model::gtk::load_css(&fonts_rs_model::icon_css(&[("dicefont-icon", "dicefont")]));
        Ok(())
    }

    fn all_glyphs() -> Vec<(CodePoint, DicefontName)> {
        Dicefont::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        Dicefont::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<DicefontName> {
        // Round-trip via the codepoint resolves shorthand input to the
        // canonical glyph name and rejects unknown names.
        Dicefont::codepoint(name).and_then(Dicefont::glyph_name)
    }

    fn display_name(name: &DicefontName) -> String {
        name.to_string()
    }

    fn subtitle(_name: &DicefontName) -> String {
        "Dicefont".to_string()
    }

    /// No metadata module - the generic supplement still matches
    /// names, hex codepoints, and characters.
    fn search(_query: &str) -> Vec<String> {
        Vec::new()
    }

    fn keywords(_name: &DicefontName) -> Vec<String> {
        Vec::new()
    }

    fn categories(_name: &DicefontName) -> Vec<String> {
        Vec::new()
    }

    fn glyph_svg(name: &DicefontName) -> Option<String> {
        Dicefont::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<DicefontSheet>()
}
