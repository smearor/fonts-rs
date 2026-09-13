//! Libre Barcode EAN13 font integration for GTK 4.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the Libre Barcode EAN13 font family.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_barcode_ean13::register_glyphs;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! register_glyphs().unwrap();
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_barcode_ean13::Ean13Name;
//! use fonts_rs_barcode_ean13::GlyphNameExt;
//!
//! let name = Ean13Name::new("zero-compatibility".to_string());
//! let codepoint = name.codepoint();
//! ```

pub mod codepoint_map;
pub mod constants;
pub mod definition;

#[cfg(feature = "render")]
pub mod fonts;

use fonts_rs_generator::FontFamilyConfig;
use fonts_rs_model::CodePoint;

// Re-export key types
pub use definition::Ean13;
pub use definition::Ean13Definition;
pub use definition::Ean13Name;

/// GResource prefix for this font family.
pub const GRESOURCE_PREFIX: &str = Ean13Definition::GRESOURCE_PREFIX;

/// Extension trait adding codepoint lookup to `GlyphName<Ean13>`.
pub trait GlyphNameExt {
    /// Resolves this glyph name to its Unicode [`CodePoint`].
    ///
    /// Returns `None` if the name is not found in the codepoint map.
    fn codepoint(&self) -> Option<CodePoint>;
}

impl GlyphNameExt for Ean13Name {
    fn codepoint(&self) -> Option<CodePoint> {
        let key = self.as_ref();
        codepoint_map::REVERSE_GLYPHS.get(key).copied().map(CodePoint::from)
    }
}

/// Registers embedded EAN13 glyphs as a GResource.
///
/// Must be called once before using any glyphs through GTK/GIO APIs.
/// After registration, glyphs are loadable via `Gtk::Image::from_resource`
/// using the resource path `{GRESOURCE_PREFIX}/scalable/glyphs/{name}.svg`.
///
/// # Errors
///
/// Returns a [`gio::glib::Error`] if resource registration fails.
#[cfg(feature = "gtk")]
pub fn register_glyphs() -> Result<(), gio::glib::Error> {
    gio::resources_register_include!("icons.gresource")?;
    Ok(())
}

/// Registers the EAN13 TTF font as a GResource.
///
/// Must be called once before using the font via CSS `@font-face` with
/// `resource://` URLs.
///
/// # Errors
///
/// Returns a [`gio::glib::Error`] if resource registration fails.
#[cfg(feature = "gtk")]
pub fn register_font() -> Result<(), gio::glib::Error> {
    gio::resources_register_include!("font.gresource")?;
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
        assert!(count > 0, "EAN13 should export at least one glyph");
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
