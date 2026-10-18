//! Doto Cheat Sheet application.
//!
//! Browse all available Doto dot-matrix glyphs with a search filter.
//! Displays each glyph with its Unicode codepoint and glyph name.
//! Search covers glyph names, hex codepoints, and characters.
//! Click a glyph to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Doto hooks via [`CheatSheetFamily`].
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-doto-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_doto::Doto;
use fonts_rs_doto::DotoName;
use fonts_rs_model::CodePoint;
use miette::IntoDiagnostic;
use miette::Result;

/// Doto hooks for the generic cheat sheet.
pub struct DotoSheet;

impl CheatSheetFamily for DotoSheet {
    const TITLE: &'static str = "Doto - Cheat Sheet";
    const NOUN: &'static str = "glyphs";
    const APP_ID: &'static str = "io.smearor.fonts_rs.doto_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by glyph name (e.g. 'doto-a', 'doto-zero')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("doto-icon");
    const FAMILY_NAME: &'static str = "Doto";
    const FAMILY_KIND: FamilyKind = FamilyKind::Font;
    const LICENSE: &'static str = "OFL-1.1";

    type Name = DotoName;

    fn init() -> Result<()> {
        Doto::install().into_diagnostic()?;
        // Register the bundled TTF with the global Pango font map -
        // without this, `font-family: 'Doto'` falls back to the system
        // font covering the ASCII codepoints.
        Doto::install_pango(&pangocairo::FontMap::default()).into_diagnostic()?;
        fonts_rs_model::gtk::load_css(&fonts_rs_model::icon_css(&[("doto-icon", "Doto")]));
        Ok(())
    }

    fn all_glyphs() -> Vec<(CodePoint, DotoName)> {
        Doto::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        Doto::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<DotoName> {
        // Round-trip via the codepoint resolves shorthand input to the
        // canonical glyph name and rejects unknown names.
        Doto::codepoint(name).and_then(Doto::glyph_name)
    }

    fn display_name(name: &DotoName) -> String {
        name.to_string()
    }

    fn subtitle(_name: &DotoName) -> String {
        "Doto".to_string()
    }

    /// No metadata module - the generic supplement still matches
    /// names, hex codepoints, and characters.
    fn search(_query: &str) -> Vec<String> {
        Vec::new()
    }

    fn keywords(_name: &DotoName) -> Vec<String> {
        Vec::new()
    }

    fn categories(_name: &DotoName) -> Vec<String> {
        Vec::new()
    }

    fn glyph_svg(name: &DotoName) -> Option<String> {
        Doto::glyph_svg(name)
    }
}
