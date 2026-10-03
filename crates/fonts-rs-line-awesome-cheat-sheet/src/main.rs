//! Line Awesome Cheat Sheet application.
//!
//! Browse all available Line Awesome icons with a search filter.
//! Displays each icon with its Unicode codepoint and icon name.
//! Click an icon to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Line Awesome hooks via [`CheatSheetFamily`]. With the `metadata`
//! feature (default), search also covers filename-derived keywords
//! (e.g. "address book" finds `la-address-book-symbolic`), the style
//! categories (`regular`/`solid`/`brands`) and the vendored Font
//! Awesome aliases (e.g. "address-book-o" finds
//! `la-address-book-symbolic`).
//!
//! Line Awesome is a hybrid icon set: grid cells render via the
//! `GtkIconTheme` (`glyph_icon_name`) - the bundled SVGs are
//! authoritative, while the three bundled `la-{regular-400,solid-900,
//! brands-400}.ttf` webfonts back `la-icon` text labels so PUA
//! codepoints never fall back to a random system font.
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-line-awesome-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_line_awesome::LineAwesome;
use fonts_rs_line_awesome::LineAwesomeIconName;
#[cfg(feature = "metadata")]
use fonts_rs_line_awesome::metadata::LineAwesomeMetadata;
use fonts_rs_model::CodePoint;
use fonts_rs_model::FontFamily;
#[cfg(feature = "metadata")]
use fonts_rs_model::GlyphMetadata;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

/// Line Awesome hooks for the generic cheat sheet.
struct LineAwesomeSheet;

impl CheatSheetFamily for LineAwesomeSheet {
    const TITLE: &'static str = "Line Awesome - Cheat Sheet";
    const NOUN: &'static str = "icons";
    const APP_ID: &'static str = "io.smearor.fonts_rs.line_awesome_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name, keyword, alias, style, or codepoint (e.g. 'home', 'address-book-o', 'F0001')...";
    const GLYPH_CSS_CLASS: Option<&'static str> = Some("la-icon");
    const NAME_FIELD_LABEL: &'static str = "Icon Name";
    const FAMILY_NAME: &'static str = "Line Awesome";
    const FAMILY_KIND: FamilyKind = FamilyKind::IconSet;
    const LICENSE: &'static str = "MIT or Good Boy License";

    type Name = LineAwesomeIconName;

    fn init() -> Result<()> {
        LineAwesome::install().into_diagnostic()?;
        // Grid cells render via Image::from_icon_name - the icon theme
        // path must be added to the display's GtkIconTheme. Runs after
        // gtk4::init(), so the default display is already open.
        let display = gtk4::gdk::Display::default().ok_or_else(|| miette::miette!("no default GDK display"))?;
        LineAwesome::install_icon_theme(&display).into_diagnostic()?;
        // Hybrid set: the three bundled TTFs drive `la-icon` text
        // labels. Without install_pango, PUA codepoints resolve via
        // whatever system font claims them (e.g. Nerd Fonts).
        LineAwesome::install_pango(&pangocairo::FontMap::default()).into_diagnostic()?;
        fonts_rs_model::gtk::load_css(&fonts_rs_model::icon_css(&[("la-icon", LineAwesome::CSS_FONT_FAMILY)]));
        Ok(())
    }

    fn all_glyphs() -> Vec<(CodePoint, LineAwesomeIconName)> {
        LineAwesome::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        LineAwesome::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<LineAwesomeIconName> {
        LineAwesomeIconName::parse(name)
    }

    fn display_name(name: &LineAwesomeIconName) -> String {
        name.css_class_name().to_string()
    }

    fn subtitle(_name: &LineAwesomeIconName) -> String {
        "Line Awesome (Font Awesome 5 compatible)".to_string()
    }

    fn search(query: &str) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            LineAwesomeMetadata.search(query).iter().map(|s| s.to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = query;
            Vec::new()
        }
    }

    fn keywords(name: &LineAwesomeIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            LineAwesomeMetadata.keywords(name).iter().map(|k| k.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    /// Style categories (`regular`/`solid`/`brands`) derived from the
    /// filename-pairing rule - they pre-fill the cheat sheet's
    /// category dropdown.
    fn categories(name: &LineAwesomeIconName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            LineAwesomeMetadata.categories(name).iter().map(|c| c.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    /// Style-group dropdown (`regular`/`solid`/`brands`) plus
    /// regular/solid variant collapsing, backed by
    /// `LineAwesomeMetadata`'s `GlyphOrdering` implementation.
    fn ordering() -> Option<&'static dyn fonts_rs_model::GlyphOrdering> {
        #[cfg(feature = "metadata")]
        {
            Some(&LineAwesomeMetadata)
        }
        #[cfg(not(feature = "metadata"))]
        {
            None
        }
    }

    /// Resolves vendored Font Awesome aliases to their canonical
    /// Line Awesome names (e.g. `la-address-book-o-symbolic` ->
    /// `la-address-book-symbolic`).
    fn alias_display(name: &LineAwesomeIconName) -> Option<String> {
        #[cfg(feature = "metadata")]
        {
            let canonical = LineAwesomeMetadata.resolve_alias(name.as_ref())?;
            Some(
                LineAwesomeIconName::parse(canonical)
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
    /// PUA char via font fallback - the bundled SVGs are authoritative.
    /// The `la-icon` CSS class remains as the font path for detail
    /// previews.
    fn glyph_icon_name(name: &LineAwesomeIconName) -> Option<String> {
        Some(name.as_ref().to_string())
    }

    fn glyph_svg(name: &LineAwesomeIconName) -> Option<String> {
        LineAwesome::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<LineAwesomeSheet>()
}
