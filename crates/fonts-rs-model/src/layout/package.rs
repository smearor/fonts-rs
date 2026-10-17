//! The [`Package`] newtype - a workspace crate/package name.

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

/// A workspace package name - the crate directory name and crates.io
/// package name in one (e.g. `fonts-rs-tabler`). Serializes
/// transparently as the plain name.
///
/// Shared by the xtask tooling (crate dirs under `crates/`) and the
/// icon-set spec (`crate = "…"` source locations, `DEP_*` links
/// metadata naming).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Package(
    /// Package name - `crates/<name>` / crates.io identifier.
    String,
);

impl Package {
    /// The package name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// `DEP_*` environment variable candidates for a `cargo:<key>`
    /// metadata value emitted by this crate's `links` build script.
    ///
    /// The `links` values of fonts-rs crates drop the `fonts-rs-`
    /// prefix (e.g. `links = "lucide"` → `DEP_LUCIDE_SVG_DIR`); both
    /// the stripped and the full package name are tried so foreign
    /// crates with full-name links values resolve too.
    pub fn dep_env_vars(&self, key: &str) -> Vec<String> {
        let upper = |name: &str| name.to_uppercase().replace('-', "_");
        let stripped = upper(self.0.strip_prefix("fonts-rs-").unwrap_or(&self.0));
        let mut vars = vec![format!("DEP_{stripped}_{key}")];
        let full = format!("DEP_{}_{key}", upper(&self.0));
        if !vars.contains(&full) {
            vars.push(full);
        }
        vars
    }

    /// The crate directory `<workspace_root>/crates/<name>`, when it
    /// exists.
    pub fn crate_dir(&self, workspace_root: &Path) -> Option<PathBuf> {
        let dir = workspace_root.join("crates").join(&**self);
        dir.is_dir().then_some(dir)
    }
}

impl Deref for Package {
    type Target = str;

    /// Borrows the package name as `&str`.
    fn deref(&self) -> &str {
        &self.0
    }
}

impl Display for Package {
    /// Writes the plain package name, honoring `Formatter`
    /// width/alignment (report tables pad via `{:<32}`).
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for Package {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for Package {
    /// Wraps a package name.
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for Package {
    /// Wraps a package name.
    fn from(name: String) -> Self {
        Self(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dep_env_vars_prefers_stripped_fonts_rs_prefix() {
        let vars = Package("fonts-rs-lucide".to_string()).dep_env_vars("SVG_DIR");
        assert_eq!(vars[0], "DEP_LUCIDE_SVG_DIR");
        assert_eq!(vars[1], "DEP_FONTS_RS_LUCIDE_SVG_DIR");
    }

    #[test]
    fn dep_env_vars_dedupes_foreign_names() {
        let vars = Package("some-icons".to_string()).dep_env_vars("SVG_DIR");
        assert_eq!(vars, vec!["DEP_SOME_ICONS_SVG_DIR".to_string()]);
    }

    #[test]
    fn crate_dir_resolves_under_workspace_root() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest_dir.parent().and_then(Path::parent).expect("workspace root");
        assert!(Package("fonts-rs-model".to_string()).crate_dir(root).is_some());
        assert!(Package("fonts-rs-nope".to_string()).crate_dir(root).is_none());
    }
}
