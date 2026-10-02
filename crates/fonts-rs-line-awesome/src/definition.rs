//! Font family configuration for Line Awesome.
//!
//! Defines the constants that make Line Awesome a distinct font
//! family: family name, display name, GResource prefix, codepoint
//! range, and icon name prefix.

use const_format::concatcp;
use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

/// Line Awesome codepoint range.
///
/// All upstream codepoints are Font Awesome 5 allocations inside the
/// BMP Private Use Area (`U+F000`–`U+F8FF`). Style variants (regular /
/// solid / brands) share the codepoint of their icon name - unlike
/// Fluent there is no Supplementary PUA-A allocation.
const LINE_AWESOME_RANGES: &[CodePointRange] = &[CodePointRange::new(CodePoint::from_char('\u{F000}'), CodePoint::from_char('\u{F8FF}'))];

/// Configuration for the Line Awesome font family.
///
/// Hybrid set: the three upstream `la-{style}-{weight}.ttf` webfonts
/// for `pango`, `html` (@font-face) and `render` (pixel-drawing), plus
/// pre-rendered SVGs for the GResource icon theme. Codepoints come from
/// the vendored stem-keyed `codepoints.json` generated from upstream
/// `_variables.scss` at vendoring time.
pub struct LineAwesomeConfig;

/// Family name inside the primary bundled TTF's name table
/// (`la-solid-900.ttf`).
///
/// Each of the three upstream webfonts declares its own family name
/// (`la-regular-400`, `la-solid-900`, `la-brands-400`) - the other two
/// are registered alongside the primary face by `install_pango`.
///
/// `dead_code`: unused in the `build.rs` context (`#[path]` include).
#[allow(dead_code)]
pub const TTF_FAMILY_NAME: &str = "la-solid-900";

/// Prefix for exported glyph names (`la-{name}-symbolic`,
/// `la-{name}-solid-symbolic` for the solid style).
///
/// `dead_code`: unused in the `build.rs` context (`#[path]` include).
#[allow(dead_code)]
pub const GLYPH_PREFIX: &str = "la";

impl FontFamilyConfig for LineAwesomeConfig {
    /// Cargo feature / font family identifier.
    const FONT_FAMILY_NAME: &'static str = "la";

    /// Human-readable display name for UI and documentation.
    const FAMILY_DISPLAY_NAME: &'static str = "Line Awesome";

    /// GResource path prefix for this family's resources.
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/line_awesome");

    /// Codepoint range covered by this font family (BMP PUA).
    const CODEPOINT_RANGES: &[CodePointRange] = LINE_AWESOME_RANGES;

    /// Prefix for icon names and CSS classes (`la-{name}`).
    const ICON_PREFIX: &'static str = "la-";
}
