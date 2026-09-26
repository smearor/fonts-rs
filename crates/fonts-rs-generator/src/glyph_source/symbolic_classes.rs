//! GTK symbolic class hints for stroke-based icon sets.
//!
//! GTK's symbolic icon recoloring ignores the `fill` and `stroke`
//! presentation attributes entirely: the painted colors are derived
//! from symbolic classes (`transparent-fill`, `foreground-stroke`,
//! ...) or, since GTK 4.20, `gtk:`-namespace attributes. Elements
//! without fill classes are filled with the foreground color, elements
//! without stroke classes are not stroked.
//!
//! Stroke-based icon sets (Lucide, Tabler outline) therefore render as
//! solid blobs when exported under `-symbolic` names without class
//! hints: their `fill="none"` is ignored and the foreground fill
//! swallows the stroked details. This processor maps each shape's
//! effective `fill`/`stroke` (including values inherited from ancestor
//! `<svg>`/`<g>` elements) onto the matching symbolic classes.

use crate::glyph_source::error::GlyphSourceError;
use crate::glyph_source::post_processor::SvgPostProcessor;
use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;

/// SVG shape elements that carry paintable fill/stroke geometry.
const SHAPE_ELEMENTS: &[&str] = &["path", "circle", "ellipse", "line", "polyline", "polygon", "rect", "text"];

/// Effective presentation paint seen at the current element.
#[derive(Default, Clone)]
struct Paint {
    /// Effective `fill` value, `None` when unset (SVG default: black).
    fill: Option<String>,
    /// Effective `stroke` value, `None` when unset (not stroked).
    stroke: Option<String>,
}

impl Paint {
    /// The parent's paints merged with this element's own
    /// `fill`/`stroke` attributes.
    fn inherited(&self, element: &BytesStart) -> Result<Paint, GlyphSourceError> {
        let mut paint = self.clone();
        for attribute in element.attributes().with_checks(false) {
            let attribute = attribute.map_err(|error| GlyphSourceError::PostProcess(error.to_string()))?;
            match attribute.key.as_ref() {
                "fill" => paint.fill = Some(attribute.value.to_string()),
                "stroke" => paint.stroke = Some(attribute.value.to_string()),
                _ => {}
            }
        }
        Ok(paint)
    }

    /// The symbolic class tokens for this paint state.
    fn classes(&self) -> Vec<&'static str> {
        let mut classes = Vec::new();
        match self.fill.as_deref() {
            Some("none") => classes.push("transparent-fill"),
            _ => classes.push("foreground-fill"),
        }
        match self.stroke.as_deref() {
            Some(stroke) if !is_unpainted(stroke) => classes.push("foreground-stroke"),
            _ => {}
        }
        classes
    }
}

/// Adds GTK symbolic classes to every shape element, derived from its
/// effective `fill`/`stroke` paint attributes.
///
/// - `fill="none"` (inherited or direct) → `transparent-fill`;
///   any other fill or no fill at all → `foreground-fill`
/// - a `stroke` other than `none`/`url(…)` → `foreground-stroke`;
///   `stroke="none"` or no stroke → no stroke class (symbolic icons
///   are "not stroked" by default)
///
/// The original attributes are kept untouched so non-symbolic
/// consumers (HTML export, `glyph_svg`, `compose_svg`) still see the
/// authored paints — the classes are inert outside GTK's symbolic
/// renderer. Existing `class` values are preserved and extended.
///
/// Limitation: `fill:`/`stroke:` declarations inside `style` attributes
/// are not interpreted; icon sets authored this way need a different
/// processor.
pub struct SymbolicClasses;

impl SvgPostProcessor for SymbolicClasses {
    fn process(&self, svg: String) -> Result<String, GlyphSourceError> {
        let mut reader = Reader::from_str(&svg);
        let mut writer = Writer::new(Vec::new());
        let mut paint_stack: Vec<Paint> = Vec::new();
        loop {
            match reader.read_event().map_err(|error| GlyphSourceError::PostProcess(error.to_string()))? {
                Event::Start(element) => {
                    let inherited = paint_stack.last().cloned().unwrap_or_default().inherited(&element)?;
                    writer.write_event(Event::Start(Self::add_classes(&element, &inherited)?))?;
                    paint_stack.push(inherited);
                }
                Event::Empty(element) => {
                    let inherited = paint_stack.last().cloned().unwrap_or_default().inherited(&element)?;
                    writer.write_event(Event::Empty(Self::add_classes(&element, &inherited)?))?;
                }
                Event::End(event) => {
                    paint_stack.pop();
                    writer.write_event(Event::End(event))?;
                }
                Event::Eof => break,
                event => writer.write_event(event)?,
            }
        }
        String::from_utf8(writer.into_inner()).map_err(|error| GlyphSourceError::PostProcess(error.to_string()))
    }
}

