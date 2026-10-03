//! Chess Cuernavaca Cheat Sheet application.
//!
//! Browse all available Chess Cuernavaca glyphs with a search filter.
//! Displays each glyph with its Unicode codepoint and glyph name.
//! Search covers glyph names, hex codepoints, and characters.
//! Click a glyph to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Cuernavaca hooks via [`CheatSheetFamily`].
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-cuernavaca-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_cuernavaca::Cuernavaca;
use fonts_rs_cuernavaca::CuernavacaName;
use fonts_rs_model::CodePoint;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

/// Chess Cuernavaca hooks for the generic cheat sheet.
struct CuernavacaSheet;

impl CheatSheetFamily for CuernavacaSheet {
    const TITLE: &'static str = "Chess Cuernavaca - Cheat Sheet";
    const NOUN: &'static str = "glyphs";
    const APP_ID: &'static str = "io.smearor.fonts_rs.cuernavaca_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by glyph name (e.g. 'cuernavaca-k', 'cuernavaca-q')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("cuernavaca-icon");
    const FAMILY_NAME: &'static str = "Chess Cuernavaca";
    const FAMILY_KIND: FamilyKind = FamilyKind::SpecialFont;
    const LICENSE: &'static str = "OFL-1.1";

    type Name = CuernavacaName;

    fn init() -> Result<()> {
        Cuernavaca::install().into_diagnostic()?;
        // Register the bundled TTF with the global Pango font map -
        // without this, `font-family: 'ChessCuernavaca'` falls back to
        // any installed font covering the codepoints.
        Cuernavaca::install_pango(&pangocairo::FontMap::default()).into_diagnostic()?;
        fonts_rs_model::gtk::load_css(&fonts_rs_model::icon_css(&[("cuernavaca-icon", "ChessCuernavaca")]));
        Ok(())
    }

    fn all_glyphs() -> Vec<(CodePoint, CuernavacaName)> {
        Cuernavaca::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        Cuernavaca::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<CuernavacaName> {
        // Round-trip via the codepoint resolves shorthand input (e.g.
        // "k") to the canonical glyph name and rejects unknown names.
        Cuernavaca::codepoint(name).and_then(Cuernavaca::glyph_name)
    }

    fn display_name(name: &CuernavacaName) -> String {
        name.to_string()
    }

    fn subtitle(_name: &CuernavacaName) -> String {
        "Chess Cuernavaca".to_string()
    }

    /// No metadata module - the generic supplement still matches
    /// names, hex codepoints, and characters.
    fn search(_query: &str) -> Vec<String> {
        Vec::new()
    }

    fn keywords(_name: &CuernavacaName) -> Vec<String> {
        Vec::new()
    }

    fn categories(_name: &CuernavacaName) -> Vec<String> {
        Vec::new()
    }

    fn glyph_svg(name: &CuernavacaName) -> Option<String> {
        Cuernavaca::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<CuernavacaSheet>()
}
