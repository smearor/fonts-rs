//! Base configuration trait for font families.
//!
//! Provides compile-time constants shared by all font family definition
//! types. The build-time methods (`icons_dir`, `prepare_icons_dir`) live
//! in `fonts-rs-generator` as the [`FontFamilyConfigExt`] extension trait.

use crate::BMP_RANGE;
use crate::CodePointRange;
use crate::ICONS_CONTEXT_GLYPHS;
use crate::SCALABLE_DIR;

/// Base configuration for font families.
///
/// Implemented by all font family definition types to provide
/// compile-time constants for GResource prefixes, icon context
/// directories, and codepoint ranges.
///
/// Build-time methods for filesystem operations are provided by
/// [`FontFamilyConfigExt`](fonts_rs_generator::FontFamilyConfigExt)
/// in `fonts-rs-generator`.
pub trait FontFamilyConfig {
    /// Font family slug used in the GResource prefix and glyph names.
    ///
    /// e.g. `"dseg7"` or `"doto"`. Combined with
    /// [`GRESOURCE_BASE_PREFIX`](crate::GRESOURCE_BASE_PREFIX) to
    /// produce the full `GRESOURCE_PREFIX`.
    const FONT_FAMILY_NAME: &'static str;

    /// Human-readable display name for the font family.
    ///
    /// e.g. `"DSEG7"` or `"Doto"`. Used in build log messages.
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
    /// needed (e.g. [`ICONS_CONTEXT_EMOJI`](crate::ICONS_CONTEXT_EMOJI) for Noto Emoji).
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

    /// Icon name prefix for this family's GTK icon names.
    ///
    /// Icon names follow the convention `{ICON_PREFIX}{name}-symbolic`
    /// (kebab-case, lowercase). Nerd Fonts uses `"nf-"`; standalone icon
    /// fonts use their own prefix (e.g. `"tabler-"` for Tabler Icons).
    ///
    /// Defaults to `""` for families that don't use typed
    /// [`IconName`](crate::IconName) handling.
    const ICON_PREFIX: &'static str = "";

    /// Validates a fully-formed icon name (`{ICON_PREFIX}{name}-symbolic`).
    ///
    /// The default accepts any name carrying the family's
    /// [`ICON_PREFIX`](Self::ICON_PREFIX) — sufficient for single-set
    /// icon fonts. Multi-set icon fonts override this to check set
    /// membership (Nerd Fonts validates via `IconSet::detect`).
    fn is_valid_icon_name(name: &str) -> bool {
        Self::ICON_PREFIX.is_empty() || name.starts_with(Self::ICON_PREFIX)
    }

    /// Build the GResource sub-prefix for icons: `{GRESOURCE_PREFIX}/{SCALABLE_DIR}/{ICONS_CONTEXT}`.
    ///
    /// Used in `icons.gresource.xml` and resource paths.
    fn icons_resource_prefix() -> String {
        format!("{}/{}/{}", Self::GRESOURCE_PREFIX, SCALABLE_DIR, Self::ICONS_CONTEXT)
    }
}
