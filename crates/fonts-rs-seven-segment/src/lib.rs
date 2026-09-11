//! DSEG7 Classic seven-segment display font integration for GTK 4.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the DSEG7 Classic font — a seven-segment display font
//! suitable for retro-style digital readouts.
//!
//! ## Quick Start
//!
//! ```no_run
//! use fonts_rs_seven_segment::register_glyphs;
//!
//! // Call once at startup before using any glyphs
//! register_glyphs().unwrap();
//! ```
//!
//! Then use glyph names with GTK's `Gtk::Image::from_icon_name`:
//!
//! ```no_run
//! use fonts_rs_seven_segment::naming::SevenSegmentName;
//! use fonts_rs_seven_segment::GlyphNameExt;
//!
//! let name = SevenSegmentName::new("dseg7-zero".to_string());
//! let codepoint = name.codepoint();
//! ```

pub mod codepoint_map;
pub mod constants;
pub mod naming;

#[cfg(feature = "render")]
pub mod fonts;

use fonts_rs_model::CodePoint;

// Re-export key types
pub use naming::SevenSegmentDefinition;
pub use naming::SevenSegmentName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`SevenSegmentName`] from the [`naming`] module — this alias
/// is provided for consistency with the template structure.
pub type FamilyName = SevenSegmentName;

/// Extension trait adding codepoint lookup to `GlyphName<SevenSegment>`.
pub trait GlyphNameExt {
    /// Resolves this glyph name to its Unicode [`CodePoint`].
    ///
    /// Returns `None` if the name is not found in the codepoint map.
    fn codepoint(&self) -> Option<CodePoint>;
}

impl GlyphNameExt for SevenSegmentName {
    fn codepoint(&self) -> Option<CodePoint> {
        codepoint_map::REVERSE_GLYPHS
            .get(self.as_ref())
            .copied()
            .map(CodePoint::from)
    }
}

/// Registers embedded DSEG7 glyphs as a GResource and adds the
/// resource path to the default `GtkIconTheme`.
///
/// Must be called once before using any glyphs through GTK/GIO APIs.
/// After registration, glyphs are resolvable via `Gtk::Image::from_icon_name`
/// and similar GTK 4 icon APIs.
///
/// # Errors
///
/// Returns a [`gio::glib::Error`] if resource registration fails.
#[cfg(feature = "gtk")]
pub fn register_glyphs() -> Result<(), gio::glib::Error> {
    gio::resources_register_include!("icons.gresource")?;
    Ok(())
}

/// Returns an iterator over all known glyphs, yielding
/// `(char, &'static str)` pairs of codepoint and glyph name.
pub fn all_glyphs() -> impl Iterator<Item = (char, &'static str)> {
    codepoint_map::GLYPHS.entries().map(|(c, name)| (*c, *name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_glyphs_is_not_empty() {
        let count = all_glyphs().count();
        assert!(count > 0, "DSEG7 should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_start_with_dseg7() {
        for (_, name) in all_glyphs() {
            assert!(
                name.starts_with("dseg7-"),
                "glyph name '{name}' should start with 'dseg7-'"
            );
        }
    }

    #[test]
    fn all_glyphs_names_are_lowercase() {
        for (_, name) in all_glyphs() {
            assert_eq!(*name, name.to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_glyphs_names_contain_no_underscores() {
        for (_, name) in all_glyphs() {
            assert!(!name.contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_glyphs_codepoints_are_unique() {
        let codepoints: Vec<char> = all_glyphs().map(|(c, _)| c).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }
}
