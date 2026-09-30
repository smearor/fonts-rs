//! Runtime font family API for Fluent UI System Icons.
//!
//! Implements the family facade via `impl_font_family!` with the
//! generated glyph maps, the typed icon name, and the capabilities
//! Fluent supports: `icon_theme` (GResource SVGs), `pango` and `html`
//! (TTF via font.gresource / @font-face - the `html` capability already
//! includes `HtmlSvgExport` for inline-SVG export).

use crate::codepoint_map;
use crate::definition::FluentConfig;
use crate::definition::TTF_FAMILY_NAME;
use crate::naming::Fluent;
use crate::naming::FluentIconName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    Fluent,
    FluentConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = FluentIconName,
    css_font_family = TTF_FAMILY_NAME,
    glyph_prefix = variant::GLYPH_PREFIX,
    gresource_prefix = variant::GRESOURCE_PREFIX,
    capabilities = [icon_theme, pango, html],
);
