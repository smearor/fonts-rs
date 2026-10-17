//! Heroicons Cheat Sheet application.
//!
//! Browse all available Heroicons icons with a search filter.
//! Displays each icon with its Unicode codepoint and icon name.
//! Click an icon to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Heroicons hooks via [`CheatSheetFamily`]. With the `metadata`
//! feature (default), search also covers the heroicons.com keywords -
//! e.g. "stars" finds `heroicons-sparkles-symbolic` - and Heroicons v1
//! aliases resolve to their v2 names.
//!
//! Heroicons is a fontless icon set: grid cells render via the
//! `GtkIconTheme` (`glyph_icon_name`), not as font text.
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-heroicons-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_heroicons::Heroicons;
use fonts_rs_heroicons::HeroiconsIconName;
#[cfg(feature = "metadata")]
use fonts_rs_heroicons::metadata::HeroiconsMetadata;
use fonts_rs_model::CodePoint;
#[cfg(feature = "metadata")]
use fonts_rs_model::GlyphMetadata;
use miette::IntoDiagnostic;
use miette::Result;

/// Heroicons hooks for the generic cheat sheet.
pub struct HeroiconsSheet;

impl CheatSheetFamily for HeroiconsSheet {
    const TITLE: &'static str = "Heroicons - Cheat Sheet";
    const NOUN: &'static str = "icons";
    const APP_ID: &'static str = "io.smearor.fonts_rs.heroicons_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name, keyword, or codepoint (e.g. 'star', 'glitter', 'EA01')...";
    const NAME_FIELD_LABEL: &'static str = "Icon Name";
    const FAMILY_NAME: &'static str = "Heroicons";
    const FAMILY_KIND: FamilyKind = FamilyKind::IconSet;
    const LICENSE: &'static str = "MIT";

    type Name = HeroiconsIconName;

    fn init() -> Result<()> {
        Heroicons::install().into_diagnostic()?;
        // Grid cells render via Image::from_icon_name - the icon theme
        // path must be added to the display's GtkIconTheme. Runs after
        // gtk4::init(), so the default display is already open. No
        // install_pango/icon_css: Heroicons ships no TTF.
        let display = gtk4::gdk::Display::default().ok_or_else(|| miette::miette!("no default GDK display"))?;
        Heroicons::install_icon_theme(&display).into_diagnostic()
    }

    fn all_glyphs() -> Vec<(CodePoint, HeroiconsIconName)> {
        Heroicons::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        Heroicons::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<HeroiconsIconName> {
        HeroiconsIconName::parse(name)
    }

    fn display_name(name: &HeroiconsIconName) -> String {
        name.css_class_name().to_string()
    }

    fn subtitle(_name: &HeroiconsIconName) -> String {
        "Heroicons".to_string()
    }

    fn search(query: &str) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            HeroiconsMetadata.search(query).iter().map(|s| s.to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = query;
            Vec::new()
        }
    }

    fn keywords(name: &HeroiconsIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            HeroiconsMetadata.keywords(name).iter().map(|k| k.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    fn categories(name: &HeroiconsIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            HeroiconsMetadata.categories(name).iter().map(|c| c.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    /// Style groups (outline/solid/mini/micro) and outline-base
    /// variant collapsing.
    fn ordering() -> Option<&'static dyn fonts_rs_model::GlyphOrdering> {
        #[cfg(feature = "metadata")]
        {
            Some(&HeroiconsMetadata)
        }
        #[cfg(not(feature = "metadata"))]
        {
            None
        }
    }

    /// Resolves Heroicons v1 alias names to their v2 canonical names
    /// (e.g. `heroicons-exclaimation-circle-symbolic` →
    /// `heroicons-exclamation-circle-symbolic`).
    fn alias_display(name: &HeroiconsIconName) -> Option<String> {
        #[cfg(feature = "metadata")]
        {
            let canonical = HeroiconsMetadata.resolve_alias(name.as_ref())?;
            Some(
                HeroiconsIconName::parse(canonical)
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

    /// Render cells via the GtkIconTheme: Heroicons is fontless, so a
    /// text label would show a PUA glyph only if some installed font
    /// covers the codepoint - the bundled SVGs are authoritative.
    fn glyph_icon_name(name: &HeroiconsIconName) -> Option<String> {
        Some(name.as_ref().to_string())
    }

    fn glyph_svg(name: &HeroiconsIconName) -> Option<String> {
        Heroicons::glyph_svg(name)
    }
}

