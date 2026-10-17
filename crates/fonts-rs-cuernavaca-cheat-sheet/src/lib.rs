//! Chess Cuernavaca Cheat Sheet application.
//!
//! Browse all available Chess Cuernavaca glyphs with a search filter.
//! Displays each glyph with its Unicode codepoint and glyph name.
//! Search covers glyph names, hex codepoints, and characters.
//! Click a glyph to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Cuernavaca hooks via [`CheatSheetFamily`]. With the `metadata`
//! feature (default), pieces are grouped into categories
//! (king/queen/rook/bishop/knight/pawn) for the category sidebar and
//! search, and the four styles (light/dark normal, light/dark
//! upside-down) collapse behind the `light normal` base with variant
//! links in the detail sidebar.
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-cuernavaca-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_cuernavaca::Cuernavaca;
use fonts_rs_cuernavaca::CuernavacaName;
#[cfg(feature = "metadata")]
use fonts_rs_cuernavaca::metadata::CuernavacaMetadata;
use fonts_rs_model::CodePoint;
#[cfg(feature = "metadata")]
use fonts_rs_model::GlyphMetadata;
use miette::IntoDiagnostic;
use miette::Result;

/// Chess Cuernavaca hooks for the generic cheat sheet.
pub struct CuernavacaSheet;

impl CheatSheetFamily for CuernavacaSheet {
    const TITLE: &'static str = "Chess Cuernavaca - Cheat Sheet";
    const NOUN: &'static str = "glyphs";
    const APP_ID: &'static str = "io.smearor.fonts_rs.cuernavaca_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name or category (e.g. 'cuernavaca-k', 'knight')...";
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

    /// Metadata-backed search over names and piece categories when
    /// the `metadata` feature is on; the generic supplement still
    /// matches names, hex codepoints, and characters.
    fn search(query: &str) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            CuernavacaMetadata.search(query).iter().map(|s| s.to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = query;
            Vec::new()
        }
    }

    /// Cuernavaca has no keywords.
    fn keywords(_name: &CuernavacaName) -> Vec<String> {
        Vec::new()
    }

    /// Piece categories (king/queen/rook/bishop/knight/pawn) come
    /// from the `metadata` feature - derived from the glyph names at
    /// build time since Cuernavaca ships no upstream category source.
    fn categories(name: &CuernavacaName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            CuernavacaMetadata.categories(name).iter().map(|c| c.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    /// Style groups and piece variants: `light normal` (white,
    /// upright) is the base, dark and upside-down styles link onto it.
    fn ordering() -> Option<&'static dyn fonts_rs_model::GlyphOrdering> {
        #[cfg(feature = "metadata")]
        {
            Some(&CuernavacaMetadata)
        }
        #[cfg(not(feature = "metadata"))]
        {
            None
        }
    }

    fn glyph_svg(name: &CuernavacaName) -> Option<String> {
        Cuernavaca::glyph_svg(name)
    }
}

