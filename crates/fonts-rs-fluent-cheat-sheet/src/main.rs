//! Fluent UI System Icons Cheat Sheet application.
//!
//! Browse all available Fluent icons with a search filter.
//! Displays each icon with its Unicode codepoint and icon name.
//! Click an icon to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Fluent hooks via [`CheatSheetFamily`]. With the `metadata`
//! feature (default), search also covers filename-derived keywords
//! (e.g. "square down" finds `fluent-arrow-square-down-symbolic`) and
//! iconify aliases (e.g. "magic-wand" finds `fluent-wand-symbolic`).
//!
//! Fluent is a hybrid icon set: grid cells render via the
//! `GtkIconTheme` (`glyph_icon_name`) - the bundled SVGs are
//! authoritative, while the bundled TTF backs `fluent-icon` text
//! labels so PUA codepoints never fall back to a random system font.
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-fluent-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_fluent::Fluent;
use fonts_rs_fluent::FluentIconName;
#[cfg(feature = "metadata")]
use fonts_rs_fluent::metadata::FluentMetadata;
use fonts_rs_model::CodePoint;
use fonts_rs_model::FontFamily;
#[cfg(feature = "metadata")]
use fonts_rs_model::GlyphMetadata;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

/// Fluent UI System Icons hooks for the generic cheat sheet.
struct FluentSheet;

impl CheatSheetFamily for FluentSheet {
    const TITLE: &'static str = "Fluent UI System Icons - Cheat Sheet";
    const NOUN: &'static str = "icons";
    const APP_ID: &'static str = "io.smearor.fonts_rs.fluent_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name, keyword, alias, or codepoint (e.g. 'home', 'wand', 'F0001')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("fluent-icon");
    const NAME_FIELD_LABEL: &'static str = "Icon Name";
    const FAMILY_NAME: &'static str = "Fluent UI System Icons";
    const FAMILY_KIND: FamilyKind = FamilyKind::IconSet;
    const LICENSE: &'static str = "MIT";

    type Name = FluentIconName;

    fn init() -> Result<()> {
        Fluent::install().into_diagnostic()?;
        // Grid cells render via Image::from_icon_name - the icon theme
        // path must be added to the display's GtkIconTheme. Runs after
        // gtk4::init(), so the default display is already open.
        let display = gtk4::gdk::Display::default().ok_or_else(|| miette::miette!("no default GDK display"))?;
        Fluent::install_icon_theme(&display).into_diagnostic()?;
        // Hybrid set: the bundled TTF drives `fluent-icon` text labels.
        // Without install_pango, PUA codepoints resolve via whatever
        // system font claims them (e.g. Nerd Fonts).
        Fluent::install_pango(&pangocairo::FontMap::default()).into_diagnostic()?;
        fonts_rs_model::gtk::load_css(&fonts_rs_model::icon_css(&[("fluent-icon", Fluent::CSS_FONT_FAMILY)]));
        Ok(())
    }

    fn all_glyphs() -> Vec<(CodePoint, FluentIconName)> {
        Fluent::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        Fluent::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<FluentIconName> {
        FluentIconName::parse(name)
    }

    fn display_name(name: &FluentIconName) -> String {
        name.css_class_name().to_string()
    }

    fn subtitle(_name: &FluentIconName) -> String {
        "Fluent UI System Icons".to_string()
    }

    fn search(query: &str) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            FluentMetadata.search(query).iter().map(|s| s.to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = query;
            Vec::new()
        }
    }

    fn keywords(name: &FluentIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            FluentMetadata.keywords(name).iter().map(|k| k.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    fn categories(name: &FluentIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            FluentMetadata.categories(name).iter().map(|c| c.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    /// Regular/filled style groups and regular->filled pair
    /// collapsing.
    fn ordering() -> Option<&'static dyn fonts_rs_model::GlyphOrdering> {
        #[cfg(feature = "metadata")]
        {
            Some(&FluentMetadata)
        }
        #[cfg(not(feature = "metadata"))]
        {
            None
        }
    }

    /// Resolves iconify alias names to their canonical Fluent names
    /// (e.g. `fluent-arrow-down-square-symbolic` ->
    /// `fluent-arrow-square-down-symbolic`).
    fn alias_display(name: &FluentIconName) -> Option<String> {
        #[cfg(feature = "metadata")]
        {
            let canonical = FluentMetadata.resolve_alias(name.as_ref())?;
            Some(
                FluentIconName::parse(canonical)
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

    /// Render cells via the GtkIconTheme: a text label would render the
    /// PUA char via font fallback - the bundled SVGs are authoritative
    /// (per the Phase-4 plan). The `fluent-icon` CSS class remains as
    /// the font path for detail previews.
    fn glyph_icon_name(name: &FluentIconName) -> Option<String> {
        Some(name.as_ref().to_string())
    }

    fn glyph_svg(name: &FluentIconName) -> Option<String> {
        Fluent::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<FluentSheet>()
}
