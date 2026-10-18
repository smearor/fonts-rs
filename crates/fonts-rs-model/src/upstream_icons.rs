//! The [`UpstreamIcons`] trait - reverse-mapping displayed icon names
//! to upstream SVG sources.

use crate::FontFamilyConfig;
use crate::icon_set::IconSetPostProcessor;
use crate::upstream_icon::UpstreamIcon;

/// Maps a family's exported icon name (`lucide-check-symbolic`) back to
/// its upstream source - the `resources/svg` subdirectory, file stem
/// and default post-processors, bundled as [`UpstreamIcon`].
///
/// Icon-capable families implement this next to their
/// `impl_font_family!` invocation. The default path covers flat
/// single-style families (`ICON_PREFIX` + `-symbolic` stripping);
/// multi-style families override [`upstream_icon`](Self::upstream_icon)
/// for per-stem routing (Eva `fill`/`outline`, Fluent
/// `regular-24`/`filled-24`, Heroicons `micro`/`mini`/`solid`).
pub trait UpstreamIcons: FontFamilyConfig {
    /// `resources/svg` subdirectory for single-style families -
    /// `Some("outline")` for Tabler, `None` for flat roots.
    const UPSTREAM_SUBDIRECTORY: Option<&'static str> = None;

    /// Post-processors the family's SVG style typically needs.
    const UPSTREAM_POST_PROCESSORS: &'static [IconSetPostProcessor] = &[];

    /// Strips `ICON_PREFIX` and `-symbolic` from an exported icon
    /// name - the shared first step of every reverse mapping.
    /// Returns `None` for names outside the family's convention.
    fn upstream_stem(icon_name: &str) -> Option<&str> {
        icon_name
            .strip_prefix(Self::ICON_PREFIX)?
            .strip_suffix("-symbolic")
            .filter(|stem| !stem.is_empty())
    }

    /// Reverse-maps a displayed icon name to its upstream location.
    /// `None` for non-conforming names - callers must skip rather
    /// than emit a wrong stem (a wrong name surfaces as `UnknownIcon`
    /// in `IconSetBuild`).
    fn upstream_icon(icon_name: &str) -> Option<UpstreamIcon> {
        Self::upstream_stem(icon_name).map(|stem| UpstreamIcon {
            subdirectory: Self::UPSTREAM_SUBDIRECTORY,
            stem: stem.to_string(),
            rename: None,
            post_processors: Self::UPSTREAM_POST_PROCESSORS,
        })
    }
}
