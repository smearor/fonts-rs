//! Reverse codepoint map construction from font glyph tables.

use std::collections::HashMap;

use nerd_fonts_model::CodePoint;

/// Build a reverse cmap (GlyphId -> CodePoint) by probing Unicode codepoints.
///
/// Nerd Fonts map glyphs to codepoints in several Unicode ranges:
/// - BMP PUA: U+E000-U+F8FF
/// - Supplementary PUA: U+F0001-U+10FFFF
/// - Miscellaneous Technical: U+23FB-U+23FE (IEC power symbols)
/// - Miscellaneous Symbols and Arrows: U+2B58
///
/// To cover all cases, we scan the entire BMP (U+0000-U+FFFF) plus
/// the supplementary PUA. At ~16ns per `glyph_index` lookup, this
/// takes ~17ms total.
pub fn build_reverse_cmap(face: &ttf_parser::Face) -> HashMap<ttf_parser::GlyphId, CodePoint> {
    let mut map = probe_range(face, 0x0000, 0xFFFF);
    map.extend(probe_range(face, 0xF0001, 0x10FFFF));
    map
}

/// Probe a contiguous range of Unicode codepoints and return matching glyphs.
fn probe_range(face: &ttf_parser::Face, start: u32, end: u32) -> HashMap<ttf_parser::GlyphId, CodePoint> {
    let mut map = HashMap::new();
    for codepoint in start..=end {
        if let Some(ch) = char::from_u32(codepoint) {
            if let Some(glyph_id) = face.glyph_index(ch) {
                map.insert(glyph_id, CodePoint::from(ch));
            }
        }
    }
    map
}
