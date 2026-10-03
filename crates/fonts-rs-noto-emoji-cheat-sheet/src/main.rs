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
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_model::CodePoint;
use fonts_rs_model::GlyphOrdering;
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
    const FAMILY_NAME: &'static str = "Noto Emoji";
    const FAMILY_KIND: FamilyKind = FamilyKind::Emoji;
    const LICENSE: &'static str = "OFL-1.1";

    type Name = String;

    fn init() -> Result<()> {
        NotoEmoji::ensure_registered().into_diagnostic()?;
        // Grid cells render via Image::from_icon_name - the icon theme
        // path must be added to the display's GtkIconTheme. Runs after
        // gtk4::init(), so the default display is already open.
        let display = gtk4::gdk::Display::default().ok_or_else(|| miette::miette!("no default GDK display"))?;
        NotoEmoji::install_icon_theme(&display).into_diagnostic()
    }

    /// Emoji picker order when metadata is enabled: glyphs follow the
    /// googlefonts/emoji-metadata group ordering (smileys first, skin-tone
    /// alternates after their base). Glyphs outside the ordering (a few
    /// sequences without ordering entries) are appended, sorted by name.
    fn all_glyphs() -> Vec<(CodePoint, String)> {
        #[cfg(feature = "metadata")]
        {
            let mut by_name: std::collections::HashMap<String, (CodePoint, String)> =
                all_emoji().into_iter().map(|(cp, name)| (name.clone(), (cp, name))).collect();
            let mut ordered: Vec<(CodePoint, String)> = Vec::with_capacity(by_name.len());
            for group in NotoEmojiMetadata.groups() {
                for name in group.glyphs {
                    if let Some(entry) = by_name.remove(*name) {
                        ordered.push(entry);
                    }
                }
            }
            let mut rest: Vec<(CodePoint, String)> = by_name.into_values().collect();
            rest.sort_by(|a, b| a.1.cmp(&b.1));
            ordered.extend(rest);
            ordered
        }
        #[cfg(not(feature = "metadata"))]
        {
            all_emoji()
        }
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

    /// Picker ordering contract: enables picker-order grid sorting,
    /// variant collapsing, tone picking, the group jump-list, and
    /// group labels in the detail sidebar.
    fn ordering() -> Option<&'static dyn GlyphOrdering> {
        #[cfg(feature = "metadata")]
        {
            Some(&NotoEmojiMetadata)
        }
        #[cfg(not(feature = "metadata"))]
        {
            None
        }
    }

    /// Render cells via the GtkIconTheme - deterministic bundled SVGs
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

#[cfg(test)]
mod tests {
    use super::NotoEmojiSheet;
    use fonts_rs_cheat_sheet::CheatSheetFamily;

    /// The ordering hook drives grid variant collapsing, tone picking,
    /// the group dropdown and the sidebar Variants section - it must
    /// be active under the crate's default features.
    #[test]
    fn ordering_hook_is_active() {
        let ordering = NotoEmojiSheet::ordering().expect("ordering should be enabled (metadata feature)");
        assert!(ordering.collapse_variants());
    }

    /// The collapse filter in the grid relies on `base_of` resolving
    /// variant glyph names to their base - verified on thumbs-up.
    #[test]
    fn ordering_resolves_thumbs_up_variants() {
        let ordering = NotoEmojiSheet::ordering().expect("ordering should be enabled");
        let base = NotoEmojiSheet::all_glyphs()
            .iter()
            .map(|(_, name)| name)
            .find(|name| name.ends_with("thumbs-up") || name.ends_with("thumbsup"))
            .expect("thumbs-up glyph should exist")
            .clone();
        assert!(!ordering.alternates(&base).is_empty(), "thumbs-up should have alternates");
        for variant in ordering.alternates(&base) {
            assert_eq!(ordering.base_of(variant), Some(base.as_str()), "variant '{variant}' should map to its base");
        }
    }

    /// Simulates the grid's browse filter (`render_grid` with empty
    /// search): after variant collapsing, exactly one thumbs-up cell
    /// must remain - the base glyph.
    #[test]
    fn browse_view_collapses_thumbs_up_variants() {
        let ordering = NotoEmojiSheet::ordering().expect("ordering should be enabled");
        let collapse = ordering.collapse_variants();
        let thumbs_up_cells: Vec<String> = NotoEmojiSheet::all_glyphs()
            .iter()
            .filter(|(_, name)| !(collapse && ordering.base_of(name).is_some()))
            .map(|(_, name)| name.clone())
            .filter(|name| name.contains("thumbs-up"))
            .collect();
        assert_eq!(thumbs_up_cells.len(), 1, "exactly one thumbs-up cell should remain");
        assert!(thumbs_up_cells[0].ends_with("thumbs-up") || thumbs_up_cells[0].ends_with("thumbsup"));
    }
}
