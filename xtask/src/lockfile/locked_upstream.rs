use crate::git_ref::GitRef;
use crate::git_repository::GitRepository;
use serde::Deserialize;
use serde::Serialize;

/// The `[upstream]` lockfile section: the manifest coordinates plus
/// the commit they resolved to at sync time.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LockedUpstream {
    /// Repository as declared in the manifest.
    pub repo: GitRepository,
    /// Ref as declared in the manifest (`ref` in TOML).
    #[serde(rename = "ref")]
    pub git_ref: GitRef,
    /// Commit SHA the ref resolved to at sync time.
    pub commit: String,
}
