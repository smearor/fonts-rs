use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;

/// Name of a vendored crate directory (e.g. `fonts-rs-tabler`).
/// Serializes transparently as the plain name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Package(
    /// Crate directory name under `crates/`.
    String,
);

impl Deref for Package {
    type Target = str;

    /// Borrows the package name as `&str` (e.g. for `crate_dir`).
    fn deref(&self) -> &str {
        &self.0
    }
}

impl Display for Package {
    /// Writes the plain crate directory name, honoring `Formatter`
    /// width/alignment (report table pads via `{:<32}`).
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for Package {
    /// Wraps a crate directory name.
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for Package {
    /// Wraps a crate directory name.
    fn from(name: String) -> Self {
        Self(name)
    }
}
