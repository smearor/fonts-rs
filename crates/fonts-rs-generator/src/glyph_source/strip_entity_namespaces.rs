//! Post-processor removing DOCTYPE entity namespaces from SVG artwork.

use crate::glyph_source::error::GlyphSourceError;
use crate::glyph_source::post_processor::SvgPostProcessor;
use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::name::QName;
use std::collections::HashSet;

/// Strips DOCTYPE entity declarations and the namespaced content that
/// relies on them.
///
/// Adobe Illustrator exports declare namespace URIs via DOCTYPE
/// entities (`xmlns:i="&ns_ai;"`) and embed metadata such as
/// `<i:aipgf>` PGIF blobs or `i:extraneous` attributes. librsvg rejects
/// entity declarations outright, and composing the SVG into a larger
/// document drops the DOCTYPE - either way the prefixes end up
/// unbound and parsing fails.
///
/// The processor removes the DOCTYPE, every `xmlns:*` attribute whose
/// value is an entity reference, and all elements and attributes using
/// the removed prefixes. Files without a DOCTYPE pass through
/// unchanged.
pub struct StripEntityNamespaces;

impl SvgPostProcessor for StripEntityNamespaces {
    fn process(&self, svg: String) -> Result<String, GlyphSourceError> {
        if !svg.contains("<!DOCTYPE") {
            return Ok(svg);
        }
        let svg = strip_doctype(&svg);

        let mut reader = Reader::from_str(&svg);
        let mut writer = Writer::new(Vec::new());
        let mut removed: HashSet<String> = HashSet::new();

        loop {
            match reader.read_event().map_err(|error| GlyphSourceError::PostProcess(error.to_string()))? {
                Event::Eof => break,
                Event::DocType(_) => continue,
                Event::Start(element) => {
                    let name = element.name().as_ref().to_owned();
                    let Some(element) = filter_element(element, &mut removed) else {
                        reader
                            .read_to_end(QName(name.as_str()))
                            .map_err(|error| GlyphSourceError::PostProcess(error.to_string()))?;
                        continue;
                    };
                    writer
                        .write_event(Event::Start(element))
                        .map_err(|error| GlyphSourceError::PostProcess(error.to_string()))?;
                }
                Event::Empty(element) => {
                    if let Some(element) = filter_element(element, &mut removed) {
                        writer
                            .write_event(Event::Empty(element))
                            .map_err(|error| GlyphSourceError::PostProcess(error.to_string()))?;
                    }
                }
                event => writer.write_event(event).map_err(|error| GlyphSourceError::PostProcess(error.to_string()))?,
            }
        }
        String::from_utf8(writer.into_inner()).map_err(|error| GlyphSourceError::PostProcess(error.to_string()))
    }
}

/// Removes the `<!DOCTYPE …>` block, including an internal entity
/// subset (`[ <!ENTITY …> ]>`), from the document prolog.
fn strip_doctype(svg: &str) -> String {
    let Some(start) = svg.find("<!DOCTYPE") else {
        return svg.to_string();
    };
    let rest = &svg[start..];
    let tag_end = rest.find('>');
    let subset_start = rest.find('[');
    let end = match (subset_start, tag_end) {
        (Some(subset), Some(tag)) if subset < tag => rest.find("]>").map(|i| i + 2),
        _ => tag_end.map(|i| i + 1),
    };
    let Some(end) = end else {
        return svg.to_string();
    };
    format!("{}{}", &svg[..start], &svg[start + end..])
}

/// Returns the namespace prefix of a qualified name, if any.
fn prefix_of(name: &str) -> Option<&str> {
    name.find(':').map(|pos| &name[..pos])
}

/// Filters one element's attributes, collecting prefixes declared via
/// entity references into `removed` and dropping attributes that use
/// removed prefixes.
///
/// Returns `None` when the element's own name uses a removed prefix -
/// the caller then skips the element (and its subtree for
/// [`Event::Start`]).
fn filter_element(element: BytesStart<'_>, removed: &mut HashSet<String>) -> Option<BytesStart<'static>> {
    let name = element.name().as_ref().to_owned();
    let mut kept = Vec::new();
    for attribute in element.attributes().flatten() {
        let key = attribute.key.as_ref();
        if let Some(prefix) = key.strip_prefix("xmlns:")
            && attribute.value.contains('&')
        {
            removed.insert(prefix.to_owned());
            continue;
        }
        if prefix_of(key).is_some_and(|prefix| removed.contains(prefix)) {
            continue;
        }
        kept.push(attribute);
    }
    if prefix_of(&name).is_some_and(|prefix| removed.contains(prefix)) {
        return None;
    }
    let mut filtered = BytesStart::new(&name);
    filtered.extend_attributes(kept);
    Some(filtered.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Illustrator-style document: DOCTYPE entities, prefixed metadata
    /// attributes, and an `<i:aipgf>` blob.
    #[test]
    fn strips_entity_namespaces_and_their_users() {
        let svg = concat!(
            "<?xml version=\"1.0\"?>",
            "<!DOCTYPE svg [ <!ENTITY ns_ai \"http://ns.adobe.com/AdobeIllustrator/10.0/\"> ]>",
            "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:i=\"&ns_ai;\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 128 128\">",
            "<g i:extraneous=\"self\"><path fill=\"#fff\" d=\"M0 0\"/></g>",
            "<i:aipgf i:pgfEncoding=\"z\"><![CDATA[binary]]></i:aipgf>",
            "</svg>"
        )
        .to_string();
        let result = StripEntityNamespaces.process(svg).expect("processed");
        assert!(!result.contains("DOCTYPE"), "doctype removed: {result}");
        assert!(!result.contains("ns_ai"), "entity declaration removed: {result}");
        assert!(!result.contains("i:"), "prefixed content removed: {result}");
        assert!(result.contains("xmlns:xlink"), "real namespace kept: {result}");
        assert!(result.contains("<path"), "artwork kept: {result}");
    }

    /// Documents without a DOCTYPE pass through unchanged.
    #[test]
    fn clean_documents_pass_through() {
        let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\"/></svg>".to_string();
        assert_eq!(StripEntityNamespaces.process(svg.clone()).expect("processed"), svg);
    }
}
