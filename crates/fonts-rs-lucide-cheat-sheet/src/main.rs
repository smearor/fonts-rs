//! Lucide Icons Cheat Sheet application.
//!
//! Browse all available Lucide icons with a search filter.
//! Displays each icon with its Unicode codepoint and icon name.
//! Click an icon to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Lucide hooks via [`CheatSheetFamily`]. With the `metadata`
//! feature (default), search also covers upstream tags from
//! `icons.json` - e.g. "font size" finds `lucide-a-arrow-down-symbolic`.
//!
//! Lucide is a fontless icon set: grid cells render via the
//! `GtkIconTheme` (`glyph_icon_name`), not as font text. Upstream
//! `categories` populate the green category chips - the first family
//! with real icon-set categories.
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-lucide-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_lucide::Lucide;
use fonts_rs_lucide::LucideIconName;
#[cfg(feature = "metadata")]
use fonts_rs_lucide::metadata::LucideMetadata;
use fonts_rs_model::CodePoint;
#[cfg(feature = "metadata")]
use fonts_rs_model::GlyphMetadata;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

/// Lucide Icons hooks for the generic cheat sheet.
struct LucideSheet;

impl CheatSheetFamily for LucideSheet {
    const TITLE: &'static str = "Lucide Icons - Cheat Sheet";
    const NOUN: &'static str = "icons";
    const APP_ID: &'static str = "io.smearor.fonts_rs.lucide_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name, keyword, category, or codepoint (e.g. 'home', 'alarm', 'EA01')...";
    const NAME_FIELD_LABEL: &'static str = "Icon Name";
    const FAMILY_NAME: &'static str = "Lucide Icons";
    const FAMILY_KIND: FamilyKind = FamilyKind::IconSet;
    const LICENSE: &'static str = "ISC";

    type Name = LucideIconName;

    fn init() -> Result<()> {
        Lucide::install().into_diagnostic()?;
        // Grid cells render via Image::from_icon_name - the icon theme
        // path must be added to the display's GtkIconTheme. Runs after
        // gtk4::init(), so the default display is already open. No
        // install_pango/icon_css: Lucide ships no TTF.
        let display = gtk4::gdk::Display::default().ok_or_else(|| miette::miette!("no default GDK display"))?;
        Lucide::install_icon_theme(&display).into_diagnostic()
    }

    fn all_glyphs() -> Vec<(CodePoint, LucideIconName)> {
        Lucide::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        Lucide::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<LucideIconName> {
        LucideIconName::parse(name)
    }

    fn display_name(name: &LucideIconName) -> String {
        name.css_class_name().to_string()
    }

    fn subtitle(_name: &LucideIconName) -> String {
        "Lucide Icons".to_string()
    }

    fn search(query: &str) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            LucideMetadata.search(query).iter().map(|s| s.to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = query;
            Vec::new()
        }
    }

    fn keywords(name: &LucideIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            LucideMetadata.keywords(name).iter().map(|k| k.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    fn categories(name: &LucideIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            LucideMetadata.categories(name).iter().map(|c| c.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    /// Render cells via the GtkIconTheme: Lucide is fontless, so a
    /// text label would show a PUA glyph only if some installed font
    /// covers the codepoint - the bundled SVGs are authoritative.
    fn glyph_icon_name(name: &LucideIconName) -> Option<String> {
        Some(name.as_ref().to_string())
    }

    fn glyph_svg(name: &LucideIconName) -> Option<String> {
        Lucide::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<LucideSheet>()
}
