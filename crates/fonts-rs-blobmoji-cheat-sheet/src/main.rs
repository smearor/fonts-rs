//! Blobmoji Cheat Sheet application.
//!
//! Browse all available Blobmoji emoji with a search filter.
//! Displays each emoji with its Unicode codepoint and glyph name.
//! Click an emoji to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Blobmoji hooks via [`CheatSheetFamily`].
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-blobmoji-cheat-sheet
//! ```

use fonts_rs_blobmoji::Blobmoji;
use fonts_rs_blobmoji::BlobmojiName;
use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_model::CodePoint;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

#[cfg(feature = "metadata")]
use fonts_rs_blobmoji::metadata::BlobmojiMetadata;
#[cfg(feature = "metadata")]
use fonts_rs_model::GlyphMetadata;

/// Blobmoji hooks for the generic cheat sheet.
struct BlobmojiSheet;

impl CheatSheetFamily for BlobmojiSheet {
    const TITLE: &'static str = "Blobmoji - Cheat Sheet";
    const NOUN: &'static str = "emoji";
    const APP_ID: &'static str = "io.smearor.fonts_rs.blobmoji_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name (e.g. 'grinning', 'cat', 'flag')...";
    const NAME_FIELD_LABEL: &'static str = "Emoji Name";

    type Name = BlobmojiName;

    fn init() -> Result<()> {
        Blobmoji::install().into_diagnostic()?;
        // Grid cells render via Image::from_icon_name - the icon theme
        // path must be added to the display's GtkIconTheme. Runs after
        // gtk4::init(), so the default display is already open.
        let display = gtk4::gdk::Display::default().ok_or_else(|| miette::miette!("no default GDK display"))?;
        Blobmoji::install_icon_theme(&display).into_diagnostic()
    }

    fn all_glyphs() -> Vec<(CodePoint, BlobmojiName)> {
        Blobmoji::all_glyphs().collect()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        Blobmoji::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<BlobmojiName> {
        name.parse().ok()
    }

    fn display_name(name: &BlobmojiName) -> String {
        name.as_ref().to_string()
    }

    fn subtitle(_name: &BlobmojiName) -> String {
        "Blobmoji".to_string()
    }

    fn search(query: &str) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            BlobmojiMetadata.search(query).iter().map(|s| s.to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = query;
            Vec::new()
        }
    }

    fn keywords(name: &BlobmojiName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            BlobmojiMetadata.keywords(name).iter().map(|k| k.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    fn categories(name: &BlobmojiName) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            BlobmojiMetadata.categories(name).iter().map(|c| c.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    fn alias_display(_name: &BlobmojiName) -> Option<String> {
        None
    }

    /// Render cells via the GtkIconTheme: the bundled TTFs declare family
    /// "Noto Color Emoji", which fontconfig resolves to the system emoji
    /// font - a text label would show non-blob artwork.
    fn glyph_icon_name(name: &BlobmojiName) -> Option<String> {
        Some(name.as_ref().to_string())
    }

    fn glyph_svg(name: &BlobmojiName) -> Option<String> {
        Blobmoji::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<BlobmojiSheet>()
}
