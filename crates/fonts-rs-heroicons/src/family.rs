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

impl fonts_rs_model::UpstreamIcons for Heroicons {
    /// Heroicons styles are an infix on the display name:
    /// `heroicons-{stem}-symbolic` is the (unmarked) `outline` style,
    /// `-micro`/`-mini`/`-solid` select the other directories; the
    /// upstream file stem drops the style suffix.
    fn upstream_icon(icon_name: &str) -> Option<fonts_rs_model::UpstreamIcon> {
        let stem = Self::upstream_stem(icon_name)?;
        for style in ["micro", "mini", "solid"] {
            if let Some(name) = stem.strip_suffix(&format!("-{style}")) {
                return Some(fonts_rs_model::UpstreamIcon {
                    subdirectory: Some(style),
                    stem: name.to_string(),
                    rename: None,
                    post_processors: &[fonts_rs_model::IconSetPostProcessor::CurrentColorFill],
                });
            }
        }
        Some(fonts_rs_model::UpstreamIcon {
            subdirectory: Some("outline"),
            stem: stem.to_string(),
            rename: None,
            post_processors: &[fonts_rs_model::IconSetPostProcessor::SymbolicClasses],
        })
    }
}
