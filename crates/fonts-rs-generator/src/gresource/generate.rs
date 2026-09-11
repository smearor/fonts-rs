//! GResource XML manifest generation via serde + quick-xml.

use std::fs;
use std::path::Path;

use fonts_rs_model::GlyphEntry;
use quick_xml::se::Serializer;
use serde::Serialize;

use super::model::GResource;
use super::model::GResourceFile;
use super::model::GResources;
use crate::font_definition::FontDefinition;

/// Generate the GResource XML manifest file for a font family.
///
/// Creates an `icons.gresource.xml` listing all glyph SVG files under
/// the font family's GResource prefix.
pub fn generate_gresource_xml<F: FontDefinition>(entries: &[GlyphEntry<F::Name>], output_path: &Path) -> std::io::Result<()> {
    let mut sorted: Vec<&GlyphEntry<F::Name>> = entries.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));

    let files: Vec<GResourceFile> = sorted
        .iter()
        .map(|entry| GResourceFile {
            path: format!("scalable/{}/{}.svg", F::ICONS_CONTEXT, entry.name.as_ref()),
        })
        .collect();

    let manifest = GResources {
        gresource: GResource { prefix: F::GRESOURCE_PREFIX.to_string(), files },
    };

    let mut buffer = String::new();
    let mut serializer = Serializer::new(&mut buffer);
    serializer.indent(' ', 2);
    manifest
        .serialize(serializer)
        .map_err(|e| std::io::Error::other(format!("XML serialization failed: {e}")))?;

    let xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{buffer}");
    fs::write(output_path, xml)?;
    Ok(())
}
