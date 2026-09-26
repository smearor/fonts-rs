//! viewBox normalization for consistent icon canvases.

use crate::glyph_source::error::GlyphSourceError;
use crate::glyph_source::post_processor::SvgPostProcessor;
use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;

/// Ensures the root `<svg>` element carries a `viewBox` and no fixed
/// `width`/`height`, so glyphs scale to the GTK icon size.
///
/// Synthesis order: existing `viewBox` → derived from `width`/`height`
/// → the configured default. Only the root element is rewritten; all
/// other content passes through unchanged.
pub struct NormalizeViewBox {
    /// Fallback canvas when neither `viewBox` nor `width`/`height`
    /// are present (e.g. `"0 0 24 24"`).
    default_view_box: String,
}

impl NormalizeViewBox {
    /// Creates a normalizer with the given fallback viewBox.
    pub fn new(default_view_box: impl Into<String>) -> Self {
        Self {
            default_view_box: default_view_box.into(),
        }
    }
}

impl SvgPostProcessor for NormalizeViewBox {
    fn process(&self, svg: String) -> Result<String, GlyphSourceError> {
        let mut reader = Reader::from_str(&svg);
        let mut writer = Writer::new(Vec::new());
        let mut root_seen = false;
        loop {
            match reader.read_event().map_err(|error| GlyphSourceError::PostProcess(error.to_string()))? {
                Event::Start(element) if !root_seen && element.local_name().as_ref() == "svg" => {
                    root_seen = true;
                    writer.write_event(Event::Start(self.normalize_root(&element)?))?;
                }
                Event::Empty(element) if !root_seen && element.local_name().as_ref() == "svg" => {
                    root_seen = true;
                    writer.write_event(Event::Empty(self.normalize_root(&element)?))?;
                }
                Event::Eof => break,
                event => writer.write_event(event)?,
            }
        }
        String::from_utf8(writer.into_inner()).map_err(|error| GlyphSourceError::PostProcess(error.to_string()))
    }
}

impl NormalizeViewBox {
    /// Rebuilds the root `<svg>` tag: drops `width`/`height`, ensures
    /// `viewBox` is present.
    fn normalize_root(&self, element: &BytesStart) -> Result<BytesStart<'static>, GlyphSourceError> {
        let mut view_box = None;
        let mut width = None;
        let mut height = None;
        let mut attributes = Vec::new();
        for attribute in element.attributes().with_checks(false) {
            let attribute = attribute.map_err(|error| GlyphSourceError::PostProcess(error.to_string()))?;
            let key = attribute.key.as_ref().to_string();
            let value = attribute.value.to_string();
            match key.as_str() {
                "viewBox" => view_box = Some(value),
                "width" => width = Some(value),
                "height" => height = Some(value),
                _ => attributes.push((key, value)),
            }
        }

        let view_box = view_box
            .or_else(|| width.zip(height).map(|(w, h)| format!("0 0 {} {}", strip_unit(&w), strip_unit(&h))))
            .unwrap_or_else(|| self.default_view_box.clone());

        let mut rewritten = element.to_owned();
        rewritten.clear_attributes();
        for (key, value) in attributes {
            rewritten.push_attribute((key.as_str(), value.as_str()));
        }
        rewritten.push_attribute(("viewBox", view_box.as_str()));
        Ok(rewritten)
    }
}

/// Strips a CSS unit suffix (`px`, `pt`, `%`, …) from a length value.
fn strip_unit(value: &str) -> &str {
    value.trim().trim_end_matches(|c: char| c.is_ascii_alphabetic() || c == '%')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_view_box_from_width_and_height() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"><path/></svg>"#.to_string();
        let result = NormalizeViewBox::new("0 0 16 16").process(svg).expect("processed");
        assert!(result.contains("viewBox=\"0 0 24 24\""));
        assert!(!result.contains("width="));
        assert!(!result.contains("height="));
    }

    #[test]
    fn keeps_existing_view_box() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="16px"/>"#.to_string();
        let result = NormalizeViewBox::new("0 0 24 24").process(svg).expect("processed");
        assert!(result.contains("viewBox=\"0 0 32 32\""));
        assert!(!result.contains("width="));
    }

    #[test]
    fn falls_back_to_default_view_box() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><path/></svg>"#.to_string();
        let result = NormalizeViewBox::new("0 0 24 24").process(svg).expect("processed");
        assert!(result.contains("viewBox=\"0 0 24 24\""));
    }

    #[test]
    fn only_rewrites_root_element() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><svg width="8" height="8"/></svg>"#.to_string();
        let result = NormalizeViewBox::new("0 0 24 24").process(svg).expect("processed");
        assert!(result.contains("width=\"8\""));
    }
}