impl SymbolicClasses {
    /// Rebuilds one element tag with symbolic classes appended for
    /// shape elements; all other elements pass through unchanged.
    fn add_classes(element: &BytesStart, paint: &Paint) -> Result<BytesStart<'static>, GlyphSourceError> {
        let tag = element.name().as_ref().to_string();
        if !SHAPE_ELEMENTS.contains(&tag.as_str()) {
            return Ok(element.to_owned());
        }

        let classes = paint.classes();
        let mut rewritten = element.to_owned();
        rewritten.clear_attributes();
        let mut wrote_class = false;
        for attribute in element.attributes().with_checks(false) {
            let attribute = attribute.map_err(|error| GlyphSourceError::PostProcess(error.to_string()))?;
            let key = attribute.key.as_ref().to_string();
            let value = attribute.value.to_string();
            if key == "class" {
                let merged = format!("{} {}", classes.join(" "), value);
                rewritten.push_attribute(("class", merged.as_str()));
                wrote_class = true;
            } else {
                rewritten.push_attribute((key.as_str(), value.as_str()));
            }
        }
        if !wrote_class {
            rewritten.push_attribute(("class", classes.join(" ").as_str()));
        }
        Ok(rewritten)
    }
}

/// Returns `true` for `stroke` values that paint nothing.
fn is_unpainted(value: &str) -> bool {
    let value = value.trim();
    value == "none" || value == "transparent" || value.starts_with("url(")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stroke_only_shape_gets_transparent_fill_and_foreground_stroke() {
        let svg = r#"<svg fill="none" stroke="currentColor"><path d="M0 0"/></svg>"#.to_string();
        let result = SymbolicClasses.process(svg).expect("processed");
        assert!(result.contains(r#"class="transparent-fill foreground-stroke""#), "got: {result}");
        assert!(result.contains(r#"stroke="currentColor""#), "authored stroke must be kept: {result}");
    }

    #[test]
    fn fill_only_shape_gets_foreground_fill_without_stroke_class() {
        let svg = r#"<svg fill="currentColor"><rect width="4" height="4"/></svg>"#.to_string();
        let result = SymbolicClasses.process(svg).expect("processed");
        assert!(result.contains(r#"class="foreground-fill""#), "got: {result}");
        assert!(!result.contains("stroke"), "no stroke class expected: {result}");
    }

    #[test]
    fn element_attributes_override_inherited_paints() {
        let svg = r#"<svg fill="none" stroke="currentColor"><circle cx="1" cy="1" r="1" fill="currentColor" stroke="none"/></svg>"#.to_string();
        let result = SymbolicClasses.process(svg).expect("processed");
        assert!(result.contains(r#"class="foreground-fill""#), "overridden fill must win: {result}");
        assert!(!result.contains("foreground-stroke"), "stroke=\"none\" suppresses the stroke class: {result}");
    }

    #[test]
    fn existing_class_values_are_extended() {
        let svg = r#"<svg fill="none" stroke="currentColor"><path class="foo" d="M0 0"/></svg>"#.to_string();
        let result = SymbolicClasses.process(svg).expect("processed");
        assert!(result.contains(r#"class="transparent-fill foreground-stroke foo""#), "got: {result}");
    }

    #[test]
    fn non_shape_elements_pass_through() {
        let svg = r#"<svg fill="none" stroke="currentColor"><defs><clipPath id="c"/></defs></svg>"#.to_string();
        let result = SymbolicClasses.process(svg).expect("processed");
        assert!(result.contains("<clipPath"), "non-shape element must pass through: {result}");
        assert!(!result.contains("foreground-"), "non-shape element must not get classes: {result}");
    }
}
