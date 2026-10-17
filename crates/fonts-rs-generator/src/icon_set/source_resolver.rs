//! The [`IconSetSourceResolver`] - maps `icons.toml` source locations
//! to absolute SVG directories.

use crate::icon_set::error::IconSetBuildError;
use fonts_rs_model::IconSetSourceLocation;
use fonts_rs_model::Package;
use serde::Deserialize;
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Resolves [`IconSetSourceLocation`]s to absolute SVG directories.
///
/// Resolution order for `crate = "…"` locations:
///
/// 1. `DEP_*_SVG_DIR` links metadata from the source crate's own
///    `build.rs` (primary; requires the crate as a *normal*
///    dependency - `links` metadata does not propagate through
///    `[build-dependencies]`).
/// 2. `cargo metadata` fallback: find the package by name and resolve
///    `<manifest_dir>/resources/svg`. Emits a `cargo:warning`.
///
/// `dir = "…"` locations resolve against `CARGO_MANIFEST_DIR`.
pub struct IconSetSourceResolver {
    /// `CARGO_MANIFEST_DIR` of the building crate (current dir as
    /// fallback for non-build-script contexts, e.g. preview tooling).
    manifest_dir: PathBuf,
}

/// Minimal `cargo metadata` output - only the fields we read.
#[derive(Deserialize)]
struct CargoMetadata {
    /// Workspace/registry packages in the dependency graph.
    packages: Vec<MetadataPackage>,
}

/// One package entry of `cargo metadata` output.
#[derive(Deserialize)]
struct MetadataPackage {
    /// Package name.
    name: String,
    /// Absolute path of the package's `Cargo.toml`.
    manifest_path: PathBuf,
}

impl IconSetSourceResolver {
    /// Creates a resolver rooted at `CARGO_MANIFEST_DIR`.
    pub fn new() -> Self {
        Self {
            manifest_dir: env::var_os("CARGO_MANIFEST_DIR").map_or_else(|| PathBuf::from("."), PathBuf::from),
        }
    }

    /// Resolves one source location to an existing directory.
    ///
    /// # Errors
    ///
    /// Returns [`IconSetBuildError::SourceNotFound`] when no resolution
    /// path yields an existing directory.
    pub fn resolve(&self, location: &IconSetSourceLocation) -> Result<PathBuf, IconSetBuildError> {
        match location {
            IconSetSourceLocation::Dir { dir } => {
                let resolved = self.manifest_dir.join(dir);
                if resolved.is_dir() {
                    Ok(resolved)
                } else {
                    Err(IconSetBuildError::SourceNotFound {
                        source_id: location.label(),
                        hint: format!("resolved to {}, which is not a directory", resolved.display()),
                    })
                }
            }
            IconSetSourceLocation::Crate { name, subdirectory } => self.resolve_crate(location, name, subdirectory.as_deref()),
        }
    }

    /// Resolves a `crate = "…"` source: `DEP_*` links metadata first,
    /// `cargo metadata` as fallback.
    fn resolve_crate(&self, location: &IconSetSourceLocation, name: &Package, subdirectory: Option<&str>) -> Result<PathBuf, IconSetBuildError> {
        let mut tried = Vec::new();
        for var in name.dep_env_vars("SVG_DIR") {
            tried.push(var.clone());
            if let Some(dir) = env::var_os(&var) {
                return self.check_dir(PathBuf::from(dir), subdirectory, location);
            }
        }

        if let Some(svg_root) = self.resolve_via_cargo_metadata(name.as_str()) {
            println!("cargo:warning=icons.toml: '{name}' resolved via `cargo metadata` (no DEP_* links metadata)");
            return self.check_dir(svg_root, subdirectory, location);
        }
        tried.push("cargo metadata".to_string());

        Err(IconSetBuildError::SourceNotFound {
            source_id: location.label(),
            hint: format!("tried {}; is '{name}' a dependency of this crate?", tried.join(", ")),
        })
    }

    /// Joins the optional `subdirectory` onto a resolved SVG root and
    /// verifies the result exists.
    fn check_dir(&self, svg_root: PathBuf, subdirectory: Option<&str>, location: &IconSetSourceLocation) -> Result<PathBuf, IconSetBuildError> {
        let dir = match subdirectory {
            Some(sub) => svg_root.join(sub),
            None => svg_root,
        };
        if dir.is_dir() {
            Ok(dir)
        } else {
            Err(IconSetBuildError::SourceNotFound {
                source_id: location.label(),
                hint: format!("resolved to {}, which is not a directory", dir.display()),
            })
        }
    }

    /// `cargo metadata` fallback: finds the package by name and returns
    /// its `resources/svg` root.
    fn resolve_via_cargo_metadata(&self, name: &str) -> Option<PathBuf> {
        let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
        let output = Command::new(cargo).args(["metadata", "--format-version", "1"]).output().ok()?;
        if !output.status.success() {
            return None;
        }
        let metadata: CargoMetadata = serde_json::from_slice(&output.stdout).ok()?;
        metadata
            .packages
            .iter()
            .find(|package| package.name == name)
            .and_then(|package| package.manifest_path.parent().map(|dir| dir.join("resources/svg")))
    }
}

impl Default for IconSetSourceResolver {
    fn default() -> Self {
        Self::new()
    }
}
