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

impl fonts_rs_model::UpstreamIcons for Fluent {
    /// Fluent exports `fluent-{name}-symbolic` (regular) and
    /// `fluent-filled-{name}-symbolic` (filled); upstream files are
    /// `ic_fluent_{snake}_24_{style}` under `regular-24`/`filled-24`.
    fn upstream_icon(icon_name: &str) -> Option<fonts_rs_model::UpstreamIcon> {
        let stem = Self::upstream_stem(icon_name)?;
        let (subdirectory, style, name) = match stem.strip_prefix("filled-") {
            Some(rest) => ("filled-24", "filled", rest),
            None => ("regular-24", "regular", stem),
        };
        Some(fonts_rs_model::UpstreamIcon {
            subdirectory: Some(subdirectory),
            stem: format!("ic_fluent_{}_24_{style}", name.replace('-', "_")),
            // The snake-case upstream stem is not kebab-spec - the
            // whitelist entry must carry the clean stem as `rename`.
            rename: Some(name.to_string()),
            post_processors: &[fonts_rs_model::IconSetPostProcessor::CurrentColorFill],
        })
    }
}
