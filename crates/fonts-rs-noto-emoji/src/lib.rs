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
//! use fonts_rs_noto_emoji::NotoEmoji;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = NotoEmoji::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve glyph names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_noto_emoji::NotoEmoji;
//!
//! let codepoint = NotoEmoji::codepoint("grinning-face");
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

use fonts_rs_model::CodePoint;

// Re-export key types
pub use definition::NotoEmojiConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::NotoEmoji;
pub use naming::NotoEmojiName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`NotoEmojiName`] from the [`naming`] module — this alias
/// is provided for consistency with the template structure.
pub type FamilyName = NotoEmojiName;

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
