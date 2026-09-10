//! Export Nerd Font glyphs as GTK4 symbolic SVG icons.
//!
//! This module provides the core logic for extracting glyph outlines from a
//! TTF/OTF font file and generating SVG icons, `metadata.json`, and
//! `icons.gresource.xml`. It is used by `build.rs` for automatic generation
//! and is also available as a public API for library users.
//!
//! The export logic is also available as a CLI binary via
//! `cargo run --features export --bin export_icons`.

use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;

use nerd_fonts_model::IconEntry;
use nerd_fonts_model::IconName;
use nerd_fonts_model::ResourcePath;

use crate::font::build_reverse_cmap;
use crate::gresource::generate_gresource_xml;
use crate::svg::EMPTY_SVG;
use crate::svg::glyph_to_svg;

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

        let codepoint = reverse_cmap.get(&glyph_id).copied();

        // Generate SVG (or empty placeholder for glyphs without outline)
        let svg = match glyph_to_svg(&face, glyph_id) {
            Some(svg) => svg,
            None => {
                // Glyph has no outline (e.g. nonmarkingreturn, blank).
                // Create a minimal empty SVG so the icon name is still registered.
                EMPTY_SVG.to_string()
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
