//! Blobmoji font integration for GTK 4 and pixel-drawing.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the Blobmoji font family - a community-maintained emoji
//! font recreating Google's classic "blob" emoji style, covering all
//! Unicode emoji codepoints with semantic names from CLDR annotation data.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_blobmoji::Blobmoji;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = Blobmoji::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_blobmoji::Blobmoji;
//!
//! let codepoint = Blobmoji::codepoint("grinning-face");
//! ```

pub mod codepoint_map;
pub mod constants;
pub mod definition;
mod family;
pub mod naming;
pub mod variant;

#[cfg(feature = "gtk")]
pub mod gtk;

#[cfg(feature = "render")]
pub mod fonts;

#[cfg(feature = "metadata")]
pub mod metadata;

use fonts_rs_model::CodePoint;

// Re-export key types
pub use definition::BlobmojiConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Blobmoji;
pub use naming::BlobmojiName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`BlobmojiName`] from the [`naming`] module - this alias
/// is provided for consistency with the template structure.
pub type FamilyName = BlobmojiName;

/// Returns all known Blobmoji glyphs as `(CodePoint, glyph_name)` pairs,
/// sorted alphabetically by glyph name.
///
/// Emoji sequences (ZWJ, keycap, flag) are represented by their
/// allocated PUA codepoint; the original sequence is recorded in
/// `metadata.json`.
pub fn all_emoji() -> Vec<(CodePoint, String)> {
    let mut emojis: Vec<(CodePoint, String)> = codepoint_map::GLYPHS
        .entries()
        .map(|(cp, name)| (CodePoint::from(*cp), name.to_string()))
        .collect();
    emojis.sort_by(|a, b| a.1.cmp(&b.1));
    emojis
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_emoji_is_not_empty() {
        let count = Blobmoji::glyph_count();
        assert!(count > 0, "Blobmoji should export at least one glyph");
    }

    #[test]
    fn all_emoji_names_start_with_blobmoji() {
        for (_, name) in Blobmoji::all_glyphs() {
            assert!(name.as_ref().starts_with("blobmoji-"), "glyph name '{name}' should start with 'blobmoji-'");
        }
    }

    #[test]
    fn all_emoji_names_are_lowercase() {
        for (_, name) in Blobmoji::all_glyphs() {
            assert_eq!(name.as_ref(), name.as_ref().to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_emoji_names_contain_no_underscores() {
        for (_, name) in Blobmoji::all_glyphs() {
            assert!(!name.as_ref().contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_emoji_codepoints_are_unique() {
        let codepoints: Vec<char> = Blobmoji::all_glyphs().map(|(c, _)| c.as_char()).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }

    #[test]
    fn codepoint_resolves_unprefixed_cldr_name() {
        // `variant::GLYPH_PREFIX` is consulted when the bare name misses.
        let codepoint = Blobmoji::codepoint("grinning-face");
        assert_eq!(codepoint.map(|c| c.as_char()), Some('\u{1F600}'));
    }

    #[test]
    fn glyph_name_resolves_emoji_codepoint() {
        let name = Blobmoji::glyph_name('\u{1F600}');
        assert!(name.is_some_and(|n| n.as_ref().ends_with("grinning-face")));
    }

    /// The TTF bundled for `pixel-drawing` must load via `ab_glyph` and
    /// resolve emoji codepoints through its cmap.
    #[cfg(feature = "render")]
    #[test]
    fn ttf_loads_and_resolves_emoji() {
        use ab_glyph::Font;
        let font = Blobmoji::font().expect("BLOBMOJI_FONT_PATH TTF should load via ab_glyph");
        let glyph_id = font.glyph_id('\u{1F600}');
        assert_ne!(glyph_id.0, 0, "grinning face should resolve to a real glyph, not .notdef");
    }

    /// `render_text` must produce a non-empty, non-black buffer for the
    /// bitmap variants: CBDT strikes are PNG blobs positioned by their
    /// bottom bearing - a regression test for the "1px line" and
    /// "black box" failure modes. Under `color-*` the COLRv1 font has
    /// empty `glyf` outlines (ab_glyph renders nothing), so this test
    /// only runs when the active variant ships CBDT strikes
    /// (`blobmoji_bitmap` cfg is emitted by build.rs).
    #[cfg(all(feature = "gtk", feature = "render", blobmoji_bitmap))]
    #[test]
    fn render_text_blits_bitmap_strikes() {
        let font = Blobmoji::font().expect("BLOBMOJI_FONT_PATH TTF should load via ab_glyph");
        let (width, height, pixels) = fonts_rs_test::render_text(font, "\u{1F600}", 64.0);
        assert!(width > 1 && height > 1, "rendered buffer should have real dimensions, got {width}x{height}");
        let non_black = pixels.chunks_exact(4).filter(|px| px[0] > 0 || px[1] > 0 || px[2] > 0).count();
        assert!(non_black > 0, "rendered buffer should contain non-black pixels");
    }

    /// `compose_svg` must keep prefixed namespaces bound: Blobmoji SVGs
    /// use `ns1:href` on `<use>` elements, declared via `xmlns:ns1` on
    /// the root tag. Stripping the root for composition must re-attach
    /// the declaration to the nested `<svg>`.
    #[cfg(feature = "gtk")]
    #[test]
    fn compose_svg_keeps_namespaces_bound() {
        let svg = Blobmoji::compose_svg("\u{1F680}", 200.0);
        assert!(svg.contains("viewBox"), "composed svg should embed the rocket glyph");
        if svg.contains("ns1:href") {
            assert!(svg.contains("xmlns:ns1"), "ns1:href used but xmlns:ns1 declaration missing");
        }
    }

    /// Bitmap variants (CBDT) must expose raster images for
    /// `pixel-drawing`. COLRv1 variants carry empty outlines instead, so
    /// this test only runs when the active variant ships CBDT strikes
    /// (`blobmoji_bitmap` cfg is emitted by build.rs, which also covers
    /// the default variant).
    #[cfg(all(feature = "render", blobmoji_bitmap))]
    #[test]
    fn ttf_provides_bitmap_raster_image() {
        use ab_glyph::Font;
        let font = Blobmoji::font().expect("BLOBMOJI_FONT_PATH TTF should load via ab_glyph");
        let glyph_id = font.glyph_id('\u{1F600}');
        assert!(
            font.glyph_raster_image2(glyph_id, 128).is_some(),
            "bitmap variant should provide a raster image for grinning face"
        );
    }
}
