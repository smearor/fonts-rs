//! The [`GlyphSource`] trait - core abstraction for glyph sources.

use crate::export_config::ExportConfig;
use crate::glyph_source::descriptor::GlyphDescriptor;
use crate::glyph_source::error::GlyphSourceError;
use crate::glyph_source::export_target::GlyphExportTarget;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GlyphEntry;
use std::fs;
use std::path::Path;

/// A source of glyphs for the build pipeline.
///
/// Implementations provide glyph identities ([`GlyphDescriptor`] list)
/// and the SVG artwork for each glyph. The shared [`export`](Self::export)
/// runner turns both into `GlyphEntry` artifacts - `metadata.json`,
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
    /// Default implementation - works for every `GlyphSource`, whether
    /// `FontFileSource` or `SvgDirSource`. Adapter over
    /// [`export_to`](Self::export_to) building the [`GlyphExportTarget`]
    /// from `X`'s compile-time constants and `config`.
    ///
    /// Returns the number of exported glyphs.
    fn export<X: FontFamilyConfig>(&self, config: &ExportConfig<X>, resources_dir: &Path) -> Result<usize, GlyphSourceError> {
        let target = GlyphExportTarget {
            gresource_prefix: config.gresource_prefix.clone(),
            icons_context: X::ICONS_CONTEXT.to_string(),
        };
        self.export_to(&target, resources_dir)
    }

    /// Runs the shared export pipeline against a runtime
    /// [`GlyphExportTarget`] - the `icons_context`/`gresource_prefix`
    /// values [`export`](Self::export) previously sourced from its
    /// `FontFamilyConfig` type parameter.
    ///
    /// Entry point for builds without a compile-time config type, e.g.
    /// TOML-driven custom icon sets.
    ///
    /// Returns the number of exported glyphs.
    fn export_to(&self, target: &GlyphExportTarget, resources_dir: &Path) -> Result<usize, GlyphSourceError> {
        let icons_dir = target.prepare_icons_dir(resources_dir)?;

        let descriptors = self.glyphs()?;
        let mut entries: Vec<GlyphEntry<String>> = Vec::with_capacity(descriptors.len());
        for descriptor in descriptors {
            let svg = self.svg_for(&descriptor)?;
            let mut entry = GlyphEntry::new(descriptor.code, descriptor.name, &target.gresource_prefix, &target.icons_context);
            entry.sequence = descriptor.sequence;
            fs::write(icons_dir.join(format!("{}.svg", entry.name)), svg)?;
            entries.push(entry);
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));

        let json_path = resources_dir.join("metadata.json");
        let json = serde_json::to_string_pretty(&entries)?;
        fs::write(&json_path, json)?;

        let xml_path = resources_dir.join("icons.gresource.xml");
        target.generate_gresource_xml(&entries, &xml_path)?;

        Ok(entries.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::glyph_source::svg_dir_source::SvgDirSource;

    /// Minimal `FontFamilyConfig` fixture for the regression test.
    struct TestConfig;

    impl FontFamilyConfig for TestConfig {
        const FONT_FAMILY_NAME: &'static str = "test-icons";
        const FAMILY_DISPLAY_NAME: &'static str = "Test Icons";
        const GRESOURCE_PREFIX: &'static str = "/io/test/icons";
    }

    /// Collects all files under `dir` as `(relative path, contents)` pairs.
    fn collect_files(dir: &Path, base: &Path, files: &mut Vec<(String, Vec<u8>)>) {
        for entry in fs::read_dir(dir).expect("read dir") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                collect_files(&path, base, files);
            } else {
                let rel = path.strip_prefix(base).expect("prefix").to_string_lossy().to_string();
                files.push((rel, fs::read(&path).expect("read file")));
            }
        }
        files.sort_by(|a, b| a.0.cmp(&b.0));
    }

    /// `export` delegates to `export_to` via a `GlyphExportTarget` built
    /// from `X`'s constants - both paths must produce byte-identical
    /// artifacts (regression gate for the export refactor).
    #[test]
    fn export_and_export_to_produce_identical_output() {
        let base = std::env::temp_dir().join(format!("glyph_source_export_test_{}", std::process::id()));
        let svg_dir = base.join("svgs");
        fs::create_dir_all(&svg_dir).expect("create svg dir");
        let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0\"/></svg>";
        fs::write(svg_dir.join("alpha.svg"), svg).expect("write alpha");
        fs::write(svg_dir.join("beta.svg"), svg).expect("write beta");

        let source = SvgDirSource::new(&svg_dir);
        let config = ExportConfig::<TestConfig>::new();

        let via_export = base.join("via_export");
        let via_export_to = base.join("via_export_to");
        source.export(&config, &via_export).expect("export");

        let target = GlyphExportTarget {
            gresource_prefix: config.gresource_prefix.clone(),
            icons_context: TestConfig::ICONS_CONTEXT.to_string(),
        };
        source.export_to(&target, &via_export_to).expect("export_to");

        let mut a = Vec::new();
        let mut b = Vec::new();
        collect_files(&via_export, &via_export, &mut a);
        collect_files(&via_export_to, &via_export_to, &mut b);
        assert_eq!(a, b);

        fs::remove_dir_all(&base).expect("remove temp dir");
    }
}
