//! The [`IconSelection`] model - which upstream icons are picked, per
//! source, with optional renames.

use crate::builder_settings::BuilderSettings;
use crate::icon_source::ICON_SOURCES;
use crate::icon_source::IconSource;
use fonts_rs_model::IconSetFile;
use fonts_rs_model::IconSetPostProcessor;
use fonts_rs_model::IconSetSourceLocation;
use fonts_rs_model::IconSetSourceSpec;
use fonts_rs_model::IconSetSpec;
use fonts_rs_model::IconSetSpecError;
use fonts_rs_model::IconWhitelistEntry;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

/// One picked icon: the upstream stem plus an optional rename.
#[derive(Debug, Clone)]
pub struct SelectedIcon {
    /// Upstream SVG file stem (`check`, `ic_fluent_check_24_regular`).
    pub upstream: String,
    /// Optional exported rename (`delete` → `{ name = "trash", rename =
    /// "delete" }`); `None` exports under the upstream name.
    pub rename: Option<String>,
}

/// One populated `[[iconset.sources]]` group - same crate + same
/// subdirectory.
struct SourceGroup {
    /// Post-processors for the group's style - set from the first
    /// icon's `UpstreamIcon`.
    post_processors: &'static [IconSetPostProcessor],
    /// Upstream stem → pick.
    icons: BTreeMap<String, SelectedIcon>,
}

