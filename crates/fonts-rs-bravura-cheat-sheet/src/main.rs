//! Bravura Cheat Sheet application.
//!
//! Browse all available Bravura SMuFL music notation glyphs with a
//! search filter. Displays each glyph with its Unicode codepoint and
//! glyph name. Search covers glyph names, hex codepoints, and
//! characters. Click a glyph to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Bravura hooks via [`CheatSheetFamily`].
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-bravura-cheat-sheet
//! ```

use fonts_rs_bravura::Bravura;
use fonts_rs_bravura::BravuraName;
use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_model::CodePoint;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

/// Bravura SMuFL hooks for the generic cheat sheet.
struct BravuraSheet;

impl CheatSheetFamily for BravuraSheet {
    const TITLE: &'static str = "Bravura - Cheat Sheet";
    const NOUN: &'static str = "glyphs";
    const APP_ID: &'static str = "io.smearor.fonts_rs.bravura_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by glyph name (e.g. 'bravura-g-clef', 'bravura-notehead-black')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("bravura-icon");
    const FAMILY_NAME: &'static str = "Bravura";
    const FAMILY_KIND: FamilyKind = FamilyKind::SpecialFont;
    const LICENSE: &'static str = "OFL-1.1";

    type Name = BravuraName;

    fn init() -> Result<()> {
        Bravura::install().into_diagnostic()?;
        // Register the bundled OTF with the global Pango font map -
        // without this, `font-family: 'Bravura'` falls back to any
        // installed font covering the SMuFL PUA codepoints.
        Bravura::install_pango(&pangocairo::FontMap::default()).into_diagnostic()?;
        fonts_rs_model::gtk::load_css(&fonts_rs_model::icon_css(&[("bravura-icon", "Bravura")]));
        Ok(())
    }

    fn all_glyphs() -> Vec<(CodePoint, BravuraName)> {
        Bravura::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        Bravura::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<BravuraName> {
        // Round-trip via the codepoint resolves shorthand input to the
        // canonical glyph name and rejects unknown names.
        Bravura::codepoint(name).and_then(Bravura::glyph_name)
    }

    fn display_name(name: &BravuraName) -> String {
        name.to_string()
    }

    fn subtitle(_name: &BravuraName) -> String {
        "Bravura".to_string()
    }

    /// No metadata module - the generic supplement still matches
    /// names, hex codepoints, and characters.
    fn search(_query: &str) -> Vec<String> {
        Vec::new()
    }

    fn keywords(_name: &BravuraName) -> Vec<String> {
        Vec::new()
    }

    fn categories(_name: &BravuraName) -> Vec<String> {
        Vec::new()
    }

    fn glyph_svg(name: &BravuraName) -> Option<String> {
        Bravura::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<BravuraSheet>()
}
