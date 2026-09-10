//! GResource XML manifest generation.

use std::fs;
use std::path::Path;

use nerd_fonts_model::GRESOURCE_PREFIX;
use nerd_fonts_model::IconEntry;

/// Generate the GResource XML manifest file.
///
/// Creates an `icons.gresource.xml` listing all icon SVG files under
/// the [`GRESOURCE_PREFIX`] resource prefix.
pub fn generate_gresource_xml(icons: &[IconEntry], output_path: &Path) -> std::io::Result<()> {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<gresources>\n");
    xml.push_str(&format!("  <gresource prefix=\"{}\">\n", GRESOURCE_PREFIX));

    let mut sorted: Vec<&IconEntry> = icons.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));

    for icon in sorted {
        xml.push_str(&format!("    <file>icons/{}.svg</file>\n", icon.name));
    }

    xml.push_str("  </gresource>\n");
    xml.push_str("</gresources>\n");

    fs::write(output_path, xml)?;
    Ok(())
}
