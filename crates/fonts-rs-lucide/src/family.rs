//! Runtime font family API for Lucide Icons.
//!
//! Implements the family facade via `impl_font_family!` with glyph
//! maps, the typed icon name, and the capabilities Lucide supports:
//! `icon_theme` (GResource SVGs via `GtkIconTheme`) and `html_svg`
//! (typed inline-SVG HTML export). Lucide ships no TTF, so `pango`,
//! `html` (FontFace) and `render` are intentionally absent.

use crate::codepoint_map;
use crate::definition::LucideConfig;
use crate::naming::Lucide;
use crate::naming::LucideIconName;

fonts_rs_generator::impl_font_family!(
    Lucide,
    LucideConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = LucideIconName,
    glyph_prefix = "lucide",
    capabilities = [icon_theme, html_svg],
);

impl fonts_rs_model::UpstreamIcons for Lucide {
    const UPSTREAM_POST_PROCESSORS: &'static [fonts_rs_model::IconSetPostProcessor] = &[fonts_rs_model::IconSetPostProcessor::SymbolicClasses];
}
