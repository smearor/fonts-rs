use crate::url::Url;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;

/// A git repository spec as declared in the manifest: `owner/name`
/// github shorthand, full clone URL or SSH remote. Serializes
/// transparently as the plain string.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GitRepository(
    /// Repository spec as written in `upstream.toml` / `upstream.lock`.
    String,
);

impl GitRepository {
    /// The git remote URL for this repository: `owner/name` expands
    /// to `https://github.com/owner/name.git`; values containing
    /// `://` or starting with `git@` are used as-is.
    pub fn remote_url(&self) -> Url {
        if self.0.contains("://") || self.0.starts_with("git@") {
            Url::from(self.0.clone())
        } else {
            Url::from(format!("https://github.com/{}.git", self.0))
        }
    }
}

impl Deref for GitRepository {
    type Target = str;

    /// Borrows the repo spec as `&str` (e.g. for `repo_key`).
    fn deref(&self) -> &str {
        &self.0
    }
}

impl Display for GitRepository {
    /// Writes the plain repo spec, honoring `Formatter`
    /// width/alignment.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<&str> for GitRepository {
    /// Compares against a plain string spec.
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl From<&str> for GitRepository {
    /// Wraps a repo spec string.
    fn from(repo: &str) -> Self {
        Self(repo.to_string())
    }
}

impl From<String> for GitRepository {
    /// Wraps a repo spec string.
    fn from(repo: String) -> Self {
        Self(repo)
    }
}
