//! `FontDefinition` trait and `normalize_to_kebab` helper.
//!
//! The central abstraction for the generic export pipeline. Each font family
//! crate implements [`FontDefinition`] to specify its GResource prefix,
//! naming convention, and glyph name normalization logic.

use serde::{Deserialize, Serialize};
use fonts_rs_model::FontFamily;

/// Defines a font family's build-time configuration for the generic
/// export pipeline.
///
/// Each font family crate implements this trait to specify its GResource
/// prefix, naming convention, and glyph name normalization logic. The
/// generic [`export_glyphs`](crate::export_glyphs) function uses this
/// trait to produce SVG icons, metadata, and GResource XML for any
/// TTF/OTF font.
pub trait FontDefinition {
    /// GResource prefix for this font family.
    ///
    /// e.g. `/io/smearor/fonts/seven_segment` or `/io/smearor/fonts/nerd_fonts`.
    const GRESOURCE_PREFIX: &'static str;

    /// Icon context subdirectory within the GResource prefix.
    ///
    /// Follows the GTK 4 `GtkIconTheme` directory convention:
    /// `{prefix}/scalable/{context}/{name}.svg`.
    ///
    /// The `scalable` directory signals that icons are vector (SVG) and
    /// can be rendered at any size. The `context` subdirectory groups
    /// icons by semantic category (e.g. `glyphs`, `status`, `actions`).
    ///
    /// For font glyph icons, `"glyphs"` is the default context.
    /// Font families can override this to use a different context if
    /// needed (e.g. `"emoji"` for Noto Emoji).
    const ICONS_CONTEXT: &'static str = "glyphs";

    /// The glyph name type used by this font family.
    ///
    /// For simple font families, this is `GlyphName<Self::Family>` — the
    /// shared phantom-typed newtype from `fonts-rs-model`. For Nerd Fonts,
    /// this is `IconName`, which carries additional semantics.
    ///
    /// Must implement `AsRef<str>` for use in GResource paths and phf maps,
    /// `Clone` + `Ord` for sorting, and `serde::Serialize` +
    /// `serde::Deserialize` for metadata persistence.
    type Name: AsRef<str> + Clone + Ord + Serialize + for<'a> Deserialize<'a>;

    /// The `FontFamily` marker type for this font family.
    ///
    /// Used as the phantom type parameter for `GlyphName<Self::Family>`.
    /// For Nerd Fonts, this is `NerdFonts`. For simple families, this is
    /// the family's marker enum (e.g. `SevenSegment`, `Barcode`).
    ///
    /// This associated type is only used when `Self::Name` is `GlyphName<F>`.
    /// It is not used when `Self::Name` is a custom type like `IconName`.
    type Family: FontFamily;

    /// Normalize a raw TTF glyph name to this font family's naming
    /// convention.
    ///
    /// Returns `None` if the glyph name is invalid or does not match
    /// the family's naming rules (e.g. unknown prefix for Nerd Fonts).
    ///
    /// # Examples
    ///
    /// - Nerd Fonts: `"fa-gamepad"` -> `"nf-fa-gamepad-symbolic"`
    /// - Seven Segment: `"zero"` -> `"dseg7-0"`
    /// - Barcode: `"code128_start_a"` -> `"barcode-code128-start-a"`
    fn normalize_name(raw_glyph_name: &str) -> Option<Self::Name>;
}

/// Normalize a raw glyph name to kebab-case.
///
/// Converts to lowercase, replaces underscores and non-alphanumeric
/// characters with hyphens, collapses consecutive hyphens, and trims
/// leading/trailing hyphens.
///
/// Returns `None` if the result is empty.
///
/// # Examples
///
/// ```
/// use fonts_rs_generator::normalize_to_kebab;
///
/// assert_eq!(normalize_to_kebab("code128_start_a"), Some("code128-start-a".to_string()));
/// assert_eq!(normalize_to_kebab("zero"), Some("zero".to_string()));
/// assert_eq!(normalize_to_kebab("---"), None);
/// ```
pub fn normalize_to_kebab(raw: &str) -> Option<String> {
    let name = raw.to_lowercase().replace('_', "-");
    let name: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).collect();
    let name: String = name.split('-').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("-");
    if name.is_empty() { None } else { Some(name) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_to_kebab_basic() {
        assert_eq!(normalize_to_kebab("code128_start_a"), Some("code128-start-a".to_string()));
    }

    #[test]
    fn normalize_to_kebab_single_word() {
        assert_eq!(normalize_to_kebab("zero"), Some("zero".to_string()));
    }

    #[test]
    fn normalize_to_kebab_empty_after_filtering() {
        assert_eq!(normalize_to_kebab("---"), None);
    }

    #[test]
    fn normalize_to_kebab_uppercase_to_lowercase() {
        assert_eq!(normalize_to_kebab("CamelCase"), Some("camelcase".to_string()));
    }

    #[test]
    fn normalize_to_kebab_special_chars() {
        assert_eq!(normalize_to_kebab("foo.bar.baz"), Some("foo-bar-baz".to_string()));
    }

    #[test]
    fn normalize_to_kebab_collapses_consecutive_hyphens() {
        assert_eq!(normalize_to_kebab("foo--bar"), Some("foo-bar".to_string()));
    }

    #[test]
    fn normalize_to_kebab_trims_hyphens() {
        assert_eq!(normalize_to_kebab("-foo-"), Some("foo".to_string()));
    }
}
