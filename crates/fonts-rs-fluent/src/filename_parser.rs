//! SVG filename parser for Fluent UI System Icons.
//!
//! Parses `ic_fluent_{name}_{size}_{style}.svg` filenames. The parser is
//! stateful per slice: it validates that both the grid `size` and the
//! `style` suffix match the slice directory it is constructed for, so a
//! re-vendoring mistake (e.g. a 16px or `light` file landing in the
//! `24-regular` slice) is rejected instead of silently exported.

use fonts_rs_generator::ParsedGlyph;
use fonts_rs_generator::SvgFilenameParser;

/// Parses `ic_fluent_{name}_{size}_{style}.svg` filenames.
///
/// Yields [`ParsedGlyph::raw_name`] as the upstream snake-case icon stem
/// (e.g. `i_o_t`); the style suffix (`regular`/`filled`) is expressed by
/// the chosen [`ExportConfig`](fonts_rs_generator::ExportConfig)
/// variant, not by the glyph name itself.
pub struct FluentFilenameParser {
    /// Expected grid size in px (v1: 24).
    size: u16,
    /// Expected style suffix (`"regular"` | `"filled"`).
    style: &'static str,
}

impl FluentFilenameParser {
    /// Creates a parser for the `{style}-{size}` slice directory.
    pub const fn new(size: u16, style: &'static str) -> Self {
        Self { size, style }
    }
}

impl SvgFilenameParser for FluentFilenameParser {
    fn parse(&self, filename: &str) -> Option<ParsedGlyph> {
        let stem = filename.strip_prefix("ic_fluent_")?.strip_suffix(".svg")?;
        let (rest, style) = stem.rsplit_once('_')?;
        if style != self.style {
            return None;
        }
        let (name, size) = rest.rsplit_once('_')?;
        if name.is_empty() || size.parse::<u16>().ok()? != self.size {
            return None;
        }
        Some(ParsedGlyph {
            codepoints: None,
            raw_name: name.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::FluentFilenameParser;
    use fonts_rs_generator::SvgFilenameParser;

    /// Parser for the 24px regular slice under test.
    const PARSER: FluentFilenameParser = FluentFilenameParser::new(24, "regular");

    #[test]
    fn parses_regular_filename() {
        let glyph = PARSER.parse("ic_fluent_add_24_regular.svg").expect("should parse");
        assert_eq!(glyph.raw_name, "add");
        assert!(glyph.codepoints.is_none());
    }

    #[test]
    fn parses_snake_case_name() {
        let glyph = PARSER.parse("ic_fluent_i_o_t_24_regular.svg").expect("should parse");
        assert_eq!(glyph.raw_name, "i_o_t");
    }

    #[test]
    fn parses_name_with_digits() {
        let glyph = PARSER.parse("ic_fluent_usb_stick_24_regular.svg").expect("should parse");
        assert_eq!(glyph.raw_name, "usb_stick");
        let glyph = PARSER.parse("ic_fluent_window_dev_tools_24_regular.svg").expect("should parse");
        assert_eq!(glyph.raw_name, "window_dev_tools");
    }

    #[test]
    fn parses_filled_style_with_matching_parser() {
        let parser = FluentFilenameParser::new(24, "filled");
        let glyph = parser.parse("ic_fluent_add_24_filled.svg").expect("should parse");
        assert_eq!(glyph.raw_name, "add");
    }

    /// Wrong-style files (`filled`, `light`, `color` variants) must be
    /// rejected by a regular-slice parser.
    #[test]
    fn rejects_wrong_style() {
        assert!(PARSER.parse("ic_fluent_add_24_filled.svg").is_none());
        assert!(PARSER.parse("ic_fluent_add_24_light.svg").is_none());
        assert!(PARSER.parse("ic_fluent_add_24_color.svg").is_none());
    }

    /// Files from other grid sizes must be rejected - the 24px slice
    /// must not pick up 16/48px artwork.
    #[test]
    fn rejects_wrong_size() {
        assert!(PARSER.parse("ic_fluent_add_16_regular.svg").is_none());
        assert!(PARSER.parse("ic_fluent_add_20_regular.svg").is_none());
        assert!(PARSER.parse("ic_fluent_add_48_regular.svg").is_none());
    }

    /// Malformed filenames (missing prefix, wrong extension, missing
    /// size, empty name, non-numeric size) are rejected.
    #[test]
    fn rejects_malformed_filenames() {
        assert!(PARSER.parse("add_24_regular.svg").is_none());
        assert!(PARSER.parse("ic_fluent_add_24_regular.png").is_none());
        assert!(PARSER.parse("ic_fluent_add_regular.svg").is_none());
        assert!(PARSER.parse("ic_fluent__24_regular.svg").is_none());
        assert!(PARSER.parse("ic_fluent_add_xx_regular.svg").is_none());
    }
}
