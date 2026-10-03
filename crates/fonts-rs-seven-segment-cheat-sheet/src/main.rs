//! DSEG7 Seven-Segment Cheat Sheet application.
//!
//! Browse all available DSEG7 seven-segment display glyphs with a
//! search filter. Displays each glyph with its Unicode codepoint and
//! glyph name. Search covers glyph names, hex codepoints, and
//! characters. Click a glyph to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the SevenSegment hooks via [`CheatSheetFamily`].
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-seven-segment-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_model::CodePoint;
use fonts_rs_seven_segment::SevenSegment;
use fonts_rs_seven_segment::SevenSegmentName;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

/// DSEG7 hooks for the generic cheat sheet.
struct SevenSegmentSheet;

impl CheatSheetFamily for SevenSegmentSheet {
    const TITLE: &'static str = "DSEG7 Seven-Segment - Cheat Sheet";
    const NOUN: &'static str = "glyphs";
    const APP_ID: &'static str = "io.smearor.fonts_rs.seven_segment_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by glyph name (e.g. 'dseg7-zero', 'dseg7-colon')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("dseg7-icon");
    const FAMILY_NAME: &'static str = "DSEG7";
    const FAMILY_KIND: FamilyKind = FamilyKind::SpecialFont;
    const LICENSE: &'static str = "OFL-1.1";

    type Name = SevenSegmentName;

    fn init() -> Result<()> {
        SevenSegment::install().into_diagnostic()?;
        // Register the bundled TTF with the global Pango font map -
        // without this, `font-family: 'DSEG7 Classic'` falls back to the
        // system font covering the ASCII codepoints.
        SevenSegment::install_pango(&pangocairo::FontMap::default()).into_diagnostic()?;
        fonts_rs_model::gtk::load_css(&fonts_rs_model::icon_css(&[("dseg7-icon", "DSEG7 Classic")]));
        Ok(())
    }

    fn all_glyphs() -> Vec<(CodePoint, SevenSegmentName)> {
        SevenSegment::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        SevenSegment::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<SevenSegmentName> {
        // Round-trip via the codepoint resolves shorthand input to the
        // canonical glyph name and rejects unknown names.
        SevenSegment::codepoint(name).and_then(SevenSegment::glyph_name)
    }

    fn display_name(name: &SevenSegmentName) -> String {
        name.to_string()
    }

    fn subtitle(_name: &SevenSegmentName) -> String {
        "DSEG7".to_string()
    }

    /// No metadata module - the generic supplement still matches
    /// names, hex codepoints, and characters.
    fn search(_query: &str) -> Vec<String> {
        Vec::new()
    }

    fn keywords(_name: &SevenSegmentName) -> Vec<String> {
        Vec::new()
    }

    fn categories(_name: &SevenSegmentName) -> Vec<String> {
        Vec::new()
    }

    fn glyph_svg(name: &SevenSegmentName) -> Option<String> {
        SevenSegment::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<SevenSegmentSheet>()
}
