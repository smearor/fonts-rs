//! The [`IconSource`] registry entry - one icon-capable family mapped
//! to its `[[iconset.sources]]` crate.

use fonts_rs_model::UpstreamIcon;

/// An icon-capable family: maps the cheat-sheet-facing family to the
/// `[[iconset.sources]]` crate it can feed. Stem/subdirectory/post-
/// processor logic lives in each family's `UpstreamIcons` impl - this
/// is pure builder wiring.
pub struct IconSource {
    /// Source crate name for `icons.toml` (`crate = "…"`).
    pub crate_name: &'static str,
    /// Reverse-maps a grid icon name to its upstream location
    /// (subdirectory + file stem + post-processors).
    pub upstream_icon: fn(&str) -> Option<UpstreamIcon>,
}

impl IconSource {
    /// The exported icon name a whitelist entry produces under the
    /// set's `icon_prefix` — `Some(rename)` wins over `upstream`.
    pub fn exported_name(set_prefix: &str, upstream: &str, rename: Option<&str>) -> String {
        format!("{}-{}-symbolic", set_prefix, rename.unwrap_or(upstream))
    }

    /// Finds the registry source index + upstream location for a grid
    /// icon name - first family whose `upstream_icon` accepts it.
    pub fn resolve(icon_name: &str) -> Option<(usize, UpstreamIcon)> {
        ICON_SOURCES
            .iter()
            .enumerate()
            .find_map(|(index, source)| (source.upstream_icon)(icon_name).map(|upstream| (index, upstream)))
    }
}

/// The icon-capable families the builder offers - the six crates that
/// ship `links`/`cargo:svg_dir` metadata for `IconSetBuild`.
///
/// Tabler's cheat sheet compiles the `outline` variant; the `filled`
/// subdirectory is selectable by hand-editing the emitted spec (the
/// sheet's grid does not surface filled icons).
pub static ICON_SOURCES: &[IconSource] = &[
    IconSource {
        crate_name: "fonts-rs-eva",
        upstream_icon: <fonts_rs_eva::Eva as fonts_rs_model::UpstreamIcons>::upstream_icon,
    },
    IconSource {
        crate_name: "fonts-rs-fluent",
        upstream_icon: <fonts_rs_fluent::Fluent as fonts_rs_model::UpstreamIcons>::upstream_icon,
    },
    IconSource {
        crate_name: "fonts-rs-heroicons",
        upstream_icon: <fonts_rs_heroicons::Heroicons as fonts_rs_model::UpstreamIcons>::upstream_icon,
    },
    IconSource {
        crate_name: "fonts-rs-line-awesome",
        upstream_icon: <fonts_rs_line_awesome::LineAwesome as fonts_rs_model::UpstreamIcons>::upstream_icon,
    },
    IconSource {
        crate_name: "fonts-rs-lucide",
        upstream_icon: <fonts_rs_lucide::Lucide as fonts_rs_model::UpstreamIcons>::upstream_icon,
    },
    IconSource {
        crate_name: "fonts-rs-tabler",
        upstream_icon: <fonts_rs_tabler::Tabler as fonts_rs_model::UpstreamIcons>::upstream_icon,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exported_name_prefers_rename() {
        assert_eq!(IconSource::exported_name("app", "trash", Some("delete")), "app-delete-symbolic");
        assert_eq!(IconSource::exported_name("app", "check", None), "app-check-symbolic");
    }

    #[test]
    fn resolve_routes_each_family() {
        let (index, upstream) = IconSource::resolve("lucide-check-symbolic").unwrap();
        assert_eq!(ICON_SOURCES[index].crate_name, "fonts-rs-lucide");
        assert_eq!(upstream.stem, "check");
        assert_eq!(upstream.subdirectory, None);

        let (_, tabler) = IconSource::resolve("tabler-heart-symbolic").unwrap();
        assert_eq!(tabler.subdirectory, Some("outline"));

        let (_, eva) = IconSource::resolve("eva-checkmark-outline-symbolic").unwrap();
        assert_eq!(eva.subdirectory, Some("outline"));
        assert_eq!(eva.stem, "checkmark-outline");

        let (_, fluent) = IconSource::resolve("fluent-panel-left-add-symbolic").unwrap();
        assert_eq!(fluent.subdirectory, Some("regular-24"));
        assert_eq!(fluent.stem, "ic_fluent_panel_left_add_24_regular");

        assert!(IconSource::resolve("unknown-check-symbolic").is_none());
    }
}
