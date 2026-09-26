//! Doto dot-matrix variable font integration for GTK 4.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the Doto font family - a dot-matrix display font with two
//! variable axes (`wght` and `rond`).
//!
//! The active variant (weight + roundness) is selected at build time via
//! Cargo features from a 5x5 matrix:
//!
//! - **wght**: 100 (ultra-light), 300 (light), 500 (regular), 700 (bold), 900 (extra-bold)
//! - **rond**: 0 (square), 25 (soft-square), 50 (medium), 75 (soft-dot), 100 (dot)
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_doto::Doto;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = Doto::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_doto::Doto;
//!
//! let codepoint = Doto::codepoint("a");
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

// Re-export key types
pub use definition::DotoConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Doto;
pub use naming::DotoName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`DotoName`] from the [`naming`] module - this alias
/// is provided for consistency with the template structure.
pub type FamilyName = DotoName;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_glyphs_is_not_empty() {
        let count = Doto::glyph_count();
        assert!(count > 0, "Doto should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_start_with_doto() {
        for (_, name) in Doto::all_glyphs() {
            assert!(name.as_ref().starts_with("doto-"), "glyph name '{name}' should start with 'doto-'");
        }
    }

    #[test]
    fn all_glyphs_names_are_lowercase() {
        for (_, name) in Doto::all_glyphs() {
            assert_eq!(name.as_ref(), name.as_ref().to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_glyphs_names_contain_no_underscores() {
        for (_, name) in Doto::all_glyphs() {
            assert!(!name.as_ref().contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_glyphs_codepoints_are_unique() {
        let codepoints: Vec<char> = Doto::all_glyphs().map(|(c, _)| c.as_char()).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }
}
