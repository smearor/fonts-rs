use crate::manifest::License;
use serde::Deserialize;

/// `[upstream]`: the git source all artifacts are vendored from.
#[derive(Debug, Deserialize)]
pub struct Upstream {
    /// Repository shorthand `owner/name` (github.com) or a full git
    /// URL / SSH remote.
    pub repo: String,
    /// Tag, branch or commit SHA to pin (`ref` in TOML).
    #[serde(rename = "ref")]
    pub git_ref: String,
    /// Additional sparse-checkout paths. The first path component of
    /// every artifact and license source is included automatically.
    #[serde(default)]
    pub sparse: Vec<String>,
    /// License file vendored verbatim into `resources/`.
    pub license: Option<License>,
}

impl Upstream {
    /// Returns the git remote URL for the declared repository.
    ///
    /// `owner/name` expands to `https://github.com/owner/name.git`;
    /// values containing `://` or starting with `git@` are used as-is.
    pub fn repo_url(&self) -> String {
        if self.repo.contains("://") || self.repo.starts_with("git@") {
            self.repo.clone()
        } else {
            format!("https://github.com/{}.git", self.repo)
        }
    }
}
