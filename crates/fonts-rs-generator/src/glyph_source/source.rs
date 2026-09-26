//! The [`GlyphSource`] trait — core abstraction for glyph sources.

use crate::export_config::ExportConfig;
use crate::font_family_config::FontFamilyConfigExt;
use crate::glyph_source::descriptor::GlyphDescriptor;
use crate::glyph_source::error::GlyphSourceError;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GlyphEntry;
use std::fs;
use std::path::Path;

/// A source of glyphs for the build pipeline.
///
/// Implementations provide glyph identities ([`GlyphDescriptor`] list)
/// and the SVG artwork for each glyph. The shared [`export`](Self::export)
/// runner turns both into `GlyphEntry` artifacts — `metadata.json`,
/// `gresource.xml`, `phf` codegen consume them uniformly.
///
/// Two implementations exist: `FontFileSource` (TTF/OTF outline
/// extraction via `skrifa`) and `SvgDirSource` (pre-rendered SVG
/// directories).
pub trait GlyphSource {
    /// Lists all glyphs the source provides, in deterministic order.
    fn glyphs(&self) -> Result<Vec<GlyphDescriptor>, GlyphSourceError>;

    /// Returns the SVG artwork for the given glyph.
    fn svg_for(&self, glyph: &GlyphDescriptor) -> Result<String, GlyphSourceError>;

    /// Runs the shared export pipeline: writes each glyph's SVG into
    /// `resources_dir`, then emits `metadata.json` and
    /// `icons.gresource.xml`.
    ///
    /// Default implementation — works for every `GlyphSource`, whether
    /// `FontFileSource` or `SvgDirSource`.
    ///
    /// Returns the number of exported glyphs.
    fn export<X: FontFamilyConfig>(&self, config: &ExportConfig<X>, resources_dir: &Path) -> Result<usize, GlyphSourceError> {
        let icons_dir = X::prepare_icons_dir(resources_dir)?;

        let descriptors = self.glyphs()?;
        let mut entries: Vec<GlyphEntry<String>> = Vec::with_capacity(descriptors.len());
        for descriptor in descriptors {
            let svg = self.svg_for(&descriptor)?;
            let mut entry = GlyphEntry::new(descriptor.code, descriptor.name, &config.gresource_prefix, X::ICONS_CONTEXT);
            entry.sequence = descriptor.sequence;
            fs::write(icons_dir.join(format!("{}.svg", entry.name)), svg)?;
            entries.push(entry);
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));

        let json_path = resources_dir.join("metadata.json");
        let json = serde_json::to_string_pretty(&entries)?;
        fs::write(&json_path, json)?;

        let xml_path = resources_dir.join("icons.gresource.xml");
        config.generate_gresource_xml(&entries, &xml_path)?;

        Ok(entries.len())
    }
}
