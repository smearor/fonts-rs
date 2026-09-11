//! Export Nerd Font glyphs as GTK4 symbolic SVG icons.
//!
//! This module provides a backward-compatible wrapper around the generic
//! `export_glyphs::<NerdFontsDefinition>()` pipeline from `fonts-rs-generator`.

use std::path::Path;

use crate::definition::NerdFontsDefinition;
use fonts_rs_generator::export_glyphs;

/// Export all Nerd Font glyphs from a TTF/OTF font file as GTK4 symbolic SVG icons.
///
/// Generates:
/// - `<output_dir>/scalable/glyphs/*.svg` — one SVG file per glyph
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
    export_glyphs::<NerdFontsDefinition>(font_path, output_dir)
}
