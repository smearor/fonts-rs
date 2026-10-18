//! The [`UpstreamIcon`] struct - one glyph's upstream source location.

use crate::icon_set::IconSetPostProcessor;

/// Where a displayed icon name lives upstream: its SVG source
/// directory, file stem and the post-processors that style needs for
/// GTK symbolic recoloring.
///
/// Returned by [`crate::UpstreamIcons::upstream_icon`]; consumed by
/// `icons.toml` emitters to group glyphs into `[[iconset.sources]]`
/// (`crate` + `subdirectory`) and fill each source's `icons` whitelist
/// and `post_processors`.
#[derive(Debug, Clone, PartialEq)]
pub struct UpstreamIcon {
    /// `resources/svg` subdirectory inside the source crate
    /// (`fill`, `outline`, `regular-24`, `solid`, …); `None` for crates
    /// with a flat `resources/svg` root.
    pub subdirectory: Option<&'static str>,
    /// Upstream SVG file stem - the whitelist name, e.g. `check` for
    /// Lucide or `ic_fluent_check_24_regular` for Fluent.
    pub stem: String,
    /// Suggested export rename when the upstream stem is not a clean
    /// kebab-case icon name (`ic_fluent_check_24_regular` →
    /// `check`, exported via `{ name, rename }`). `None` when the stem
    /// is already spec-kebab.
    pub rename: Option<String>,
    /// Post-processors this style's SVGs typically need - stroke-based
    /// sets take `symbolic-classes`, fill-based `current-color-fill`.
    pub post_processors: &'static [IconSetPostProcessor],
}
