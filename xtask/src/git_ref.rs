use crate::upstream::is_commit_sha;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;

/// A git ref as declared in a manifest (`ref` key) or lockfile:
/// tag, branch or full commit SHA. Serializes transparently as the
/// plain string.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GitRef(
    /// The ref string (tag, branch name or 40-hex commit SHA).
    String,
);

impl GitRef {
    /// Whether this ref is a full 40-hex commit SHA pin.
    pub fn is_commit_sha(&self) -> bool {
        is_commit_sha(&self.0)
    }
}

impl Deref for GitRef {
    type Target = str;

    /// Borrows the ref as `&str`.
    fn deref(&self) -> &str {
        &self.0
    }
}

impl Display for GitRef {
    /// Writes the plain ref, honoring `Formatter` width/alignment
    /// (report table pads via `{:<12}`).
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<&str> for GitRef {
    /// Compares against a plain ref string.
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl From<&str> for GitRef {
    /// Wraps a ref string.
    fn from(git_ref: &str) -> Self {
        Self(git_ref.to_string())
    }
}

impl From<String> for GitRef {
    /// Wraps a ref string.
    fn from(git_ref: String) -> Self {
        Self(git_ref)
    }
}
