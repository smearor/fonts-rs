//! Noto Emoji Cheat Sheet application.
//!
//! Browse all available Noto Emoji glyphs with a search filter.
//! Displays each emoji with its Unicode codepoint, glyph name,
//! keywords, and categories. Search covers glyph names, keywords,
//! and categories. Click an emoji to open a detail sidebar.
//!
//! The UI lives in `fonts-rs-cheat-sheet`; this binary only supplies
//! the Noto Emoji hooks via [`CheatSheetFamily`].
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-noto-emoji-cheat-sheet
//! ```

use fonts_rs_cheat_sheet::CheatSheetFamily;
use fonts_rs_model::CodePoint;
use fonts_rs_noto_emoji::NotoEmoji;
use fonts_rs_noto_emoji::all_emoji;
use gtk4::glib;
use miette::IntoDiagnostic;
use miette::Result;

#[cfg(feature = "metadata")]
use fonts_rs_model::GlyphMetadata;
#[cfg(feature = "metadata")]
use fonts_rs_noto_emoji::metadata::NotoEmojiMetadata;

/// Noto Emoji hooks for the generic cheat sheet.
struct NotoEmojiSheet;

impl CheatSheetFamily for NotoEmojiSheet {
    const TITLE: &'static str = "Noto Emoji - Cheat Sheet";
    const NOUN: &'static str = "emoji";
    const APP_ID: &'static str = "io.smearor.fonts_rs.noto_emoji_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name, keyword, or category (e.g. 'face', 'fire', 'Smileys')...";

    type Name = String;

    fn init() -> Result<()> {
        NotoEmoji::ensure_registered().into_diagnostic()?;
        // Grid cells render via Image::from_icon_name — the icon theme
        // path must be added to the display's GtkIconTheme. Runs after
        // gtk4::init(), so the default display is already open.
        let display = gtk4::gdk::Display::default().ok_or_else(|| miette::miette!("no default GDK display"))?;
        NotoEmoji::install_icon_theme(&display).into_diagnostic()
    }

    fn all_glyphs() -> Vec<(CodePoint, String)> {
        all_emoji()
    }

    fn codepoint(name: &str) -> Option<CodePoint> {
        NotoEmoji::codepoint(name)
    }

    fn parse_name(name: &str) -> Option<String> {
        Some(name.to_string())
    }

    fn display_name(name: &String) -> String {
        name.clone()
    }

    fn subtitle(_name: &String) -> String {
        "Noto Emoji".to_string()
    }

    fn search(query: &str) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            NotoEmojiMetadata.search(query).iter().map(|s| s.to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = query;
            Vec::new()
        }
    }

    fn keywords(name: &String) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            NotoEmojiMetadata.keywords(name).iter().map(|k| k.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    fn categories(name: &String) -> Vec<String> {
        #[cfg(feature = "metadata")]
        {
            NotoEmojiMetadata.categories(name).iter().map(|c| c.as_str().to_string()).collect()
        }
        #[cfg(not(feature = "metadata"))]
        {
            let _ = name;
            Vec::new()
        }
    }

    /// Render cells via the GtkIconTheme — deterministic bundled SVGs
    /// instead of whichever emoji font fontconfig resolves on the
    /// system.
    fn glyph_icon_name(name: &String) -> Option<String> {
        Some(name.clone())
    }

    fn glyph_svg(name: &String) -> Option<String> {
        NotoEmoji::glyph_svg(name)
    }
}

fn main() -> Result<glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<NotoEmojiSheet>()
}
