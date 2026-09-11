//! GResource XML manifest generation.

use std::fs;
use std::path::Path;

use fonts_rs_model::GlyphEntry;

use crate::font_definition::FontDefinition;

/// Generate the GResource XML manifest file for a font family.
///
/// Creates an `icons.gresource.xml` listing all glyph SVG files under
/// the font family's GResource prefix.
pub fn generate_gresource_xml<F: FontDefinition>(entries: &[GlyphEntry<F::Name>], output_path: &Path) -> std::io::Result<()> {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<gresources>\n");
    xml.push_str(&format!("  <gresource prefix=\"{}\">\n", F::GRESOURCE_PREFIX));

    let mut sorted: Vec<&GlyphEntry<F::Name>> = entries.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));

    for entry in sorted {
        xml.push_str(&format!("    <file>scalable/{}/{}.svg</file>\n", F::ICONS_CONTEXT, entry.name.as_ref()));
    }

    xml.push_str("  </gresource>\n");
    xml.push_str("</gresources>\n");

    fs::write(output_path, xml)?;
    Ok(())
}
