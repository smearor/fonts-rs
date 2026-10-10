use crate::git_ref::GitRef;
use crate::git_repository::GitRepository;
use crate::manifest::License;
use serde::Deserialize;

/// `[upstream]`: the git source all artifacts are vendored from.
#[derive(Debug, Deserialize)]
pub struct Upstream {
    /// Repository shorthand `owner/name` (github.com) or a full git
    /// URL / SSH remote.
    pub repo: GitRepository,
    /// Tag, branch or commit SHA to pin (`ref` in TOML).
    #[serde(rename = "ref")]
    pub git_ref: GitRef,
    /// Additional sparse-checkout paths. The first path component of
    /// every artifact and license source is included automatically.
    #[serde(default)]
    pub sparse: Vec<String>,
    /// License file vendored verbatim into `resources/`.
    pub license: Option<License>,
}
