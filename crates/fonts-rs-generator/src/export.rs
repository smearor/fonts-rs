//! Generic glyph export from TTF/OTF fonts as GTK 4 symbolic SVG icons.

use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;

use skrifa::GlyphId;
use skrifa::MetadataProvider;

use fonts_rs_model::GlyphEntry;
use fonts_rs_model::ResourcePath;

use crate::font::Font;
use crate::font_definition::FontDefinition;
use crate::gresource::generate_gresource_xml;
use crate::svg::EMPTY_SVG;
use crate::svg::glyph_to_svg;

/// Export all glyphs from a TTF/OTF font file as GTK 4 symbolic SVG icons.
///
/// This is the generic version of the export pipeline, parameterized by
/// a [`FontDefinition`] implementation. It uses the trait to normalize
/// glyph names, construct resource paths, and generate GResource XML for
/// any font family.
///
/// Generates:
/// - `<output_dir>/scalable/{context}/*.svg` — one SVG file per glyph
/// - `<output_dir>/metadata.json` — glyph metadata (name, codepoint, file path)
/// - `<output_dir>/icons.gresource.xml` — GResource bundle manifest
///
/// # Type Parameters
///
/// - `F` — The font family's `FontDefinition` implementation.
///
/// # Arguments
///
/// - `font_path` — Path to the TTF/OTF font file
/// - `output_dir` — Root output directory (typically `resources/`)
///
/// # Returns
///
/// The number of exported glyphs on success, or an `io::Error` on failure.
pub fn export_glyphs<F: FontDefinition>(font_path: &Path, output_dir: &Path) -> std::io::Result<usize> {
    let font_data = fs::read(font_path)?;
    let font = Font::from_data(&font_data).map_err(|e| std::io::Error::other(format!("Failed to parse font: {e}")))?;

    // SVG output directory follows GTK 4 IconTheme convention:
    // {output_dir}/scalable/{context}/
    let icons_dir = output_dir.join("scalable").join(F::ICONS_CONTEXT);
    fs::create_dir_all(&icons_dir)?;

    let reverse_cmap = font.build_reverse_cmap::<F>();

    let mut entries: Vec<GlyphEntry<F::Name>> = Vec::new();
    let mut seen_names: HashSet<String> = HashSet::new();

    let num_glyphs = font.glyph_names().num_glyphs();

    for glyph_index in 0..num_glyphs {
        let glyph_id = GlyphId::new(glyph_index);

        let name = match font.glyph_names().get(glyph_id) {
            Some(n) if !n.as_str().is_empty() => n.to_string(),
            _ => continue,
        };

        // Skip invalid glyph names (same filters as Nerd Fonts)
        if name.starts_with('.') || name.starts_with("uni") || name.starts_with("u") {
            continue;
        }

        let Some(glyph_name) = F::normalize_name(&name) else {
            continue;
        };

        // Deduplicate
        if seen_names.contains(glyph_name.as_ref()) {
            continue;
        }
        seen_names.insert(glyph_name.as_ref().to_string());

        let codepoint = reverse_cmap.get(&glyph_id).copied();

        let svg = match glyph_to_svg(&font, glyph_id) {
            Some(svg) => svg,
            None => EMPTY_SVG.to_string(),
        };

        let filename = icons_dir.join(format!("{}.svg", glyph_name.as_ref()));
        let mut file = fs::File::create(&filename)?;
        file.write_all(svg.as_bytes())?;

        // GResource path follows GTK 4 GtkIconTheme convention:
        // {prefix}/scalable/{context}/{name}.svg
        let resource_prefix = format!("{}/scalable/{}", F::GRESOURCE_PREFIX, F::ICONS_CONTEXT);
        let resource_path = ResourcePath::from_name(&resource_prefix, glyph_name.as_ref());

        entries.push(GlyphEntry {
            code: codepoint,
            name: glyph_name.clone(),
            file: Path::new(&format!("resources/scalable/{}/{}.svg", F::ICONS_CONTEXT, glyph_name.as_ref())).to_path_buf(),
            resource_path,
        });
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    let json_path = output_dir.join("metadata.json");
    let json = serde_json::to_string_pretty(&entries).map_err(std::io::Error::other)?;
    fs::write(&json_path, json)?;

    let xml_path = output_dir.join("icons.gresource.xml");
    generate_gresource_xml::<F>(&entries, &xml_path)?;

    Ok(entries.len())
}