/// Selection grouping key: `ICON_SOURCES` family index + the
/// subdirectory reported by `upstream_icon`.
pub type SourceKey = (usize, Option<&'static str>);

/// The builder's selection model: per (family, subdirectory) group,
/// the picked upstream icons in `BTreeMap` order (deterministic
/// emission).
pub struct IconSelection {
    /// (family index, subdirectory) → group.
    entries: BTreeMap<SourceKey, SourceGroup>,
}

impl IconSelection {
    /// An empty selection.
    pub fn new() -> Self {
        Self { entries: BTreeMap::new() }
    }

    /// Adds the icon (by grid icon name) to its source's selection;
    /// `false` when the name matches no family or convention.
    pub fn add(&mut self, icon_name: &str) -> bool {
        let Some((index, upstream)) = IconSource::resolve(icon_name) else {
            return false;
        };
        self.entries
            .entry((index, upstream.subdirectory))
            .or_insert_with(|| SourceGroup {
                post_processors: upstream.post_processors,
                icons: BTreeMap::new(),
            })
            .icons
            .insert(
                upstream.stem.clone(),
                SelectedIcon {
                    upstream: upstream.stem,
                    rename: upstream.rename,
                },
            );
        true
    }

    /// Removes an icon by grid icon name (routes to its source group).
    pub fn remove_name(&mut self, icon_name: &str) {
        if let Some((index, upstream)) = IconSource::resolve(icon_name)
            && let Some(group) = self.entries.get_mut(&(index, upstream.subdirectory))
        {
            group.icons.remove(&upstream.stem);
        }
    }

    /// Removes an upstream icon from a source group's selection.
    pub fn remove(&mut self, key: SourceKey, upstream: &str) {
        if let Some(group) = self.entries.get_mut(&key) {
            group.icons.remove(upstream);
        }
    }

    /// Sets/clears the rename of a selected icon; empty text clears it.
    pub fn set_rename(&mut self, key: SourceKey, upstream: &str, rename: Option<String>) {
        if let Some(icon) = self.entries.get_mut(&key).and_then(|group| group.icons.get_mut(upstream)) {
            icon.rename = rename.filter(|name| !name.is_empty());
        }
    }

    /// Whether the grid icon name is currently picked.
    pub fn contains(&self, icon_name: &str) -> bool {
        let Some((index, upstream)) = IconSource::resolve(icon_name) else {
            return false;
        };
        self.entries
            .get(&(index, upstream.subdirectory))
            .is_some_and(|group| group.icons.contains_key(&upstream.stem))
    }

    /// The populated source groups - (group key, source, icons) in
    /// deterministic order; the key carries (family index, subdirectory).
    pub fn groups(&self) -> impl Iterator<Item = (SourceKey, &'static IconSource, &BTreeMap<String, SelectedIcon>)> {
        self.entries
            .iter()
            .map(|(key @ (index, _subdirectory), group)| (*key, &ICON_SOURCES[*index], &group.icons))
    }

    /// Total picked icons across all groups.
    pub fn len(&self) -> usize {
        self.entries.values().map(|group| group.icons.len()).sum()
    }

    /// Whether nothing is picked.
    pub fn is_empty(&self) -> bool {
        self.entries.values().all(|group| group.icons.is_empty())
    }

    /// The exported name under the set's `icon_prefix`.
    pub fn exported_name(&self, settings: &BuilderSettings, icon: &SelectedIcon) -> String {
        IconSource::exported_name(&settings.icon_prefix, &icon.upstream, icon.rename.as_deref())
    }

    /// Exported names that would collide across sources - the same
    /// `IconNameCollision` rule the generator enforces, surfaced live.
    pub fn collisions(&self, settings: &BuilderSettings) -> BTreeSet<String> {
        let mut seen = BTreeSet::new();
        let mut dupes = BTreeSet::new();
        for group in self.entries.values() {
            for icon in group.icons.values() {
                let name = self.exported_name(settings, icon);
                if !seen.insert(name.clone()) {
                    dupes.insert(name);
                }
            }
        }
        dupes
    }

    /// Builds the spec-conformant `IconSetFile` and validates it.
    pub fn to_file(&self, settings: &BuilderSettings) -> Result<IconSetFile, IconSetSpecError> {
        let sources: Vec<IconSetSourceSpec> = self
            .entries
            .iter()
            .map(|((index, subdirectory), group)| IconSetSourceSpec {
                location: IconSetSourceLocation::Crate {
                    name: ICON_SOURCES[*index].crate_name.into(),
                    subdirectory: subdirectory.map(str::to_string),
                },
                post_processors: group.post_processors.to_vec(),
                icons: group
                    .icons
                    .values()
                    .map(|icon| match &icon.rename {
                        Some(rename) => IconWhitelistEntry::Renamed {
                            name: icon.upstream.clone(),
                            rename: rename.clone(),
                        },
                        None => IconWhitelistEntry::Name(icon.upstream.clone()),
                    })
                    .collect(),
            })
            .collect();
        let spec = IconSetSpec {
            name: settings.name.clone(),
            display_name: settings.display_name.clone(),
            icon_prefix: settings.icon_prefix.clone(),
            gresource_prefix: settings.gresource_prefix.clone(),
            icons_context: settings.icons_context.clone(),
            sources,
        };
        spec.validate()?;
        Ok(IconSetFile { iconset: spec })
    }

    /// Emits the spec-conformant `icons.toml` text.
    pub fn to_toml(&self, settings: &BuilderSettings) -> Result<String, IconSelectionError> {
        let file = self.to_file(settings)?;
        toml::to_string_pretty(&file).map_err(IconSelectionError::Serialize)
    }
}

impl Default for IconSelection {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors when emitting the selection as `icons.toml`.
#[derive(Debug, thiserror::Error)]
pub enum IconSelectionError {
    /// The assembled spec failed `IconSetSpec::validate` (e.g. non-PascalCase
    /// `name`, exported-name collision left unresolved).
    #[error(transparent)]
    Spec(#[from] IconSetSpecError),
    /// TOML serialization failed.
    #[error("failed to serialize icons.toml: {0}")]
    Serialize(toml::ser::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> BuilderSettings {
        BuilderSettings {
            name: "AppIcons".to_string(),
            display_name: "App Icons".to_string(),
            icon_prefix: "app".to_string(),
            gresource_prefix: "/com/example/app/icons".to_string(),
            icons_context: "actions".to_string(),
        }
    }

    #[test]
    fn add_selects_via_grid_name() {
        let mut selection = IconSelection::new();
        assert!(selection.add("lucide-check-symbolic"));
        assert!(selection.add("tabler-heart-symbolic"));
        assert!(!selection.add("unknown-check-symbolic"));
        assert_eq!(selection.len(), 2);
        assert!(selection.contains("lucide-check-symbolic"));
        selection.remove_name("lucide-check-symbolic");
        assert!(!selection.contains("lucide-check-symbolic"));
    }

    #[test]
    fn styles_group_into_separate_sources() {
        let mut selection = IconSelection::new();
        selection.add("eva-checkmark-symbolic");
        selection.add("eva-checkmark-outline-symbolic");
        let spec = selection.to_file(&settings()).unwrap();
        let eva_sources: Vec<&IconSetSourceSpec> = spec
            .iconset
            .sources
            .iter()
            .filter(|source| matches!(&source.location, IconSetSourceLocation::Crate { name, .. } if name.as_str() == "fonts-rs-eva"))
            .collect();
        assert_eq!(eva_sources.len(), 2);
        let subdirs: Vec<Option<String>> = eva_sources
            .iter()
            .map(|source| match &source.location {
                IconSetSourceLocation::Crate { subdirectory, .. } => subdirectory.clone(),
                _ => None,
            })
            .collect();
        assert!(subdirs.contains(&Some("fill".to_string())));
        assert!(subdirs.contains(&Some("outline".to_string())));
    }

    #[test]
    fn toml_round_trips_through_spec_parse() {
        let mut selection = IconSelection::new();
        selection.add("lucide-check-symbolic");
        selection.add("lucide-trash-symbolic");
        let lucide = ICON_SOURCES.iter().position(|s| s.crate_name == "fonts-rs-lucide").unwrap();
        selection.set_rename((lucide, None), "trash", Some("delete".to_string()));

        let text = selection.to_toml(&settings()).unwrap();
        let parsed: IconSetFile = toml::from_str(&text).unwrap();
        parsed.iconset.validate().unwrap();

        assert_eq!(parsed.iconset.name, "AppIcons");
        assert_eq!(parsed.iconset.sources.len(), 1);
        assert!(matches!(
            parsed.iconset.sources[0].icons.as_slice(),
            [IconWhitelistEntry::Name(_), IconWhitelistEntry::Renamed { .. }]
        ));
    }

    #[test]
    fn collision_detected_across_sources() {
        let mut selection = IconSelection::new();
        selection.add("lucide-check-symbolic");
        selection.add("tabler-check-symbolic");
        let dupes = selection.collisions(&settings());
        assert!(dupes.contains("app-check-symbolic"));
    }

    #[test]
    fn rename_resolves_collision() {
        let mut selection = IconSelection::new();
        selection.add("lucide-check-symbolic");
        selection.add("tabler-check-symbolic");
        let lucide = ICON_SOURCES.iter().position(|s| s.crate_name == "fonts-rs-lucide").unwrap();
        selection.set_rename((lucide, None), "check", Some("check-lucide".to_string()));
        assert!(selection.collisions(&settings()).is_empty());
    }
}
