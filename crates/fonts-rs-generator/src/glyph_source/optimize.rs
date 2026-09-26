//! Rust-native SVG optimization pass.

use crate::glyph_source::error::GlyphSourceError;
use crate::glyph_source::post_processor::SvgPostProcessor;
use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::Event;

/// Rust-native optimization pass: strips comments, processing
/// instructions, and whitespace-only text nodes.
///
/// Deterministic output for hash-based change detection; no external
/// toolchain. Numeric precision rounding is intentionally left out —
/// path-data rewriting is error-prone and upstream SVGs are usually
/// already optimized.
pub struct Optimize;

impl SvgPostProcessor for Optimize {
    fn process(&self, svg: String) -> Result<String, GlyphSourceError> {
        let mut reader = Reader::from_str(&svg);
        let mut writer = Writer::new(Vec::new());
        loop {
            match reader.read_event().map_err(|error| GlyphSourceError::PostProcess(error.to_string()))? {
                Event::Eof => break,
                Event::Comment(_) | Event::PI(_) => continue,
                Event::Text(text) if text.trim().is_empty() => continue,
                event => writer.write_event(event)?,
            }
        }
        String::from_utf8(writer.into_inner()).map_err(|error| GlyphSourceError::PostProcess(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_comments_and_processing_instructions() {
        let svg = "<?xml version=\"1.0\"?><!-- comment --><svg xmlns=\"http://www.w3.org/2000/svg\"/>".to_string();
        let result = Optimize.process(svg).expect("processed");
        assert!(!result.contains("<!--"));
        assert!(result.contains("<svg"));
    }

    #[test]
    fn strips_whitespace_only_text_nodes() {
        let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\">\n  <path d=\"M0 0\"/>\n</svg>".to_string();
        let result = Optimize.process(svg).expect("processed");
        assert_eq!(result, "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\"/></svg>");
    }

    #[test]
    fn preserves_text_content() {
        let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\"><text>hi</text></svg>".to_string();
        let result = Optimize.process(svg).expect("processed");
        assert!(result.contains("<text>hi</text>"));
    }
}
