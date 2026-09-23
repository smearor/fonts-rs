//! Redacted placeholder font integration for GTK 4.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the Redacted font family — a placeholder/blocking text font
//! for wireframes and prototypes.
//!
//! The active variant is selected at build time via Cargo features:
//! - `redacted` — block bars (Regular only)
//! - `script-light`, `script-regular`, `script-bold` — scribble style
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_redacted::Redacted;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = Redacted::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_redacted::Redacted;
//!
//! let codepoint = Redacted::codepoint("a");
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
pub use definition::RedactedConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Redacted;
pub use naming::RedactedName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`RedactedName`] from the [`naming`] module — this alias
/// is provided for consistency with the template structure.
pub type FamilyName = RedactedName;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_glyphs_is_not_empty() {
        let count = Redacted::glyph_count();
        assert!(count > 0, "Redacted should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_are_lowercase() {
        for (_, name) in Redacted::all_glyphs() {
            assert_eq!(name.as_ref(), name.as_ref().to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_glyphs_names_contain_no_underscores() {
        for (_, name) in Redacted::all_glyphs() {
            assert!(!name.as_ref().contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_glyphs_codepoints_are_unique() {
        let codepoints: Vec<char> = Redacted::all_glyphs().map(|(c, _)| c.as_char()).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }
}
