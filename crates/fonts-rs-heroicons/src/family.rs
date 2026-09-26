//! Runtime font family API for Heroicons.
//!
//! Implements the family facade via `impl_font_family!` with glyph
//! maps, the typed icon name, and the capabilities Heroicons supports:
//! `icon_theme` (GResource SVGs via `GtkIconTheme`) and `html_svg`
//! (typed inline-SVG HTML export). Heroicons ships no TTF, so `pango`,
//! `html` (FontFace) and `render` are intentionally absent.

use crate::codepoint_map;
use crate::definition::HeroiconsConfig;
use crate::naming::Heroicons;
use crate::naming::HeroiconsIconName;

fonts_rs_generator::impl_font_family!(
    Heroicons,
    HeroiconsConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = HeroiconsIconName,
    glyph_prefix = "heroicons",
    capabilities = [icon_theme, html_svg],
);
