//! `emoji_u*` filename parser for emoji SVG sets.

use crate::glyph_source::filename_parser::SvgFilenameParser;
use crate::glyph_source::parsed_glyph::ParsedGlyph;
use fonts_rs_model::CodePoint;

/// Parses `emoji_u{cp1}_{cp2}_….svg` filenames (blobmoji2, noto-emoji).
///
/// Handles multi-codepoint sequences (ZWJ emoji, skin-tone modifiers,
/// keycaps) — each `_`-separated hex segment becomes one [`CodePoint`].
/// The filename stem (without `emoji_u` prefix and `.svg` suffix)
/// becomes [`ParsedGlyph::raw_name`].
pub struct EmojiUFilenameParser;

impl SvgFilenameParser for EmojiUFilenameParser {
    fn parse(&self, filename: &str) -> Option<ParsedGlyph> {
        let stem = filename.strip_prefix("emoji_u")?.strip_suffix(".svg")?;
        let codepoints = stem
            .split('_')
            .map(|segment| u32::from_str_radix(segment, 16).ok().and_then(char::from_u32).map(CodePoint::from_char))
            .collect::<Option<Vec<_>>>()?;
        Some(ParsedGlyph {
            codepoints: Some(codepoints),
            raw_name: stem.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_codepoint() {
        let parsed = EmojiUFilenameParser.parse("emoji_u1f600.svg").expect("parsed");
        assert_eq!(parsed.raw_name, "1f600");
        let codepoints = parsed.codepoints.expect("codepoints");
        assert_eq!(codepoints.len(), 1);
        assert_eq!(codepoints[0].as_char(), '\u{1F600}');
    }

    #[test]
    fn parses_zwj_sequence() {
        let parsed = EmojiUFilenameParser.parse("emoji_u1f468_200d_1f469.svg").expect("parsed");
        let codepoints = parsed.codepoints.expect("codepoints");
        let chars: Vec<char> = codepoints.iter().map(|codepoint| codepoint.as_char()).collect();
        assert_eq!(chars, ['\u{1F468}', '\u{200D}', '\u{1F469}']);
    }

    #[test]
    fn rejects_non_emoji_filenames() {
        assert!(EmojiUFilenameParser.parse("home.svg").is_none());
        assert!(EmojiUFilenameParser.parse("emoji_u1f600.png").is_none());
        assert!(EmojiUFilenameParser.parse("emoji_u.svg").is_none());
    }

    #[test]
    fn rejects_invalid_hex_segments() {
        assert!(EmojiUFilenameParser.parse("emoji_uZZZ.svg").is_none());
        assert!(EmojiUFilenameParser.parse("emoji_u1f600_.svg").is_none());
    }
}
