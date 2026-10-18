//! Nerd Font Cheat Sheet application.
//!
//! Browse all available Nerd Font icons with a search filter.
//! Displays each icon with its Unicode codepoint, CSS class name,
//! keywords, and categories. Search covers icon names, keywords,
//! categories, and aliases. Click an icon to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Nerd Fonts hooks via [`CheatSheetFamily`].
//!
//! Run with:
//! ```sh
//! cargo run -p nerd-fonts-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_model::GlyphMetadata;
use miette::IntoDiagnostic;
use miette::Result;
use nerd_fonts_model::CodePoint;
use nerd_fonts_model::IconName;
use nerd_fonts_rs::NerdFonts;
use nerd_fonts_rs::icons::all_icons_typed;
use nerd_fonts_rs::metadata::NerdFontsMetadata;

/// Nerd Fonts hooks for the generic cheat sheet.
pub struct NerdFontsSheet;

impl CheatSheetFamily for NerdFontsSheet {
    const TITLE: &'static str = "Nerd Fonts - Cheat Sheet";
    const NOUN: &'static str = "icons";
    const APP_ID: &'static str = "io.smearor.fonts_rs.cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name, keyword, or category (e.g. 'gamepad', 'folder', 'Childhood')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("nerd-icon");
    const NAME_FIELD_LABEL: &'static str = "CSS Class";
    const FAMILY_NAME: &'static str = "Nerd Fonts";
    const FAMILY_KIND: FamilyKind = FamilyKind::IconFont;
    const LICENSE: &'static str = "MIT / OFL-1.1";

    type Name = IconName;

    fn init() -> Result<()> {
        nerd_fonts_rs::init().into_diagnostic()
    }

    fn all_glyphs() -> Vec<(CodePoint, IconName)> {
        all_icons_typed()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        NerdFonts::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<IconName> {
        IconName::parse(name)
    }

    fn display_name(name: &IconName) -> String {
        name.css_class_name().to_string()
    }

    fn subtitle(name: &IconName) -> String {
        name.icon_set().label().to_string()
    }

    fn search(query: &str) -> Vec<String> {
        NerdFontsMetadata.search(query).iter().map(|s| s.to_string()).collect()
    }

    fn keywords(name: &IconName) -> Vec<String> {
        NerdFontsMetadata.keywords(name).iter().map(|k| k.as_str().to_string()).collect()
    }

    fn categories(name: &IconName) -> Vec<String> {
        NerdFontsMetadata.categories(name).iter().map(|c| c.as_str().to_string()).collect()
    }

    /// Icon-set groups (`nf-fa-*`, `nf-md-*`, …) power the sidebar's
    /// "Groups" section, the jump-list dropdown, and group-major search
    /// ordering.
    fn ordering() -> Option<&'static dyn fonts_rs_model::GlyphOrdering> {
        Some(&NerdFontsMetadata)
    }

    fn alias_display(name: &IconName) -> Option<String> {
        let canonical = NerdFontsMetadata.resolve_alias(name.as_ref())?;
        Some(
            IconName::parse(canonical)
                .map(|n| n.css_class_name().to_string())
                .unwrap_or_else(|| canonical.to_string()),
        )
    }

    fn glyph_svg(name: &IconName) -> Option<String> {
        NerdFonts::glyph_svg(name)
    }
}
