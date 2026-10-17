//! The [`IconSetSpec`] struct - serde model of `[iconset]`.

use crate::icon_set::error::IconSetSpecError;
use crate::icon_set::source_spec::IconSetSourceSpec;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;

fn default_icons_context() -> String {
    "actions".to_string()
}

/// Serde model of the application's `icons.toml` `[iconset]` table.
///
/// The schema is the wire format both ways: `IconSetBuild` reads it at
/// build time, tooling like the icon-set builder writes it. Always
/// wrap in [`IconSetFile`](crate::icon_set::IconSetFile) for
/// (de)serialization - the fields live under the `[iconset]` table.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IconSetSpec {
    /// PascalCase marker enum name, e.g. `MyAppIcons`.
    pub name: String,
    /// Human-readable display name for build logs.
    pub display_name: String,
    /// Glyph name prefix producing `{icon_prefix}-{name}-symbolic`
    /// icon names.
    pub icon_prefix: String,
    /// GResource prefix for the compiled bundle.
    pub gresource_prefix: String,
    /// Freedesktop icon context (`actions` by default).
    #[serde(default = "default_icons_context")]
    pub icons_context: String,
    /// SVG sources merged into this set, in TOML order. Must be
    /// non-empty.
    pub sources: Vec<IconSetSourceSpec>,
}

impl IconSetSpec {
    /// Validates the spec: naming conventions, non-empty fields and
    /// sources, no duplicate whitelist entries, no exported-name
    /// collisions across sources.
    ///
    /// Covers everything decidable from the document alone -
    /// filesystem-dependent checks (unknown icons, source resolution)
    /// run later in `IconSetBuild`.
    ///
    /// # Errors
    ///
    /// Returns [`IconSetSpecError`] describing the first violation.
    pub fn validate(&self) -> Result<(), IconSetSpecError> {
        if !is_pascal_case(&self.name) {
            return Err(IconSetSpecError::InvalidIdent {
                what: "iconset.name",
                value: self.name.clone(),
                expected: "PascalCase marker name",
            });
        }
        if self.display_name.is_empty() {
            return Err(IconSetSpecError::EmptyField { what: "iconset.display_name" });
        }
        if !is_kebab(&self.icon_prefix) {
            return Err(IconSetSpecError::InvalidIdent {
                what: "iconset.icon_prefix",
                value: self.icon_prefix.clone(),
                expected: "kebab-case icon prefix",
            });
        }
        if !self.gresource_prefix.starts_with('/') || self.gresource_prefix.len() < 2 {
            return Err(IconSetSpecError::InvalidIdent {
                what: "iconset.gresource_prefix",
                value: self.gresource_prefix.clone(),
                expected: "absolute GResource path prefix, e.g. /com/example/app/icons",
            });
        }
        if self.sources.is_empty() {
            return Err(IconSetSpecError::EmptySources);
        }

        let mut exported: HashMap<String, String> = HashMap::new();
        for source in &self.sources {
            let source_label = source.location.label();
            let mut upstream: HashSet<&str> = HashSet::new();
            for entry in &source.icons {
                if !upstream.insert(entry.upstream_name()) {
                    return Err(IconSetSpecError::DuplicateIcon {
                        source_id: source_label.clone(),
                        name: entry.upstream_name().to_string(),
                    });
                }
                if !is_kebab(entry.exported_name()) {
                    return Err(IconSetSpecError::InvalidIdent {
                        what: "iconset.icons entry",
                        value: entry.exported_name().to_string(),
                        expected: "kebab-case icon name",
                    });
                }
                if let Some(existing) = exported.insert(entry.exported_name().to_string(), source_label.clone()) {
                    let _ = existing;
                    return Err(IconSetSpecError::IconNameCollision {
                        name: entry.exported_name().to_string(),
                    });
                }
            }
        }
        Ok(())
    }
}

/// Checks a PascalCase identifier: first char uppercase ASCII, rest
/// alphanumeric.
fn is_pascal_case(value: &str) -> bool {
    let mut chars = value.chars();
    chars.next().is_some_and(|first| first.is_ascii_uppercase()) && chars.all(|ch| ch.is_ascii_alphanumeric())
}

/// Checks strict kebab-case: lowercase ASCII letters, digits and single
/// dashes between segments.
fn is_kebab(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
        && value.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_spec() -> IconSetSpec {
        IconSetSpec {
            name: "MyAppIcons".to_string(),
            display_name: "My App Icons".to_string(),
            icon_prefix: "myapp".to_string(),
            gresource_prefix: "/com/example/myapp/icons".to_string(),
            icons_context: "actions".to_string(),
            sources: vec![IconSetSourceSpec {
                location: crate::icon_set::IconSetSourceLocation::Dir { dir: "icons".to_string() },
                post_processors: Vec::new(),
                icons: vec![crate::icon_set::IconWhitelistEntry::Name("check".to_string())],
            }],
        }
    }

    #[test]
    fn valid_spec_passes() {
        assert!(valid_spec().validate().is_ok());
    }

    #[test]
    fn empty_sources_fail() {
        let mut spec = valid_spec();
        spec.sources.clear();
        assert!(matches!(spec.validate(), Err(IconSetSpecError::EmptySources)));
    }

    #[test]
    fn snake_case_marker_fails() {
        let mut spec = valid_spec();
        spec.name = "my_app_icons".to_string();
        assert!(matches!(spec.validate(), Err(IconSetSpecError::InvalidIdent { what: "iconset.name", .. })));
    }

    #[test]
    fn collision_across_sources_fails() {
        let mut spec = valid_spec();
        spec.sources.push(IconSetSourceSpec {
            location: crate::icon_set::IconSetSourceLocation::Dir { dir: "other".to_string() },
            post_processors: Vec::new(),
            icons: vec![crate::icon_set::IconWhitelistEntry::Name("check".to_string())],
        });
        assert!(matches!(spec.validate(), Err(IconSetSpecError::IconNameCollision { .. })));
    }

    #[test]
    fn rename_resolves_collision() {
        let mut spec = valid_spec();
        spec.sources.push(IconSetSourceSpec {
            location: crate::icon_set::IconSetSourceLocation::Dir { dir: "other".to_string() },
            post_processors: Vec::new(),
            icons: vec![crate::icon_set::IconWhitelistEntry::Renamed {
                name: "check".to_string(),
                rename: "check-filled".to_string(),
            }],
        });
        assert!(spec.validate().is_ok());
    }
}
