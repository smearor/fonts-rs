//! DSEG14 Fourteen-Segment Cheat Sheet application.
//!
//! Browse all available DSEG14 fourteen-segment display glyphs with a
//! search filter. Displays each glyph with its Unicode codepoint and
//! glyph name. Search covers glyph names, hex codepoints, and
//! characters. Click a glyph to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the FourteenSegment hooks via [`CheatSheetFamily`].
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-fourteen-segment-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_fourteen_segment::FourteenSegment;
use fonts_rs_fourteen_segment::FourteenSegmentName;
use fonts_rs_model::CodePoint;
use miette::IntoDiagnostic;
use miette::Result;

/// DSEG14 hooks for the generic cheat sheet.
pub struct FourteenSegmentSheet;

impl CheatSheetFamily for FourteenSegmentSheet {
    const TITLE: &'static str = "DSEG14 Fourteen-Segment - Cheat Sheet";
    const NOUN: &'static str = "glyphs";
    const APP_ID: &'static str = "io.smearor.fonts_rs.fourteen_segment_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by glyph name (e.g. 'dseg14-zero', 'dseg14-a')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("dseg14-icon");
    const FAMILY_NAME: &'static str = "DSEG14";
    const FAMILY_KIND: FamilyKind = FamilyKind::SpecialFont;
    const LICENSE: &'static str = "OFL-1.1";

    type Name = FourteenSegmentName;

    fn init() -> Result<()> {
        FourteenSegment::install().into_diagnostic()?;
        // Register the bundled TTF with the global Pango font map -
        // without this, `font-family: 'DSEG14 Classic'` falls back to the
        // system font covering the ASCII codepoints.
        FourteenSegment::install_pango(&pangocairo::FontMap::default()).into_diagnostic()?;
        fonts_rs_model::gtk::load_css(&fonts_rs_model::icon_css(&[("dseg14-icon", "DSEG14 Classic")]));
        Ok(())
    }

    fn all_glyphs() -> Vec<(CodePoint, FourteenSegmentName)> {
        FourteenSegment::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        FourteenSegment::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<FourteenSegmentName> {
        // Round-trip via the codepoint resolves shorthand input to the
        // canonical glyph name and rejects unknown names.
        FourteenSegment::codepoint(name).and_then(FourteenSegment::glyph_name)
    }

    fn display_name(name: &FourteenSegmentName) -> String {
        name.to_string()
    }

    fn subtitle(_name: &FourteenSegmentName) -> String {
        "DSEG14".to_string()
    }

    /// No metadata module - the generic supplement still matches
    /// names, hex codepoints, and characters.
    fn search(_query: &str) -> Vec<String> {
        Vec::new()
    }

    fn keywords(_name: &FourteenSegmentName) -> Vec<String> {
        Vec::new()
    }

    fn categories(_name: &FourteenSegmentName) -> Vec<String> {
        Vec::new()
    }

    fn glyph_svg(name: &FourteenSegmentName) -> Option<String> {
        FourteenSegment::glyph_svg(name)
    }
}
