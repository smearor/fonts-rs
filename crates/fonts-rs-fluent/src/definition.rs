//! Font family configuration for Fluent UI System Icons.
//!
//! Defines the constants that make Fluent a distinct font family:
//! family name, display name, GResource prefix, codepoint ranges, and
//! icon name prefix.

use const_format::concatcp;
use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

/// Fluent UI System Icons codepoint ranges.
///
/// The upstream codepoint map spans two disjoint Private Use Areas:
/// the BMP PUA (`U+E000`–`U+F8FF`) for the majority of icons and the
/// Supplementary PUA-A tail (`U+F0000`–`U+F0D9F`, ~26% of the map).
/// Both ranges must be probed for the reverse cmap.
const FLUENT_RANGES: &[CodePointRange] = &[
    CodePointRange::new(CodePoint::from_char('\u{E000}'), CodePoint::from_char('\u{F8FF}')),
    CodePointRange::new(CodePoint::from_char('\u{F0000}'), CodePoint::from_char('\u{F0DFF}')),
];

/// Configuration for the Fluent UI System Icons font family.
///
/// Hybrid set: bundled `FluentSystemIcons-{Regular,Filled}.ttf` webfonts
/// for `pango`, `html` (@font-face) and `render` (pixel-drawing), plus
/// pre-rendered SVGs for the GResource icon theme. Codepoints come from
/// the vendored upstream `FluentSystemIcons-*.json` maps.
pub struct FluentConfig;

/// Family name inside the bundled TTF's name table - variant-aware
/// because the two upstream webfonts declare different families.
///
/// Used as `CSS_FONT_FAMILY` so Pango/`font-family` resolution and
/// `@font-face` pages reference the name the TTF actually exposes;
/// the display name `"Fluent UI System Icons"` would never match.
///
/// `dead_code`: unused in the `build.rs` context (`#[path]` include).
#[cfg(feature = "filled")]
#[allow(dead_code)]
pub const TTF_FAMILY_NAME: &str = "FluentSystemIcons-Filled";

/// Family name inside the bundled TTF's name table (regular variant).
/// See the `filled` variant above for the semantic.
///
/// `dead_code`: unused in the `build.rs` context (`#[path]` include).
#[cfg(not(feature = "filled"))]
#[allow(dead_code)]
pub const TTF_FAMILY_NAME: &str = "FluentSystemIcons-Regular";

impl FontFamilyConfig for FluentConfig {
    /// Cargo feature / font family identifier.
    const FONT_FAMILY_NAME: &'static str = "fluent";

    /// Human-readable display name for UI and documentation.
    const FAMILY_DISPLAY_NAME: &'static str = "Fluent UI System Icons";

    /// GResource path prefix for this family's resources.
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", FluentConfig::FONT_FAMILY_NAME);

    /// Codepoint ranges covered by this font family (BMP PUA + PUA-A tail).
    const CODEPOINT_RANGES: &[CodePointRange] = FLUENT_RANGES;

    /// Prefix for icon names and CSS classes (`fluent-{name}`).
    const ICON_PREFIX: &'static str = "fluent-";
}
