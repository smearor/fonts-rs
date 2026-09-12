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
//! use fonts_rs_bravura::register_glyphs;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = fonts_rs_bravura::register_glyphs() {
//!     eprintln!("Failed to register glyphs: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_bravura::naming::BravuraName;
//! use fonts_rs_bravura::GlyphNameExt;
//!
//! let name = BravuraName::new("g-clef".to_string());
//! let codepoint = name.codepoint();
//! ```

pub mod codepoint_map;
pub mod constants;
pub mod naming;
pub mod variant;

#[cfg(feature = "render")]
pub mod fonts;

use fonts_rs_model::CodePoint;

// Re-export key types
pub use naming::BravuraName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`BravuraName`] from the [`naming`] module — this alias
/// is provided for consistency with the template structure.
pub type FamilyName = BravuraName;

/// Extension trait adding codepoint lookup to `GlyphName<Bravura>`.
///
/// Accepts either a base glyph name (e.g. `"g-clef"`) or a fully-qualified
/// name (e.g. `"bravura-g-clef"`). When a base name is given, the
/// [`GLYPH_PREFIX`](variant::GLYPH_PREFIX) is prepended automatically.
pub trait GlyphNameExt {
    /// Resolves this glyph name to its Unicode [`CodePoint`].
    ///
    /// Returns `None` if the name is not found in the codepoint map.
    fn codepoint(&self) -> Option<CodePoint>;
}

impl GlyphNameExt for BravuraName {
    fn codepoint(&self) -> Option<CodePoint> {
        let key = self.as_ref();
        if let Some(ch) = codepoint_map::REVERSE_GLYPHS.get(key).copied() {
            return Some(CodePoint::from(ch));
        }
        let full = format!("{}-{}", variant::GLYPH_PREFIX, key);
        codepoint_map::REVERSE_GLYPHS.get(&full).copied().map(CodePoint::from)
    }
}

/// Registers embedded Bravura glyphs as a GResource.
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
        assert!(count > 0, "Bravura should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_start_with_bravura() {
        for (_, name) in all_glyphs() {
            assert!(name.starts_with("bravura-"), "glyph name '{name}' should start with 'bravura-'");
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

    #[test]
    fn all_glyphs_codepoints_in_smufl_pua() {
        for (c, _) in all_glyphs() {
            let cp = c as u32;
            assert!(
                (0xE000..=0xF8FF).contains(&cp),
                "codepoint U+{cp:04X} should be in the SMuFL Private Use Area (U+E000–U+F8FF)"
            );
        }
    }
}
