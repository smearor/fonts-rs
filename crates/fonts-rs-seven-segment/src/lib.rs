//! DSEG7 seven-segment display font integration for GTK 4.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the DSEG7 font family — a seven-segment display font
//! suitable for retro-style digital readouts.
//!
//! The active variant (style + weight) is selected at build time via
//! Cargo features (e.g. `classic-regular`, `modern-mini-bold`).
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_seven_segment::SevenSegment;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = SevenSegment::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_seven_segment::SevenSegment;
//!
//! let codepoint = SevenSegment::codepoint("zero");
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
pub use definition::SevenSegmentConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::SevenSegment;
pub use naming::SevenSegmentName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`SevenSegmentName`] from the [`naming`] module — this alias
/// is provided for consistency with the template structure.
pub type FamilyName = SevenSegmentName;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_glyphs_is_not_empty() {
        let count = SevenSegment::glyph_count();
        assert!(count > 0, "DSEG7 should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_start_with_dseg7() {
        for (_, name) in SevenSegment::all_glyphs() {
            assert!(name.as_ref().starts_with("dseg7-"), "glyph name '{name}' should start with 'dseg7-'");
        }
    }

    #[test]
    fn all_glyphs_names_are_lowercase() {
        for (_, name) in SevenSegment::all_glyphs() {
            assert_eq!(name.as_ref(), name.as_ref().to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_glyphs_names_contain_no_underscores() {
        for (_, name) in SevenSegment::all_glyphs() {
            assert!(!name.as_ref().contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_glyphs_codepoints_are_unique() {
        let codepoints: Vec<char> = SevenSegment::all_glyphs().map(|(c, _)| c.as_char()).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }
}
