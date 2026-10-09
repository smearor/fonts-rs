use crate::manifest::Artifact;
use crate::manifest::MetadataSpec;
use crate::manifest::Upstream;
use miette::IntoDiagnostic;
use miette::miette;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::Component;
use std::path::Path;
use std::str::FromStr;

/// The `upstream.toml` sourcing manifest of a vendored crate.
#[derive(Debug, Deserialize)]
pub struct Manifest {
    /// Git upstream artifacts are vendored from. Optional: crates
    /// sourced purely from `url` artifacts (release assets, pinned
    /// raw-file URLs, API endpoints) omit `[upstream]` entirely.
    pub upstream: Option<Upstream>,
    /// `[[artifacts]]` copy entries.
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
    /// `[[metadata]]` provider entries.
    #[serde(default)]
    pub metadata: Vec<MetadataSpec>,
}

impl Manifest {
    /// Reads, parses and validates the manifest at `path`.
    pub fn load(path: &Path) -> miette::Result<Self> {
        let text = std::fs::read_to_string(path)
            .into_diagnostic()
            .map_err(|e| e.wrap_err(format!("Failed to read {}", path.display())))?;
        let manifest: Self = text.parse()?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Validates the manifest: repo format, relative paths, unique
    /// targets and compilable glob patterns.
    pub fn validate(&self) -> miette::Result<()> {
        if let Some(upstream) = &self.upstream {
            if upstream.repo.is_empty() {
                return Err(miette!("upstream.repo must not be empty"));
            }
            if !upstream.repo.contains("://") && !upstream.repo.starts_with("git@") {
                let segments: Vec<&str> = upstream.repo.split('/').collect();
                if segments.len() != 2 || segments.iter().any(|s| s.is_empty()) {
                    return Err(miette!("upstream.repo '{}' is not a URL and not an 'owner/name' shorthand", upstream.repo));
                }
            }
            if upstream.git_ref.is_empty() {
                return Err(miette!("upstream.ref must not be empty"));
            }
            if let Some(license) = &upstream.license {
                check_relative("upstream.license.source", &license.source)?;
                check_relative("upstream.license.target", &license.target)?;
            }
        }
        for artifact in &self.artifacts {
            if let Some(source) = &artifact.source {
                check_relative("artifacts.source", source)?;
            }
            check_relative("artifacts.target", &artifact.target)?;
            if let Some(pattern) = &artifact.pattern {
                globset::Glob::new(pattern).map_err(|e| miette!("artifacts.pattern '{pattern}' is not a valid glob: {e}"))?;
            }
        }
        for spec in &self.metadata {
            if spec.provider.is_empty() {
                return Err(miette!("metadata.provider must not be empty"));
            }
            check_relative("metadata.output", &spec.output)?;
        }
        let mut targets = HashSet::new();
        if let Some(license) = self.upstream.as_ref().and_then(|upstream| upstream.license.as_ref()) {
            targets.insert(license.target.as_str());
        }
        for artifact in &self.artifacts {
            if !targets.insert(artifact.target.as_str()) {
                return Err(miette!("duplicate artifact target '{}'", artifact.target));
            }
        }
        for spec in &self.metadata {
            if !targets.insert(spec.output.as_str()) {
                return Err(miette!("metadata output '{}' collides with an artifact target", spec.output));
            }
        }
        Ok(())
    }
}

impl FromStr for Manifest {
    type Err = miette::Report;

    fn from_str(text: &str) -> miette::Result<Self> {
        toml::from_str(text).into_diagnostic().map_err(|e| e.wrap_err("Failed to parse upstream.toml"))
    }
}

/// Rejects absolute paths and `..` traversal for checkout-relative
/// and crate-relative manifest paths.
fn check_relative(field: &str, value: &str) -> miette::Result<()> {
    if value.is_empty() {
        return Err(miette!("{field} must not be empty"));
    }
    for component in Path::new(value).components() {
        match component {
            Component::RootDir | Component::ParentDir | Component::Prefix(_) => {
                return Err(miette!("{field} '{value}' must be a relative path inside the crate/checkout"));
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses the smallest valid manifest.
    #[test]
    fn parses_minimal_manifest() {
        let manifest: Manifest = r#"
            [upstream]
            repo = "tailwindlabs/heroicons"
            ref = "v2.2.0"
        "#
        .parse()
        .unwrap();
        let upstream = manifest.upstream.as_ref().unwrap();
        assert_eq!(upstream.repo, "tailwindlabs/heroicons");
        assert_eq!(upstream.git_ref, "v2.2.0");
        manifest.validate().unwrap();
    }

    /// Parses artifacts, license and metadata sections.
    #[test]
    fn parses_full_manifest() {
        let manifest: Manifest = r#"
            [upstream]
            repo = "tailwindlabs/heroicons"
            ref = "v2.2.0"

            [upstream.license]
            source = "LICENSE"
            target = "resources/LICENSE"

            [[artifacts]]
            source = "optimized/24/outline"
            target = "resources/svg/outline"
            pattern = "*.svg"

            [[metadata]]
            provider = "iconify_aliases"
            output = "resources/metadata/aliases.json"
            with.iconify_set = "heroicons"
            with.suffixes = ["-solid"]
        "#
        .parse()
        .unwrap();
        manifest.validate().unwrap();
        assert_eq!(manifest.artifacts.len(), 1);
        assert_eq!(manifest.metadata.len(), 1);
        assert_eq!(manifest.metadata[0].with.get("iconify_set").and_then(|v| v.as_str()), Some("heroicons"));
    }

    /// Rejects duplicate artifact targets.
    #[test]
    fn rejects_duplicate_targets() {
        let manifest: Manifest = r#"
            [upstream]
            repo = "o/r"
            ref = "main"

            [[artifacts]]
            source = "a"
            target = "resources/x"

            [[artifacts]]
            source = "b"
            target = "resources/x"
        "#
        .parse()
        .unwrap();
        assert!(manifest.validate().is_err());
    }

    /// Rejects path traversal in targets.
    #[test]
    fn rejects_parent_dir_traversal() {
        let manifest: Manifest = r#"
            [upstream]
            repo = "o/r"
            ref = "main"

            [[artifacts]]
            source = "a"
            target = "../escape"
        "#
        .parse()
        .unwrap();
        assert!(manifest.validate().is_err());
    }

    /// Rejects invalid `owner/name` shorthands.
    #[test]
    fn rejects_bad_repo_shorthand() {
        let manifest: Manifest = r#"
            [upstream]
            repo = "a/b/c"
            ref = "main"
        "#
        .parse()
        .unwrap();
        assert!(manifest.validate().is_err());
    }
}
