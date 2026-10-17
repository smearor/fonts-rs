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
use fonts_rs_cheat_sheet::FamilyKind;
use fonts_rs_model::CodePoint;
use fonts_rs_model::GlyphOrdering;
use miette::IntoDiagnostic;
use miette::Result;

#[cfg(feature = "metadata")]
use fonts_rs_blobmoji::metadata::BlobmojiMetadata;
#[cfg(feature = "metadata")]
use fonts_rs_model::GlyphMetadata;

/// Blobmoji hooks for the generic cheat sheet.
pub struct BlobmojiSheet;

impl CheatSheetFamily for BlobmojiSheet {
    const TITLE: &'static str = "Blobmoji - Cheat Sheet";
    const NOUN: &'static str = "emoji";
    const APP_ID: &'static str = "io.smearor.fonts_rs.blobmoji_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search by name (e.g. 'grinning', 'cat', 'flag')...";
    const NAME_FIELD_LABEL: &'static str = "Emoji Name";
    const FAMILY_NAME: &'static str = "Blobmoji";
    const FAMILY_KIND: FamilyKind = FamilyKind::Emoji;
    const LICENSE: &'static str = "OFL-1.1";

    type Name = BlobmojiName;

    fn init() -> Result<()> {
        Blobmoji::install().into_diagnostic()?;
        // Grid cells render via Image::from_icon_name - the icon theme
        // path must be added to the display's GtkIconTheme. Runs after
        // gtk4::init(), so the default display is already open.
        let display = gtk4::gdk::Display::default().ok_or_else(|| miette::miette!("no default GDK display"))?;
        Blobmoji::install_icon_theme(&display).into_diagnostic()
    }

    /// Emoji picker order when metadata is enabled: glyphs follow the
    /// googlefonts/emoji-metadata group ordering (smileys first, skin-tone
    /// alternates after their base). Glyphs outside the ordering (a few
    /// sequences without ordering entries) are appended, sorted by name.
    fn all_glyphs() -> Vec<(CodePoint, BlobmojiName)> {
        #[cfg(feature = "metadata")]
        {
            let mut by_name: std::collections::HashMap<String, (CodePoint, BlobmojiName)> =
                Blobmoji::all_glyphs().map(|(cp, name)| (name.as_ref().to_string(), (cp, name))).collect();
            let mut ordered: Vec<(CodePoint, BlobmojiName)> = Vec::with_capacity(by_name.len());
            for group in BlobmojiMetadata.groups() {
                for name in group.glyphs {
                    if let Some(entry) = by_name.remove(*name) {
                        ordered.push(entry);
                    }
                }
            }
            let mut rest: Vec<(CodePoint, BlobmojiName)> = by_name.into_values().collect();
            rest.sort_by(|a, b| a.1.cmp(&b.1));
            ordered.extend(rest);
            ordered
        }
        #[cfg(not(feature = "metadata"))]
        {
            Blobmoji::all_glyphs().collect()
        }
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

    /// Picker ordering contract: enables picker-order grid sorting,
    /// the group jump-list, and group labels in the detail sidebar.
    fn ordering() -> Option<&'static dyn GlyphOrdering> {
        #[cfg(feature = "metadata")]
        {
            Some(&BlobmojiMetadata)
        }
        #[cfg(not(feature = "metadata"))]
        {
            None
        }
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

#[cfg(test)]
mod tests {
    use super::BlobmojiSheet;
    use fonts_rs_cheat_sheet::CheatSheetFamily;

    /// The ordering hook drives grid variant collapsing, the group
    /// dropdown and the sidebar Variants section - it must be active
    /// under the crate's default features.
    #[test]
    fn ordering_hook_is_active() {
        let ordering = BlobmojiSheet::ordering().expect("ordering should be enabled (metadata feature)");
        assert!(ordering.collapse_variants());
    }

    /// The collapse filter in the grid relies on `base_of` resolving
    /// variant glyph names to their base - verified on thumbs-up.
    #[test]
    fn ordering_resolves_thumbs_up_variants() {
        let ordering = BlobmojiSheet::ordering().expect("ordering should be enabled");
        let base = "blobmoji-color-flags-thumbs-up";
        assert!(!ordering.alternates(base).is_empty(), "thumbs-up should have alternates");
        for variant in ordering.alternates(base) {
            assert_eq!(ordering.base_of(variant), Some(base), "variant '{variant}' should map to its base");
        }
    }

    /// Simulates the grid's browse filter (`render_grid` with empty
    /// search): after variant collapsing, exactly one thumbs-up cell
    /// must remain - the base glyph.
    #[test]
    fn browse_view_collapses_thumbs_up_variants() {
        let ordering = BlobmojiSheet::ordering().expect("ordering should be enabled");
        let collapse = ordering.collapse_variants();
        let thumbs_up_cells: Vec<String> = BlobmojiSheet::all_glyphs()
            .iter()
            .filter(|(_, name)| !(collapse && ordering.base_of(name.as_ref()).is_some()))
            .map(|(_, name)| name.as_ref().to_string())
            .filter(|name| name.contains("thumbs-up"))
            .collect();
        assert_eq!(thumbs_up_cells, vec!["blobmoji-color-flags-thumbs-up"]);
    }
}
