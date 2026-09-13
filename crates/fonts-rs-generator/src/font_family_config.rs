//! Base configuration trait for font families.
//!
//! Provides compile-time constants shared by both [`FontDefinition`](crate::FontDefinition)
//! (type-safe glyph name pipeline) and [`ExportConfig`](crate::ExportConfig)
//! (runtime variant pipeline).

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use fonts_rs_model::BMP_RANGE;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::ICONS_CONTEXT_GLYPHS;
use fonts_rs_model::SCALABLE_DIR;

/// Base configuration for font families.
///
/// Implemented by all font family definition types to provide
/// compile-time constants for GResource prefixes, icon context
/// directories, and codepoint ranges.
///
/// [`FontDefinition`](crate::FontDefinition) extends this trait with
/// type-safe glyph name handling. [`ExportConfig`](crate::ExportConfig)
/// is generic over any type implementing this trait.
pub trait FontFamilyConfig {
    /// Font family slug used in the GResource prefix and glyph names.
    ///
    /// e.g. `"dseg7"` or `"doto"`. Combined with
    /// [`GRESOURCE_BASE_PREFIX`](fonts_rs_model::GRESOURCE_BASE_PREFIX) to
    /// produce the full `GRESOURCE_PREFIX`, and used directly as the
    /// `glyph_name_prefix` base in [`ExportConfig::new`](crate::ExportConfig::new).
    const FONT_FAMILY_NAME: &'static str;

    /// Human-readable display name for the font family.
    ///
    /// e.g. `"DSEG7"` or `"Doto"`. Used in build log messages and
    /// [`ExportConfig::new`](crate::ExportConfig::new) for the `family_display_name` field.
    const FAMILY_DISPLAY_NAME: &'static str;

    /// GResource prefix for this font family.
    ///
    /// e.g. `/io/smearor/fonts/dseg7` or `/io/smearor/fonts/nerd_fonts`.
    const GRESOURCE_PREFIX: &'static str;

    /// Icon context subdirectory within the GResource prefix.
    ///
    /// Follows the Freedesktop Icon Theme Specification used by GTK 4's
    /// `GtkIconTheme`: `{prefix}/scalable/{context}/{name}.svg`.
    ///
    /// The `scalable` directory signals that icons are vector (SVG) and
    /// can be rendered at any size. The `context` subdirectory groups
    /// icons by semantic category (e.g. `glyphs`, `status`, `actions`).
    ///
    /// For font glyph icons, [`ICONS_CONTEXT_GLYPHS`] is the default context.
    /// Font families can override this to use a different context if
    /// needed (e.g. [`ICONS_CONTEXT_EMOJI`](fonts_rs_model::ICONS_CONTEXT_EMOJI) for Noto Emoji).
    const ICONS_CONTEXT: &'static str = ICONS_CONTEXT_GLYPHS;

    /// Unicode codepoint ranges to probe when building the reverse cmap.
    ///
    /// Each tuple is `(start, end)` inclusive. The generic pipeline probes
    /// these ranges to map `GlyphId` -> `CodePoint` for each glyph in the
    /// font.
    ///
    /// Defaults to the BMP (`U+0000`–`U+FFFF`), which covers most fonts.
    /// Font families with glyphs in supplementary planes (e.g. Nerd Fonts
    /// PUA at `U+F0001`–`U+10FFFF`) should override this.
    const CODEPOINT_RANGES: &[CodePointRange] = &[BMP_RANGE];

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

    /// Build the GResource sub-prefix for icons: `{GRESOURCE_PREFIX}/{SCALABLE_DIR}/{ICONS_CONTEXT}`.
    ///
    /// Used in `icons.gresource.xml` and resource paths.
    fn icons_resource_prefix() -> String {
        format!("{}/{}/{}", Self::GRESOURCE_PREFIX, SCALABLE_DIR, Self::ICONS_CONTEXT)
    }
}
