//! The [`GlyphExportTarget`] struct - runtime export destination.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use fonts_rs_model::GlyphEntry;
use fonts_rs_model::SCALABLE_DIR;
use quick_xml::se::Serializer;
use serde::Serialize;

use crate::export_error::ExportError;
use crate::gresource::model::GResource;
use crate::gresource::model::GResourceFile;
use crate::gresource::model::GResources;

/// Runtime export destination for a glyph source.
///
/// Carries the two values
/// [`GlyphSource::export`](crate::glyph_source::GlyphSource::export) previously
/// read from the `FontFamilyConfig` type parameter - split out so TOML-driven
/// builds (custom icon sets) can export without a compile-time config type.
pub struct GlyphExportTarget {
    /// GResource prefix written into `GlyphEntry` resource paths and
    /// `icons.gresource.xml`, e.g. `/io/smearor/fonts/dseg7/classic_regular`.
    pub gresource_prefix: String,
    /// Freedesktop icon context subdirectory (`scalable/{context}/`),
    /// e.g. `actions` or `emblems`.
    pub icons_context: String,
}

impl GlyphExportTarget {
    /// Build the icons output directory: `{output_dir}/{SCALABLE_DIR}/{icons_context}`.
    ///
    /// Follows the Freedesktop Icon Theme Specification used by GTK 4's
    /// `GtkIconTheme`.
    pub fn icons_dir(&self, output_dir: &Path) -> PathBuf {
        output_dir.join(SCALABLE_DIR).join(&self.icons_context)
    }

    /// Prepare the icons output directory for a fresh glyph export.
    ///
    /// Removes any existing icons directory (including stale SVGs from
    /// previous builds) and recreates it. Returns the cleaned directory path.
    pub fn prepare_icons_dir(&self, output_dir: &Path) -> std::io::Result<PathBuf> {
        let icons_dir = self.icons_dir(output_dir);
        if icons_dir.exists() {
            fs::remove_dir_all(&icons_dir)?;
        }
        fs::create_dir_all(&icons_dir)?;
        Ok(icons_dir)
    }

    /// Write `icons.gresource.xml` describing the exported glyph entries.
    ///
    /// Entries are sorted by name; each file path follows the Freedesktop
    /// layout `{SCALABLE_DIR}/{icons_context}/{name}.svg`.
    pub(crate) fn generate_gresource_xml(&self, entries: &[GlyphEntry<String>], output_path: &Path) -> Result<(), ExportError> {
        let mut sorted: Vec<&GlyphEntry<String>> = entries.iter().collect();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));

        let files: Vec<GResourceFile> = sorted
            .iter()
            .map(|entry| GResourceFile {
                path: format!("{}/{}/{}.svg", SCALABLE_DIR, self.icons_context, entry.name),
            })
            .collect();

        let manifest = GResources {
            gresource: GResource {
                prefix: self.gresource_prefix.clone(),
                files,
            },
        };

        let mut buffer = String::new();
        let mut serializer = Serializer::new(&mut buffer);
        serializer.indent(' ', 2);
        manifest.serialize(serializer)?;

        let xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{buffer}");
        fs::write(output_path, xml)?;
        Ok(())
    }
}
