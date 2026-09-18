//! Libre Barcode Code 39 font integration for GTK 4.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the Libre Barcode Code 39 font family.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_barcode_code39::Code39;
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_barcode_code39::GtkFont;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! Code39::register_glyphs().unwrap();
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_barcode_code39::Code39Name;
//! use fonts_rs_barcode_code39::GlyphNameExt;
//!
//! let name = Code39Name::new("uni0030".to_string());
//! let codepoint = name.codepoint();
//! ```

pub mod codepoint_map;
pub mod constants;
pub mod definition;
mod family;

#[cfg(feature = "gtk")]
pub mod gtk;

#[cfg(feature = "render")]
pub mod fonts;

use fonts_rs_model::CodePoint;

// Re-export key types
pub use definition::Code39;
pub use definition::Code39Definition;
pub use definition::Code39Name;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;

/// Extension trait adding codepoint lookup to `GlyphName<Code39>`.
#[deprecated(since = "0.3.0", note = "use `Code39::codepoint(name)` from the `FontFamily` trait instead")]
pub trait GlyphNameExt {
    /// Resolves this glyph name to its Unicode [`CodePoint`].
    ///
    /// Returns `None` if the name is not found in the codepoint map.
    fn codepoint(&self) -> Option<CodePoint>;
}

#[allow(deprecated)]
impl GlyphNameExt for Code39Name {
    fn codepoint(&self) -> Option<CodePoint> {
        let key = self.as_ref();
        codepoint_map::REVERSE_GLYPHS.get(key).copied().map(CodePoint::from)
    }
}

/// Returns an iterator over all known glyphs, yielding
/// `(char, &'static str)` pairs of codepoint and glyph name.
#[deprecated(since = "0.3.0", note = "use `Code39::all_glyphs()` from the `FontFamily` trait instead")]
pub fn all_glyphs() -> impl Iterator<Item = (char, &'static str)> {
    codepoint_map::GLYPHS.entries().map(|(c, name)| (*c, *name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use fonts_rs_model::FontFamily;

    #[test]
    fn all_glyphs_is_not_empty() {
        let count = Code39::all_glyphs().count();
        assert!(count > 0, "Code 39 should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_are_lowercase() {
        for (_, name) in Code39::all_glyphs() {
            assert_eq!(*name, name.to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_glyphs_names_contain_no_underscores() {
        for (_, name) in Code39::all_glyphs() {
            assert!(!name.contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_glyphs_codepoints_are_unique() {
        let codepoints: Vec<char> = Code39::all_glyphs().map(|(c, _)| c).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }
}
