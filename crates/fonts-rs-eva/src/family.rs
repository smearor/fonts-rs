//! Runtime font family API for Eva Icons.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::EvaConfig;
use crate::naming::Eva;
use crate::naming::EvaIconName;

fonts_rs_generator::impl_font_family!(
    Eva,
    EvaConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = EvaIconName,
    css_font_family = "Eva-Icons",
    glyph_prefix = "eva",
    capabilities = [icon_theme, pango, html],
);

impl fonts_rs_model::UpstreamIcons for Eva {
    /// Eva ships two styles: `fill/` (stems like `activity`) and
    /// `outline/` (stems like `activity-outline` - the style suffix is
    /// part of the upstream file name).
    fn upstream_icon(icon_name: &str) -> Option<fonts_rs_model::UpstreamIcon> {
        let stem = Self::upstream_stem(icon_name)?;
        let (subdirectory, post_processors) = if stem.ends_with("-outline") {
            (Some("outline"), &[fonts_rs_model::IconSetPostProcessor::SymbolicClasses][..])
        } else {
            (Some("fill"), &[fonts_rs_model::IconSetPostProcessor::CurrentColorFill][..])
        };
        Some(fonts_rs_model::UpstreamIcon {
            subdirectory,
            stem: stem.to_string(),
            rename: None,
            post_processors,
        })
    }
}
