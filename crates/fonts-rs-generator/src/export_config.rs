//! Runtime configuration for glyph export, decoupled from `FontDefinition`.
//!
//! Allows build scripts to export glyphs with variant-specific parameters
//! determined at runtime (e.g. from Cargo features) without needing a
//! separate `FontDefinition` impl per variant.

use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;

use fonts_rs_model::AxisValues;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::GlyphEntry;
use fonts_rs_model::GlyphNameMap;
use fonts_rs_model::ResourcePath;
use quick_xml::se::Serializer;
use serde::Serialize;
use skrifa::GlyphId;
use skrifa::MetadataProvider;
use skrifa::instance::LocationRef;

use crate::font::Font;
use crate::font_definition::normalize_to_kebab;
use crate::gresource::model::GResource;
use crate::gresource::model::GResourceFile;
use crate::gresource::model::GResources;
use crate::svg::EMPTY_SVG;

/// Runtime configuration for glyph export, decoupled from `FontDefinition`.
///
/// Allows build scripts to export glyphs with variant-specific parameters
/// determined at runtime (e.g. from Cargo features) without needing a
/// separate `FontDefinition` impl per variant.
pub struct ExportConfig {
    /// GResource prefix, e.g. `/io/smearor/fonts/seven_segment/classic_regular`.
    pub gresource_prefix: String,
    /// Icon context subdirectory, e.g. `glyphs`.
    pub icons_context: String,
    /// Glyph name prefix, e.g. `dseg7-classic-regular`.
    pub glyph_name_prefix: String,
    /// Unicode codepoint ranges to probe.
    pub codepoint_ranges: &'static [CodePointRange],
    /// Variable font axis settings in user space (e.g. `wght=700.0, ROND=50.0`).
    ///
    /// Empty for non-variable fonts (renders at default location).
    pub axes: AxisValues,
    /// Custom glyph name filter, returning `true` if the glyph should be exported.
    ///
    /// If `None`, uses [`default_name_filter`] which skips names starting with
    /// `.`, `uni`, or `u` (auto-generated PostScript names).
    ///
    /// Fonts with legitimate glyph names starting with `u` (e.g. SMuFL's
    /// `unison`, `upBow`) should provide a custom filter.
    pub name_filter: Option<fn(&str) -> bool>,
    /// Human-readable font family name for diagnostics and generated code comments,
    /// e.g. `"DSEG7"`, `"Doto"`, `"Bravura"`.
    pub family_display_name: String,
}

/// Default glyph name filter: skips auto-generated PostScript names.
///
/// Returns `false` for names starting with `.`, `uni`, or `u`.
pub fn default_name_filter(name: &str) -> bool {
    !(name.starts_with('.') || name.starts_with("uni") || name.starts_with("u"))
}

