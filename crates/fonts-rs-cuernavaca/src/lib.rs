//! Chess Cuernavaca font integration for GTK 4.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the Cuernavaca chess piece font.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_cuernavaca::Cuernavaca;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = Cuernavaca::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_cuernavaca::Cuernavaca;
//!
//! let codepoint = Cuernavaca::codepoint("k");
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

// Re-export key types
pub use definition::CuernavacaConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Cuernavaca;
pub use naming::CuernavacaName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`CuernavacaName`] from the [`naming`] module - this alias
/// is provided for consistency with the template structure.
pub type FamilyName = CuernavacaName;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_glyphs_is_not_empty() {
        let count = Cuernavaca::glyph_count();
        assert!(count > 0, "Cuernavaca should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_are_lowercase() {
        for (_, name) in Cuernavaca::all_glyphs() {
            assert_eq!(name.as_ref(), name.as_ref().to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_glyphs_names_contain_no_underscores() {
        for (_, name) in Cuernavaca::all_glyphs() {
            assert!(!name.as_ref().contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_glyphs_names_start_with_cuernavaca() {
        for (_, name) in Cuernavaca::all_glyphs() {
            assert!(name.as_ref().starts_with("cuernavaca-"), "glyph name '{name}' should start with 'cuernavaca-'");
        }
    }

    #[test]
    fn all_glyphs_codepoints_are_unique() {
        let codepoints: Vec<char> = Cuernavaca::all_glyphs().map(|(c, _)| c.as_char()).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }

    /// Every piece letter maps to its piece category; squares and
    /// borders carry none.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_piece_categories() {
        use fonts_rs_model::GlyphMetadata;
        let categories = |name: &str| metadata::CuernavacaMetadata.categories(name).iter().map(|c| c.as_str()).collect::<Vec<_>>();
        assert_eq!(categories("cuernavaca-k"), ["king"]);
        assert_eq!(categories("cuernavaca-l"), ["king"]);
        assert_eq!(categories("cuernavaca-d"), ["rook"]); // upside-down rook / rook+knight fairy
        assert_eq!(categories("cuernavaca-s"), ["bishop"]);
        assert_eq!(categories("cuernavaca-g"), ["knight"]);
        assert_eq!(categories("cuernavaca-o"), ["pawn"]);
        assert!(categories("cuernavaca-plus").is_empty()); // dark square
        assert!(categories("cuernavaca-exclam").is_empty()); // border
    }

    /// The four styles per piece form a variant set: `light normal`
    /// is the base, dark and upside-down styles link onto it.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_style_variants() {
        use fonts_rs_model::GlyphOrdering;
        let md = metadata::CuernavacaMetadata;
        assert_eq!(md.alternates("cuernavaca-r"), ["cuernavaca-t", "cuernavaca-d", "cuernavaca-f"].as_slice());
        assert_eq!(md.alternates("cuernavaca-n"), ["cuernavaca-m", "cuernavaca-g", "cuernavaca-h"].as_slice());
        assert_eq!(md.alternates("cuernavaca-k"), ["cuernavaca-l"].as_slice());
        assert_eq!(md.base_of("cuernavaca-v"), Some("cuernavaca-b"));
        assert_eq!(md.base_of("cuernavaca-a"), Some("cuernavaca-b"));
        assert!(md.base_of("cuernavaca-k").is_none());
        assert!(md.base_of("cuernavaca-plus").is_none());
    }

    /// Style groups cover every piece glyph; the canonical order
    /// lists pieces in king..pawn order.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_groups_and_order() {
        use fonts_rs_model::GlyphOrdering;
        let md = metadata::CuernavacaMetadata;
        let labels: Vec<&str> = md.group_labels().iter().map(|l| l.as_str()).collect();
        assert_eq!(labels, ["light normal", "dark normal", "light upside-down", "dark upside-down"]);
        assert_eq!(md.group("cuernavaca-h").map(|g| g.as_str()), Some("dark upside-down"));
        assert!(md.group("cuernavaca-space").is_none());
        // King sorts before queen (piece order, not letter order).
        assert!(md.order_index("cuernavaca-k").unwrap() < md.order_index("cuernavaca-q").unwrap());
        // Variants resolve to their base's position.
        assert!(md.order_index("cuernavaca-l").is_none());
    }
}
