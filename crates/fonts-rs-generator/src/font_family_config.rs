//! Build-time extension trait for font families.
//!
//! Provides [`FontFamilyConfigExt`] as an extension trait for any type
//! implementing [`FontFamilyConfig`](fonts_rs_model::FontFamilyConfig).
//! The base trait lives in `fonts-rs-model`; this module adds build-time
//! filesystem operations (directory creation, cleanup).

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::SCALABLE_DIR;

/// Build-time extension trait for [`FontFamilyConfig`].
///
/// Adds filesystem operations for preparing icon output directories during
/// the build pipeline. Implemented blanketly for all `T: FontFamilyConfig`.
pub trait FontFamilyConfigExt: FontFamilyConfig {
    /// Build the icons output directory: `{output_dir}/{SCALABLE_DIR}/{ICONS_CONTEXT}`.
    ///
    /// Follows the Freedesktop Icon Theme Specification used by GTK 4's
    /// `GtkIconTheme`.
    fn icons_dir(output_dir: &Path) -> PathBuf {
        output_dir.join(SCALABLE_DIR).join(Self::ICONS_CONTEXT)
    }

    /// Prepare the icons output directory for a fresh glyph export.
    ///
    /// Removes any existing icons directory (including stale SVGs from
    /// previous builds) and recreates it. Returns the cleaned directory path.
    fn prepare_icons_dir(output_dir: &Path) -> std::io::Result<PathBuf> {
        let icons_dir = Self::icons_dir(output_dir);
        if icons_dir.exists() {
            fs::remove_dir_all(&icons_dir)?;
        }
        fs::create_dir_all(&icons_dir)?;
        Ok(icons_dir)
    }
}

impl<T: FontFamilyConfig> FontFamilyConfigExt for T {}
