//! TTF/OTF-backed [`GlyphSource`] — retrofits the `skrifa` outline extraction.

use crate::export_config::ExportConfig;
use crate::export_config::default_name_filter;
use crate::font::Font;
use crate::font_definition::normalize_to_kebab;
use crate::glyph_source::descriptor::GlyphDescriptor;
use crate::glyph_source::error::GlyphSourceError;
use crate::glyph_source::source::GlyphSource;
use crate::svg::EMPTY_SVG;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GlyphNameMap;
use skrifa::GlyphId;
use skrifa::MetadataProvider;
use skrifa::instance::LocationRef;
use std::collections::HashSet;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;

/// `GlyphSource` backed by a TTF/OTF font file.
///
/// Retrofits the existing `skrifa`-based outline extraction
/// ([`ExportConfig::export_glyphs`] / [`ExportConfig::export_glyphs_by_name_map`])
/// to the [`GlyphSource`] interface without changing the emitted artifacts.
///
/// Owns the raw font bytes because `skrifa::FontRef` borrows them —
/// [`Font`] is re-created per call (table parsing is cheap).
pub struct FontFileSource<'a, X: FontFamilyConfig> {
    /// Raw font file bytes.
    font_data: Vec<u8>,
    /// Export configuration carrying family constants and variant state.
    config: &'a ExportConfig<X>,
    /// Optional name map for fonts with non-semantic glyph names
    /// (e.g. `uniXXXX` PostScript names, SMuFL `post` 3.0 fonts).
    name_map: Option<&'a GlyphNameMap>,
    /// Marker for the family config type.
    _marker: PhantomData<X>,
}

impl<'a, X: FontFamilyConfig> FontFileSource<'a, X> {
    /// Reads `font_path` and validates it parses as a font.
    ///
    /// # Errors
    ///
    /// Returns [`GlyphSourceError::Io`] if the file cannot be read, or
    /// [`GlyphSourceError::FontParse`] if the data is not a valid font.
    pub fn new(font_path: &Path, config: &'a ExportConfig<X>) -> Result<Self, GlyphSourceError> {
        let font_data = fs::read(font_path)?;
        Font::from_data(&font_data)?;
        Ok(Self {
            font_data,
            config,
            name_map: None,
            _marker: PhantomData,
        })
    }

    /// Sets the external name→codepoint map (SMuFL, CLDR-style fonts).
    pub fn name_map(mut self, name_map: &'a GlyphNameMap) -> Self {
        self.name_map = Some(name_map);
        self
    }

    /// Lists glyphs via PostScript names — mirrors `export_glyphs`.
    fn glyphs_by_post_names(&self, font: &Font) -> Result<Vec<GlyphDescriptor>, GlyphSourceError> {
        let reverse_cmap = font.build_reverse_cmap_with_ranges(X::CODEPOINT_RANGES);
        let should_export = self.config.name_filter.unwrap_or(default_name_filter);
        let mut seen_names = HashSet::new();
        let mut descriptors = Vec::new();

        for glyph_index in 0..font.glyph_names().num_glyphs() {
            let glyph_id = GlyphId::new(glyph_index);
            let Some(name) = font.glyph_names().get(glyph_id).filter(|name| !name.as_str().is_empty()) else {
                continue;
            };
            if !should_export(name.as_str()) {
                continue;
            }
            let Some(kebab) = normalize_to_kebab(name.as_str()) else { continue };
            let glyph_name = format!("{}-{kebab}", self.config.glyph_name_prefix);
            if !seen_names.insert(glyph_name.clone()) {
                continue;
            }

            descriptors.push(GlyphDescriptor {
                code: reverse_cmap.get(&glyph_id).copied(),
                name: glyph_name,
                source_key: glyph_index.to_string(),
                sequence: None,
            });
        }
        descriptors.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(descriptors)
    }

    /// Lists glyphs via an external name→codepoint map — mirrors
    /// `export_glyphs_by_name_map`.
    fn glyphs_by_name_map(&self, font: &Font, name_map: &GlyphNameMap) -> Result<Vec<GlyphDescriptor>, GlyphSourceError> {
        let charmap = font.charmap();
        let mut seen_names = HashSet::new();
        let mut descriptors = Vec::new();

        for (upstream_name, codepoint) in name_map.iter() {
            let Some(glyph_id) = charmap.map(codepoint.as_char()) else { continue };
            let Some(kebab) = normalize_to_kebab(upstream_name) else { continue };
            let glyph_name = format!("{}-{kebab}", self.config.glyph_name_prefix);
            if !seen_names.insert(glyph_name.clone()) {
                continue;
            }

            descriptors.push(GlyphDescriptor {
                code: Some(*codepoint),
                name: glyph_name,
                source_key: glyph_id.to_u32().to_string(),
                sequence: None,
            });
        }
        descriptors.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(descriptors)
    }
}

impl<X: FontFamilyConfig> GlyphSource for FontFileSource<'_, X> {
    fn glyphs(&self) -> Result<Vec<GlyphDescriptor>, GlyphSourceError> {
        let font = Font::from_data(&self.font_data)?;
        match self.name_map {
            Some(name_map) => self.glyphs_by_name_map(&font, name_map),
            None => self.glyphs_by_post_names(&font),
        }
    }

    fn svg_for(&self, glyph: &GlyphDescriptor) -> Result<String, GlyphSourceError> {
        let font = Font::from_data(&self.font_data)?;
        let location = font.location(&self.config.axes);
        let glyph_index = glyph
            .source_key
            .parse::<u32>()
            .map_err(|_| GlyphSourceError::UnresolvableFilename(glyph.source_key.clone()))?;
        let svg = font
            .glyph_to_svg_full_height_at(GlyphId::new(glyph_index), LocationRef::from(&location))
            .map(|document| document.into_inner())
            .unwrap_or_else(|| EMPTY_SVG.to_string());
        Ok(svg)
    }
}
