//! Bravura SMuFL music notation font integration for GTK 4.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the Bravura font family — the reference font for the
//! Standard Music Font Layout (SMuFL) specification, covering thousands
//! of musical notation symbols mapped to the Unicode Private Use Area.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_bravura::Bravura;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = Bravura::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_bravura::Bravura;
//!
//! let codepoint = Bravura::codepoint("g-clef");
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
pub use definition::BravuraConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Bravura;
pub use naming::BravuraName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`BravuraName`] from the [`naming`] module — this alias
/// is provided for consistency with the template structure.
pub type FamilyName = BravuraName;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_glyphs_is_not_empty() {
        let count = Bravura::glyph_count();
        assert!(count > 0, "Bravura should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_start_with_bravura() {
        for (_, name) in Bravura::all_glyphs() {
            assert!(name.as_ref().starts_with("bravura-"), "glyph name '{name}' should start with 'bravura-'");
        }
    }

    #[test]
    fn all_glyphs_names_are_lowercase() {
        for (_, name) in Bravura::all_glyphs() {
            assert_eq!(name.as_ref(), name.as_ref().to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_glyphs_names_contain_no_underscores() {
        for (_, name) in Bravura::all_glyphs() {
            assert!(!name.as_ref().contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_glyphs_codepoints_are_unique() {
        let codepoints: Vec<char> = Bravura::all_glyphs().map(|(c, _)| c.as_char()).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }

    #[test]
    fn all_glyphs_codepoints_in_smufl_pua() {
        for (c, _) in Bravura::all_glyphs() {
            let cp = c.as_char() as u32;
            assert!(
                (0xE000..=0xF8FF).contains(&cp),
                "codepoint U+{cp:04X} should be in the SMuFL Private Use Area (U+E000–U+F8FF)"
            );
        }
    }
}
