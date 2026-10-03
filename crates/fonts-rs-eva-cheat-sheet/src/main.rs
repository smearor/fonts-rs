//! Eva Icons Cheat Sheet application.
//!
//! Browse all available Eva icons with a search filter.
//! Displays each icon with its Unicode codepoint and icon name.
//! Click an icon to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Eva hooks via [`CheatSheetFamily`]. With the `metadata` feature
//! (default), search also covers upstream keywords from
//! `icons-tags.json` - e.g. "alarm" finds `eva-bell-symbolic`.
//! Eva ships no categories or aliases, so those stay empty.
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-eva-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_eva::Eva;
use fonts_rs_eva::EvaIconName;
#[cfg(feature = "metadata")]
use fonts_rs_eva::metadata::EvaMetadata;
use fonts_rs_model::CodePoint;
#[cfg(feature = "metadata")]
use fonts_rs_model::GlyphMetadata;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

/// Eva Icons hooks for the generic cheat sheet.
struct EvaSheet;

impl CheatSheetFamily for EvaSheet {
    const TITLE: &'static str = "Eva Icons - Cheat Sheet";
    const NOUN: &'static str = "icons";
    const APP_ID: &'static str = "io.smearor.fonts_rs.eva_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name, keyword, or codepoint (e.g. 'home', 'alarm', 'EA01')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("eva-icon");
    const NAME_FIELD_LABEL: &'static str = "Icon Name";
    const FAMILY_NAME: &'static str = "Eva Icons";
    const FAMILY_KIND: FamilyKind = FamilyKind::IconSet;
    const LICENSE: &'static str = "MIT";

    type Name = EvaIconName;

    fn init() -> Result<()> {
        Eva::install().into_diagnostic()?;
        // Register the bundled TTF with the global Pango font map -
        // without this, `font-family: 'Eva-Icons'` falls back to any
        // installed font covering the PUA codepoints (e.g. Nerd Fonts).
        Eva::install_pango(&pangocairo::FontMap::default()).into_diagnostic()?;
        fonts_rs_model::gtk::load_css(&fonts_rs_model::icon_css(&[("eva-icon", "Eva-Icons")]));
        Ok(())
    }

    fn all_glyphs() -> Vec<(CodePoint, EvaIconName)> {
        Eva::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        Eva::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<EvaIconName> {
        EvaIconName::parse(name)
    }

    fn display_name(name: &EvaIconName) -> String {
        name.css_class_name().to_string()
    }

    fn subtitle(_name: &EvaIconName) -> String {
        "Eva Icons".to_string()
    }

    fn search(query: &str) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            EvaMetadata.search(query).iter().map(|s| s.to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = query;
            Vec::new()
        }
    }

    fn keywords(name: &EvaIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            EvaMetadata.keywords(name).iter().map(|k| k.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    fn categories(name: &EvaIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            EvaMetadata.categories(name).iter().map(|c| c.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    /// Fill/outline style groups and fill->outline variant collapsing.
    fn ordering() -> Option<&'static dyn fonts_rs_model::GlyphOrdering> {
        #[cfg(feature = "metadata")]
        {
            Some(&EvaMetadata)
        }
        #[cfg(not(feature = "metadata"))]
        {
            None
        }
    }

    fn glyph_svg(name: &EvaIconName) -> Option<String> {
        Eva::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<EvaSheet>()
}
