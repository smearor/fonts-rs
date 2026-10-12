//! Noto Emoji `color-flags` font integration for GTK 4 and pixel-drawing.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the `color-flags` variant of the Noto Emoji font family -
//! Google's COLRv1 vector emoji font (`Noto-COLRv1`, including flag emoji)
//! covering all Unicode emoji codepoints with semantic names from CLDR
//! annotation data. GResource SVGs ship full-color artwork regardless of
//! variant.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_noto_emoji_color_flags::NotoEmoji;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = NotoEmoji::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_noto_emoji_color_flags::NotoEmoji;
//!
//! let codepoint = NotoEmoji::codepoint("grinning-face");
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
pub use definition::NotoEmojiConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::NotoEmoji;
pub use naming::NotoEmojiName;

/// Whether the bundled TTF rasterizes visible output via `ab_glyph`.
///
/// COLRv1 glyphs carry empty `glyf` outlines - pixel-drawing produces
/// no visible output. `true` for `mono` and `bitmap-*` variants.
pub const RENDERABLE: bool = false;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`NotoEmojiName`] from the [`naming`] module - this alias
/// is provided for consistency with the template structure.
pub type FamilyName = NotoEmojiName;

/// Returns all known Noto Emoji glyphs as `(CodePoint, glyph_name)` pairs,
/// sorted alphabetically by glyph name.
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

    /// `glyph_name` must resolve emoji codepoints to variant-prefixed
    /// icon names, and `glyph_svg` must return the colored vector
    /// artwork from the GResource - the chain behind `gtk_icon_theme`
    /// and `svg_gresource`.
    #[cfg(feature = "gtk")]
    #[test]
    fn glyph_svg_returns_colored_artwork() {
        let name = NotoEmoji::glyph_name('\u{1F600}').expect("grinning face should resolve");
        let svg = NotoEmoji::glyph_svg(&name).expect("glyph SVG should be in the GResource");
        assert!(svg.contains("<svg"), "expected an SVG document, got: {}", &svg[..svg.len().min(200)]);
        assert!(
            svg.contains("stop-color") || svg.contains("fill:") || svg.contains("fill=\""),
            "expected colored artwork in {name}"
        );
    }

    /// `compose_svg` must produce a parseable document for glyphs whose
    /// source SVGs carried Illustrator DOCTYPE entity namespaces
    /// (`emoji_u1f680.svg` declares `xmlns:i="&ns_ai;"`). The export
    /// strips them via `StripEntityNamespaces`; a regression would
    /// surface as "namespace prefix" parse errors in GTK.
    #[cfg(feature = "gtk")]
    #[test]
    fn compose_svg_has_no_unbound_prefixes() {
        let svg = NotoEmoji::compose_svg("\u{1F680}", 200.0);
        assert!(svg.contains("<svg"), "expected a composed SVG document");
        assert!(!svg.contains("&ns_"), "entity references must not leak into the composed SVG");
        assert!(!svg.contains("i:aipgf"), "Illustrator metadata must be stripped");
    }
}
