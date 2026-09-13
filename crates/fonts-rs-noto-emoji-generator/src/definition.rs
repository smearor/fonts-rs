//! `NotoEmojiDefinition` — `FontDefinition` implementation for Noto Emoji.
//!
//! This module implements the generic [`FontDefinition`] trait for the Noto Emoji
//! font family, allowing the `FontDefinition::export_glyphs` pipeline to be used.
//!
//! Note: Noto Emoji glyph names in the font are `uniXXXX` format (PostScript
//! auto-generated), so the build script uses [`export_glyphs_by_name_map`] with
//! CLDR annotation data for semantic names instead of the generic pipeline.
//! This definition provides the constants and type information for the family.

use fonts_rs_generator::FontDefinition;
use fonts_rs_model::FontFamily;
use fonts_rs_model::sealed;

/// Marker enum for the Noto Emoji font family.
///
/// Used as the `FontFamily` phantom type parameter. Zero-sized, exists
/// only at the type level for compile-time type safety.
pub enum NotoEmoji {}

impl sealed::Sealed for NotoEmoji {}

impl FontFamily for NotoEmoji {}

/// TTF font file name (relative to `resources/`).
pub const FONT_FILE: &str = "NotoEmoji-Regular.ttf";

/// CLDR annotations file name (relative to `resources/`).
pub const ANNOTATIONS_FILE: &str = "metadata/annotations.json";

/// Unicode emoji-test.txt file name (relative to `resources/`).
pub const EMOJI_TEST_FILE: &str = "metadata/emoji-test.txt";

/// Unicode codepoint ranges for emoji:
/// - BMP: Misc symbols (U+2600–U+26FF), Dingbats (U+2700–U+27BF)
/// - SMP: Supplemental symbols and pictographs (U+1F300–U+1FAFF)
pub const EMOJI_RANGES: &[(u32, u32)] = &[
    (0x2600, 0x26FF),
    (0x2700, 0x27BF),
    (0x1F300, 0x1F5FF),
    (0x1F600, 0x1F64F),
    (0x1F680, 0x1F6FF),
    (0x1F700, 0x1F77F),
    (0x1F780, 0x1F7FF),
    (0x1F900, 0x1F9FF),
    (0x1FA70, 0x1FAFF),
];

/// GResource prefix for Noto Emoji.
pub const GRESOURCE_PREFIX: &str = "/io/smearor/fonts/noto_emoji";

/// Glyph name prefix for Noto Emoji.
pub const GLYPH_PREFIX: &str = "noto-emoji";

/// Build-time configuration for the Noto Emoji font family.
///
/// Implements [`FontDefinition`] to plug into the generic
/// [`FontDefinition::export_glyphs`] pipeline.
///
/// Note: The build script typically uses [`export_glyphs_by_name_map`] with
/// CLDR annotation data instead of the generic pipeline, because Noto Emoji
/// glyph names are `uniXXXX` format and require CLDR lookup for semantic names.
/// This definition still provides the constants and type information.
pub struct NotoEmojiDefinition;

impl FontDefinition for NotoEmojiDefinition {
    const GRESOURCE_PREFIX: &'static str = GRESOURCE_PREFIX;

    const ICONS_CONTEXT: &'static str = "emoji";

    const CODEPOINT_RANGES: &[(u32, u32)] = EMOJI_RANGES;

    type Name = String;

    type Family = NotoEmoji;

    fn normalize_name(raw_glyph_name: &str) -> Option<Self::Name> {
        // Noto Emoji glyph names are `uniXXXX` format and require CLDR
        // annotation data for semantic names. The build script uses
        // `export_glyphs_by_name_map` with a pre-built name map instead.
        // This implementation is provided for completeness but returns
        // `None` for `uni`-prefixed names.
        if raw_glyph_name.starts_with("uni") || raw_glyph_name.starts_with("u") || raw_glyph_name.starts_with('.') {
            return None;
        }
        Some(raw_glyph_name.to_string())
    }

    fn should_skip(raw_glyph_name: &str) -> bool {
        // Skip auto-generated PostScript names; the build script uses
        // `export_glyphs_by_name_map` which bypasses this filter.
        raw_glyph_name.starts_with('.') || raw_glyph_name.starts_with("uni") || raw_glyph_name.starts_with("u")
    }
}
