//! Reverse codepoint map construction from font glyph tables.

use std::collections::HashMap;

use skrifa::FontRef;
use skrifa::GlyphId;
use skrifa::MetadataProvider;

use fonts_rs_model::CodePoint;

/// Build a reverse cmap (GlyphId -> CodePoint) by probing Unicode codepoints.
///
/// Scans the entire BMP (U+0000-U+FFFF) plus the supplementary PUA
/// (U+F0001-U+10FFFF) to cover all Nerd Font codepoint ranges.
/// At ~16ns per `glyph_index` lookup, this takes ~17ms total.
pub fn build_reverse_cmap(font: &FontRef) -> HashMap<GlyphId, CodePoint> {
    let mut map = probe_range(font, 0x0000, 0xFFFF);
    map.extend(probe_range(font, 0xF0001, 0x10FFFF));
    map
}

/// Probe a contiguous range of Unicode codepoints and return matching glyphs.
fn probe_range(font: &FontRef, start: u32, end: u32) -> HashMap<GlyphId, CodePoint> {
    let charmap = font.charmap();
    let mut map = HashMap::new();
    for codepoint in start..=end {
        if let Some(ch) = char::from_u32(codepoint)
            && let Some(glyph_id) = charmap.map(ch)
        {
            map.insert(glyph_id, CodePoint::from(ch));
        }
    }
    map
}
