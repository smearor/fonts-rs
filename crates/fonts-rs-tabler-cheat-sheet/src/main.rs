//! Tabler Icons Cheat Sheet application.
//!
//! Browse all available Tabler icons with a search filter.
//! Displays each icon with its Unicode codepoint, icon name,
//! keywords, and categories. Search covers icon names, keywords,
//! and categories. Click an icon to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Tabler hooks via [`CheatSheetFamily`].
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-tabler-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_model::CodePoint;
use fonts_rs_tabler::Tabler;
use fonts_rs_tabler::TablerIconName;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

#[cfg(feature = "metadata")]
use fonts_rs_model::GlyphMetadata;
#[cfg(feature = "metadata")]
use fonts_rs_tabler::metadata::TablerMetadata;

/// Tabler Icons hooks for the generic cheat sheet.
struct TablerSheet;

impl CheatSheetFamily for TablerSheet {
    const TITLE: &'static str = "Tabler Icons - Cheat Sheet";
    const NOUN: &'static str = "icons";
    const APP_ID: &'static str = "io.smearor.fonts_rs.tabler_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name, keyword, or category (e.g. 'home', 'arrow', 'Development')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("tabler-icon");
    const NAME_FIELD_LABEL: &'static str = "Icon Name";

    type Name = TablerIconName;

    fn init() -> Result<()> {
        Tabler::install().into_diagnostic()?;
        // Register the bundled TTF with the global Pango font map -
        // without this, `font-family: 'tabler-icons'` falls back to any
        // installed font covering the PUA codepoints (e.g. Nerd Fonts).
        Tabler::install_pango(&pangocairo::FontMap::default()).into_diagnostic()?;
        fonts_rs_model::gtk::load_css(&fonts_rs_model::icon_css(&[("tabler-icon", "tabler-icons")]));
        Ok(())
    }

    fn all_glyphs() -> Vec<(CodePoint, TablerIconName)> {
        Tabler::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        Tabler::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<TablerIconName> {
        TablerIconName::parse(name)
    }

    fn display_name(name: &TablerIconName) -> String {
        name.css_class_name().to_string()
    }

    fn subtitle(_name: &TablerIconName) -> String {
        "Tabler Icons".to_string()
    }

    fn search(query: &str) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            TablerMetadata.search(query).iter().map(|s| s.to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = query;
            Vec::new()
        }
    }

    fn keywords(name: &TablerIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            TablerMetadata.keywords(name).iter().map(|k| k.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    fn categories(name: &TablerIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            TablerMetadata.categories(name).iter().map(|c| c.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    fn alias_display(name: &TablerIconName) -> Option<String> {
        #[cfg(feature = "metadata")]
        {
            let canonical = TablerMetadata.resolve_alias(name.as_ref())?;
            Some(
                TablerIconName::parse(canonical)
                    .map(|n| n.css_class_name().to_string())
                    .unwrap_or_else(|| canonical.to_string()),
            )
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            None
        }
    }

    fn glyph_svg(name: &TablerIconName) -> Option<String> {
        Tabler::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<TablerSheet>()
}