/// Generate GResource XML with an explicit prefix (runtime version).
fn generate_gresource_xml_with_prefix(entries: &[GlyphEntry<String>], output_path: &Path, prefix: &str, context: &str) -> std::io::Result<()> {
    let mut sorted: Vec<&GlyphEntry<String>> = entries.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));

    let files: Vec<GResourceFile> = sorted
        .iter()
        .map(|entry| GResourceFile {
            path: format!("scalable/{}/{}.svg", context, entry.name),
        })
        .collect();

    let manifest = GResources {
        gresource: GResource {
            prefix: prefix.to_string(),
            files,
        },
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

impl ExportConfig {
    /// Export all glyphs from a TTF/OTF font file.
    ///
    /// Generates:
    /// - `<output_dir>/scalable/{context}/*.svg` — one SVG file per glyph
    /// - `<output_dir>/metadata.json` — glyph metadata
    /// - `<output_dir>/icons.gresource.xml` — GResource bundle manifest
    ///
    /// # Arguments
    ///
    /// - `font_path` — Path to the TTF/OTF font file
    /// - `output_dir` — Root output directory (typically `resources/`)
    ///
    /// # Returns
    ///
    /// The number of exported glyphs on success, or an `io::Error` on failure.
    pub fn export_glyphs(&self, font_path: &Path, output_dir: &Path) -> std::io::Result<usize> {
        let font_data = fs::read(font_path)?;
        let font = Font::from_data(&font_data).map_err(|e| std::io::Error::other(format!("Failed to parse font: {e}")))?;

        let location = font.location(&self.axes);
        let location_ref = LocationRef::from(&location);

        let icons_dir = output_dir.join("scalable").join(&self.icons_context);
        fs::create_dir_all(&icons_dir)?;

        let reverse_cmap = font.build_reverse_cmap_with_ranges(self.codepoint_ranges);

        let mut entries: Vec<GlyphEntry<String>> = Vec::new();
        let mut seen_names: HashSet<String> = HashSet::new();

        let num_glyphs = font.glyph_names().num_glyphs();

        for glyph_index in 0..num_glyphs {
            let glyph_id = GlyphId::new(glyph_index);

            let name = match font.glyph_names().get(glyph_id) {
                Some(n) if !n.as_str().is_empty() => n.to_string(),
                _ => continue,
            };

            let should_export = self.name_filter.unwrap_or(default_name_filter);
            if !should_export(&name) {
                continue;
            }

            let Some(kebab) = normalize_to_kebab(&name) else { continue };
            let glyph_name = format!("{}-{}", self.glyph_name_prefix, kebab);

            if seen_names.contains(&glyph_name) {
                continue;
            }
            seen_names.insert(glyph_name.clone());

            let codepoint = reverse_cmap.get(&glyph_id).copied();

            let svg = match font.glyph_to_svg_full_height_at(glyph_id, location_ref) {
                Some(svg) => svg,
                None => EMPTY_SVG.to_string(),
            };

            let filename = icons_dir.join(format!("{}.svg", glyph_name));
            let mut file = fs::File::create(&filename)?;
            file.write_all(svg.as_bytes())?;

            let resource_prefix = format!("{}/scalable/{}", self.gresource_prefix, self.icons_context);
            let resource_path = ResourcePath::from_name(&resource_prefix, &glyph_name);

            entries.push(GlyphEntry {
                code: codepoint,
                name: glyph_name.clone(),
                file: Path::new(&format!("resources/scalable/{}/{}.svg", self.icons_context, glyph_name)).to_path_buf(),
                resource_path,
            });
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));

        let json_path = output_dir.join("metadata.json");
        let json = serde_json::to_string_pretty(&entries).map_err(std::io::Error::other)?;
        fs::write(&json_path, json)?;

        let xml_path = output_dir.join("icons.gresource.xml");
        generate_gresource_xml_with_prefix(&entries, &xml_path, &self.gresource_prefix, &self.icons_context)?;

        Ok(entries.len())
    }

    /// Export glyphs from a font using an external name→codepoint map.
    ///
    /// For fonts that lack PostScript glyph names (e.g. SMuFL fonts with
    /// `post` table version 3.0), this method uses an externally provided
    /// mapping of glyph names to Unicode codepoints (e.g. from SMuFL's
    /// `glyphnames.json`) to drive the export.
    ///
    /// For each entry in `name_map`, the method:
    /// 1. Looks up the glyph ID via the font's `cmap` table
    /// 2. Renders the glyph to SVG
    /// 3. Names the file using the kebab-cased glyph name prefixed with `glyph_name_prefix`
    ///
    /// Generates the same outputs as [`export_glyphs`](Self::export_glyphs):
    /// - `<output_dir>/scalable/{context}/*.svg`
    /// - `<output_dir>/metadata.json`
    /// - `<output_dir>/icons.gresource.xml`
    ///
    /// # Arguments
    ///
    /// - `font_path` — Path to the TTF/OTF font file
    /// - `output_dir` — Root output directory (typically `resources/`)
    /// - `name_map` — Mapping of glyph names to Unicode codepoints
    ///
    /// # Returns
    ///
    /// The number of exported glyphs on success, or an `io::Error` on failure.
    pub fn export_glyphs_by_name_map(&self, font_path: &Path, output_dir: &Path, name_map: &GlyphNameMap) -> std::io::Result<usize> {
        let font_data = fs::read(font_path)?;
        let font = Font::from_data(&font_data).map_err(|e| std::io::Error::other(format!("Failed to parse font: {e}")))?;

        let location = font.location(&self.axes);
        let location_ref = LocationRef::from(&location);

        let icons_dir = output_dir.join("scalable").join(&self.icons_context);
        fs::create_dir_all(&icons_dir)?;

        let charmap = font.charmap();

        let mut entries: Vec<GlyphEntry<String>> = Vec::new();
        let mut seen_names: HashSet<String> = HashSet::new();

        for (smufl_name, codepoint) in name_map.iter() {
            let ch = codepoint.as_char();
            let Some(glyph_id) = charmap.map(ch) else { continue };

            let Some(kebab) = normalize_to_kebab(smufl_name) else { continue };
            let glyph_name = format!("{}-{}", self.glyph_name_prefix, kebab);

            if seen_names.contains(&glyph_name) {
                continue;
            }
            seen_names.insert(glyph_name.clone());

            let svg = match font.glyph_to_svg_full_height_at(glyph_id, location_ref) {
                Some(svg) => svg,
                None => EMPTY_SVG.to_string(),
            };

            let filename = icons_dir.join(format!("{}.svg", glyph_name));
            let mut file = fs::File::create(&filename)?;
            file.write_all(svg.as_bytes())?;

            let resource_prefix = format!("{}/scalable/{}", self.gresource_prefix, self.icons_context);
            let resource_path = ResourcePath::from_name(&resource_prefix, &glyph_name);

            entries.push(GlyphEntry {
                code: Some(fonts_rs_model::CodePoint::from(ch)),
                name: glyph_name.clone(),
                file: Path::new(&format!("resources/scalable/{}/{}.svg", self.icons_context, glyph_name)).to_path_buf(),
                resource_path,
            });
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));

        let json_path = output_dir.join("metadata.json");
        let json = serde_json::to_string_pretty(&entries).map_err(std::io::Error::other)?;
        fs::write(&json_path, json)?;

        let xml_path = output_dir.join("icons.gresource.xml");
        generate_gresource_xml_with_prefix(&entries, &xml_path, &self.gresource_prefix, &self.icons_context)?;

        Ok(entries.len())
    }

    /// Write `variant.rs` with `GRESOURCE_PREFIX` and `GLYPH_PREFIX` constants.
    ///
    /// Generates a Rust source file in `OUT_DIR/variant.rs` containing the
    /// GResource prefix and glyph name prefix as `pub const` strings, for use
    /// by the crate's `lib.rs` at compile time.
    pub fn write_variant_info(&self) -> miette::Result<()> {
        let out_dir = std::env::var("OUT_DIR")
            .map_err(|e| miette::miette!("OUT_DIR not set: {e}"))?;
        let variant_info = format!(
            "// @generated by build.rs — do not edit\n\n\
             /// GResource prefix for {}.\n\
             pub const GRESOURCE_PREFIX: &str = \"{}\";\n\n\
             /// Glyph name prefix for {}.\n\
             pub const GLYPH_PREFIX: &str = \"{}\";\n",
            self.family_display_name,
            self.gresource_prefix,
            self.family_display_name,
            self.glyph_name_prefix,
        );
        std::fs::write(std::path::Path::new(&out_dir).join("variant.rs"), variant_info)
            .map_err(|e| miette::miette!("Failed to write variant.rs: {e}"))?;
        Ok(())
    }
}
