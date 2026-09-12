//! Font family marker type for Noto Emoji.
//!
//! The `NotoEmoji` marker is used as the phantom type parameter in
//! `GlyphName<NotoEmoji>`, ensuring type safety at compile time.

use fonts_rs_model::FontFamily;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying Noto Emoji in `GlyphName<NotoEmoji>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<NotoEmoji>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NotoEmoji {}

impl FontFamily for NotoEmoji {}

impl sealed::Sealed for NotoEmoji {}

/// Convenience type alias for Noto Emoji glyph names.
pub type NotoEmojiName = GlyphName<NotoEmoji>;
