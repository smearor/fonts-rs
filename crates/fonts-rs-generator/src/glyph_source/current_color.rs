//! `currentColor` normalization for symbolic-icon recoloring.

use crate::glyph_source::error::GlyphSourceError;
use crate::glyph_source::paint_target::PaintTarget;
use crate::glyph_source::post_processor::SvgPostProcessor;
use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;

/// Replaces explicit `stroke`/`fill` values with `currentColor` so GTK
/// symbolic icons recolor via CSS `color` / `-gtk-icon-palette`.
///
/// XML-aware via `quick_xml`: only paint attributes on element tags are
/// rewritten — comments, CDATA and metadata pass through untouched.
/// Opt-in per family; multi-color sets (Blobmoji) simply don't register
/// the hook.
pub struct CurrentColor {
    /// Which paint attributes to rewrite.
    target: PaintTarget,
}

impl CurrentColor {
    /// Creates a `CurrentColor` processor for the given paint target.
    pub fn new(target: PaintTarget) -> Self {
        Self { target }
    }
}

impl SvgPostProcessor for CurrentColor {
    fn process(&self, svg: String) -> Result<String, GlyphSourceError> {
        let mut reader = Reader::from_str(&svg);
        let mut writer = Writer::new(Vec::new());
        loop {
            match reader.read_event().map_err(|error| GlyphSourceError::PostProcess(error.to_string()))? {
                Event::Start(element) => writer.write_event(Event::Start(self.rewrite_paints(&element)?))?,
                Event::Empty(element) => writer.write_event(Event::Empty(self.rewrite_paints(&element)?))?,
                Event::Eof => break,
                event => writer.write_event(event)?,
            }
        }
        String::from_utf8(writer.into_inner()).map_err(|error| GlyphSourceError::PostProcess(error.to_string()))
    }
}

impl CurrentColor {
    /// Rebuilds one element tag with paint attributes rewritten per
    /// `target`. All other attributes pass through unchanged.
    fn rewrite_paints(&self, element: &BytesStart) -> Result<BytesStart<'static>, GlyphSourceError> {
        let mut rewritten = element.to_owned();
        rewritten.clear_attributes();
        for attribute in element.attributes().with_checks(false) {
            let attribute = attribute.map_err(|error| GlyphSourceError::PostProcess(error.to_string()))?;
            let key = attribute.key.as_ref().to_string();
            let value = attribute.value.to_string();
            let value = match key.as_str() {
                "stroke" if self.target.rewrites_stroke() && is_rewritable_paint(&value) => "currentColor".to_string(),
                "fill" if self.target.rewrites_fill() && is_rewritable_paint(&value) => "currentColor".to_string(),
                "style" => self.rewrite_style(&value),
                _ => value,
            };
            rewritten.push_attribute((key.as_str(), value.as_str()));
        }
        Ok(rewritten)
    }

    /// Rewrites `fill:`/`stroke:` declarations inside a `style` value,
    /// leaving all other declarations untouched.
    fn rewrite_style(&self, style: &str) -> String {
        style
            .split(';')
            .map(|declaration| {
                let Some((property, value)) = declaration.split_once(':') else {
                    return declaration.to_string();
                };
                let targeted = (property.trim() == "stroke" && self.target.rewrites_stroke()) || (property.trim() == "fill" && self.target.rewrites_fill());
                if targeted && is_rewritable_paint(value) {
                    format!("{}:currentColor", property.trim())
                } else {
                    declaration.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(";")
    }
}

/// Returns `true` if `value` is a plain color that may be rewritten.
///
/// `none` is structural (e.g. `fill="none"` on stroke-based sets),
/// `url(…)` references gradients/patterns, `currentColor` and
/// `inherit` are already correct.
fn is_rewritable_paint(value: &str) -> bool {
    let value = value.trim();
    !matches!(value, "none" | "currentColor" | "inherit") && !value.starts_with("url(")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_stroke_attribute() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg"><path stroke="#000" d="M0 0"/></svg>"##.to_string();
        let result = CurrentColor::new(PaintTarget::Stroke).process(svg).expect("processed");
        assert!(result.contains("stroke=\"currentColor\""));
    }

    #[test]
    fn preserves_fill_none_on_stroke_sets() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" fill="none"><path stroke="#333"/></svg>"##.to_string();
        let result = CurrentColor::new(PaintTarget::Stroke).process(svg).expect("processed");
        assert!(result.contains("fill=\"none\""));
        assert!(result.contains("stroke=\"currentColor\""));
    }

    #[test]
    fn rewrites_fill_only_when_targeted() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg"><path fill="#fff" stroke="#000"/></svg>"##.to_string();
        let result = CurrentColor::new(PaintTarget::Fill).process(svg).expect("processed");
        assert!(result.contains("fill=\"currentColor\""));
        assert!(result.contains("stroke=\"#000\""));
    }

    #[test]
    fn rewrites_style_declarations() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg"><path style="fill: #000; stroke-width: 2"/></svg>"##.to_string();
        let result = CurrentColor::new(PaintTarget::Fill).process(svg).expect("processed");
        assert!(result.contains("fill:currentColor"));
        assert!(result.contains("stroke-width: 2"));
    }

    #[test]
    fn preserves_url_references_and_comments() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg"><!-- fill="#000" --><path fill="url(#g)"/></svg>"##.to_string();
        let result = CurrentColor::new(PaintTarget::Both).process(svg).expect("processed");
        assert!(result.contains("<!-- fill=\"#000\" -->"));
        assert!(result.contains("fill=\"url(#g)\""));
    }
}
