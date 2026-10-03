//! Dicefont dice face font integration for GTK 4.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the Dicefont font family - an icon font with die face
//! glyphs for all standard dice (D2, D4, D6, D8, D10, D12, D20) plus
//! dot-d6 variants.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_dicefont::Dicefont;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = Dicefont::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_dicefont::Dicefont;
//!
//! let codepoint = Dicefont::codepoint("d6-1");
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
pub use definition::DicefontConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Dicefont;
pub use naming::DicefontName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`DicefontName`] from the [`naming`] module - this alias
/// is provided for consistency with the template structure.
pub type FamilyName = DicefontName;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_glyphs_is_not_empty() {
        let count = Dicefont::glyph_count();
        assert!(count > 0, "Dicefont should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_start_with_dicefont() {
        for (_, name) in Dicefont::all_glyphs() {
            assert!(name.as_ref().starts_with("dicefont-"), "glyph name '{name}' should start with 'dicefont-'");
        }
    }

    #[test]
    fn all_glyphs_names_are_lowercase() {
        for (_, name) in Dicefont::all_glyphs() {
            assert_eq!(name.as_ref(), name.as_ref().to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_glyphs_names_contain_no_underscores() {
        for (_, name) in Dicefont::all_glyphs() {
            assert!(!name.as_ref().contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_glyphs_codepoints_are_unique() {
        let codepoints: Vec<char> = Dicefont::all_glyphs().map(|(c, _)| c.as_char()).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }

    #[test]
    fn all_glyphs_codepoints_in_pua() {
        for (c, _) in Dicefont::all_glyphs() {
            let cp = c.as_char() as u32;
            assert!((0xF000..=0xFFFF).contains(&cp), "codepoint U+{cp:04X} should be in the Private Use Area");
        }
    }
}
