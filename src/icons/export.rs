//! Export Nerd Font glyphs as GTK4 symbolic SVG icons.
//!
//! This module provides the core logic for extracting glyph outlines from a
//! TTF/OTF font file and generating SVG icons, `metadata.json`, and
//! `icons.gresource.xml`. It is used by `build.rs` for automatic generation
//! and is also available as a public API for library users.
//!
//! The export logic is also available as a CLI binary via
//! `cargo run --features export --bin export_icons`.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;

// IconEntry is a sibling module in both contexts:
// - Library: `icons::export` and `icons::entry` (super::entry)
// - build.rs: `export` and `entry` at crate root (super::entry)
use super::codepoint::CodePoint;
use super::entry::IconEntry;
use super::name::IconName;
use super::paths::GRESOURCE_PREFIX;
use super::resource_path::ResourcePath;
use super::svg_path_builder::SvgPathBuilder;

/// Generate the SVG content for a single glyph.
fn glyph_to_svg(face: &ttf_parser::Face, glyph_id: ttf_parser::GlyphId) -> Option<String> {
    let mut builder = SvgPathBuilder::new();
    let bbox = face.outline_glyph(glyph_id, &mut builder)?;

    let width = bbox.x_max - bbox.x_min;
    let height = bbox.y_max - bbox.y_min;

    if width <= 0 || height <= 0 {
        return None;
    }

    // Flip Y axis (font coords are bottom-up, SVG is top-down)
    let transform = format!("matrix(1 0 0 -1 0 {})", bbox.y_max);

    Some(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="{} {} {} {}"><g transform="{}"><path d="{}"/></g></svg>"#,
        width,
        height,
        bbox.x_min,
        bbox.y_min,
        width,
        height,
        transform,
        builder.path.trim()
    ))
}

/// Build a reverse cmap (GlyphId -> char) by probing Unicode codepoints.
///
/// Nerd Fonts map glyphs to codepoints in several Unicode ranges:
/// - BMP PUA: U+E000-U+F8FF
/// - Supplementary PUA: U+F0001-U+10FFFF
/// - Miscellaneous Technical: U+23FB-U+23FE (IEC power symbols)
/// - Miscellaneous Symbols and Arrows: U+2B58
///
/// To cover all cases, we scan the entire BMP (U+0000-U+FFFF) plus
/// the supplementary PUA. At ~16ns per `glyph_index` lookup, this
/// takes ~17ms total.
fn build_reverse_cmap(face: &ttf_parser::Face) -> HashMap<ttf_parser::GlyphId, char> {
    let mut map = HashMap::new();

    // Full BMP: U+0000 - U+FFFF
    for codepoint in 0..=0xFFFFu32 {
        if let Some(ch) = char::from_u32(codepoint) {
            if let Some(glyph_id) = face.glyph_index(ch) {
                map.insert(glyph_id, ch);
            }
        }
    }

    // Supplementary PUA: U+F0001 - U+10FFFF
    for codepoint in 0xF0001..=0x10FFFFu32 {
        if let Some(ch) = char::from_u32(codepoint) {
            if let Some(glyph_id) = face.glyph_index(ch) {
                map.insert(glyph_id, ch);
            }
        }
    }

    map
}

/// Generate the GResource XML manifest file.
fn generate_gresource_xml(icons: &[IconEntry], output_path: &Path) -> std::io::Result<()> {
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

/// Export all Nerd Font glyphs from a TTF/OTF font file as GTK4 symbolic SVG icons.
///
/// Generates:
/// - `<output_dir>/icons/*.svg` — one SVG file per glyph
/// - `<output_dir>/metadata.json` — icon metadata (name, codepoint, file path)
/// - `<output_dir>/icons.gresource.xml` — GResource bundle manifest
///
/// # Arguments
///
/// - `font_path` — Path to the TTF/OTF font file
/// - `output_dir` — Root output directory (typically `resources/`)
///
/// # Returns
///
/// The number of exported icons on success, or an `io::Error` on failure.
pub fn export_icons(font_path: &Path, output_dir: &Path) -> std::io::Result<usize> {
    let font_data = fs::read(font_path)?;
    let face = ttf_parser::Face::parse(&font_data, 0).map_err(|e| std::io::Error::other(format!("Failed to parse font: {e}")))?;

    let icons_dir = output_dir.join("icons");
    fs::create_dir_all(&icons_dir)?;

    let reverse_cmap = build_reverse_cmap(&face);

    let mut icons: Vec<IconEntry> = Vec::new();
    let mut seen_names: HashSet<String> = HashSet::new();

    let num_glyphs = face.number_of_glyphs();

    for glyph_id_u16 in 0..num_glyphs {
        let glyph_id = ttf_parser::GlyphId(glyph_id_u16);

        let name = match face.glyph_name(glyph_id) {
            Some(n) if !n.is_empty() => n.to_string(),
            _ => continue,
        };

        // Skip invalid glyph names (same filters as original Python script)
        if name.starts_with('.') || name.starts_with("uni") || name.starts_with("u") {
            continue;
        }

        let Some(icon_name) = IconName::from_glyph_name(&name) else {
            continue;
        };

        // Deduplicate
        if seen_names.contains(icon_name.as_ref()) {
            continue;
        }
        seen_names.insert(icon_name.as_ref().to_string());

        let codepoint = reverse_cmap.get(&glyph_id).map(|ch| CodePoint::from(*ch));

        // Generate SVG (or empty placeholder for glyphs without outline)
        let svg = match glyph_to_svg(&face, glyph_id) {
            Some(svg) => svg,
            None => {
                // Glyph has no outline (e.g. nonmarkingreturn, blank).
                // Create a minimal empty SVG so the icon name is still registered.
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="0 0 1 1"><path d=""/></svg>"#.to_string()
            }
        };

        let filename = icons_dir.join(format!("{}.svg", icon_name));
        let mut file = fs::File::create(&filename)?;
        file.write_all(svg.as_bytes())?;

        icons.push(IconEntry {
            code: codepoint,
            name: icon_name.clone(),
            file: Path::new(&format!("resources/icons/{}.svg", icon_name)).to_path_buf(),
            resource_path: ResourcePath::from(&icon_name),
        });
    }

    // Sort icons by name for stable output
    icons.sort_by(|a, b| a.name.cmp(&b.name));

    // Write metadata.json
    let json_path = output_dir.join("metadata.json");
    let json = serde_json::to_string_pretty(&icons).map_err(std::io::Error::other)?;
    fs::write(&json_path, json)?;

    // Write icons.gresource.xml
    let xml_path = output_dir.join("icons.gresource.xml");
    generate_gresource_xml(&icons, &xml_path)?;

    Ok(icons.len())
}
