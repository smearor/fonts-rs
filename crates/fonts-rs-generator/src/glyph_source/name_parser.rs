//! Generic `{name}.svg` filename parser.

use crate::glyph_source::filename_parser::SvgFilenameParser;
use crate::glyph_source::parsed_glyph::ParsedGlyph;

/// Parses `{name}.svg` filenames - the near-universal icon-set pattern
/// (Lucide, Heroicons, Feather, Tabler, Simple Icons, …).
///
/// The filename stem becomes [`ParsedGlyph::raw_name`]; no codepoints
/// are encoded in the filename.
pub struct NameParser;

impl SvgFilenameParser for NameParser {
    fn parse(&self, filename: &str) -> Option<ParsedGlyph> {
        let raw_name = filename.strip_suffix(".svg")?;
        if raw_name.is_empty() {
            return None;
        }
        Some(ParsedGlyph {
            codepoints: None,
            raw_name: raw_name.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_svg_filename() {
        let parsed = NameParser.parse("home.svg").expect("parsed");
        assert_eq!(parsed.raw_name, "home");
        assert_eq!(parsed.codepoints, None);
    }

    #[test]
    fn keeps_hyphens_and_underscores() {
        let parsed = NameParser.parse("arrow-up_right.svg").expect("parsed");
        assert_eq!(parsed.raw_name, "arrow-up_right");
    }

    #[test]
    fn rejects_non_svg_files() {
        assert!(NameParser.parse("README.md").is_none());
        assert!(NameParser.parse(".gitkeep").is_none());
        assert!(NameParser.parse(".svg").is_none());
    }
}
