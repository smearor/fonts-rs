//! Noto Emoji font integration for GTK 4 and pixel-drawing.
//!
//! Provides glyph name resolution, GResource registration, and codepoint
//! lookup for the Noto Emoji font family — Google's monochrome emoji font
//! covering all Unicode emoji codepoints with semantic names from CLDR
//! annotation data.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_noto_emoji::register_glyphs;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = fonts_rs_noto_emoji::register_glyphs() {
//!     eprintln!("Failed to register glyphs: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_noto_emoji::naming::NotoEmojiName;
//! use fonts_rs_noto_emoji::GlyphNameExt;
//!
//! let name = NotoEmojiName::new("grinning-face".to_string());
//! let codepoint = name.codepoint();
//! ```

pub mod codepoint_map;
pub mod constants;
pub mod naming;
pub mod variant;

#[cfg(feature = "render")]
pub mod fonts;

#[cfg(feature = "metadata")]
pub mod metadata;

use fonts_rs_model::CodePoint;

// Re-export key types
pub use naming::NotoEmojiName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`NotoEmojiName`] from the [`naming`] module — this alias
/// is provided for consistency with the template structure.
pub type FamilyName = NotoEmojiName;

/// Extension trait adding codepoint lookup to `GlyphName<NotoEmoji>`.
///
/// Accepts either a base glyph name (e.g. `"grinning-face"`) or a fully-qualified
/// name (e.g. `"noto-emoji-grinning-face"`). When a base name is given, the
/// [`GLYPH_PREFIX`](variant::GLYPH_PREFIX) is prepended automatically.
pub trait GlyphNameExt {
    /// Resolves this glyph name to its Unicode [`CodePoint`].
    ///
    /// Returns `None` if the name is not found in the codepoint map.
    fn codepoint(&self) -> Option<CodePoint>;
}

impl GlyphNameExt for NotoEmojiName {
    fn codepoint(&self) -> Option<CodePoint> {
        let key = self.as_ref();
        if let Some(ch) = codepoint_map::REVERSE_GLYPHS.get(key).copied() {
            return Some(CodePoint::from(ch));
        }
        let full = format!("{}-{}", variant::GLYPH_PREFIX, key);
        codepoint_map::REVERSE_GLYPHS.get(&full).copied().map(CodePoint::from)
    }
}

/// Registers embedded Noto Emoji glyphs as a GResource.
///
/// Must be called once before using any glyphs through GTK/GIO APIs.
/// After registration, glyphs are loadable via `Gtk::Image::from_resource`
/// using the resource path `{GRESOURCE_PREFIX}/scalable/emoji/{name}.svg`.
///
/// # Errors
///
/// Returns a [`gio::glib::Error`] if resource registration fails.
#[cfg(feature = "gtk")]
pub fn register_glyphs() -> Result<(), gio::glib::Error> {
    gio::resources_register_include!("icons.gresource")?;
    Ok(())
}

/// Returns all known Noto Emoji glyphs as `(CodePoint, glyph_name)` pairs,
/// sorted alphabetically by glyph name.
pub fn all_emoji() -> Vec<(CodePoint, String)> {
    let mut emojis: Vec<(CodePoint, String)> = codepoint_map::GLYPHS
        .entries()
        .map(|(cp, name)| (CodePoint::from(*cp), name.to_string()))
        .collect();
    emojis.sort_by(|a, b| a.1.cmp(&b.1));
    emojis
}
