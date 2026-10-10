use crate::check::ChangedUrl;
use crate::check::Status;
use crate::git_ref::GitRef;
use crate::git_repository::GitRepository;
use crate::layout::Package;
use serde::Serialize;

/// One row of the check report.
#[derive(Debug, Serialize)]
pub struct CheckRow {
    /// Vendored crate this row reports on.
    pub package: Package,
    /// Worst status across the git upstream and URL checks.
    pub status: &'static str,
    /// `[upstream]` repository, when the manifest has a git source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<GitRepository>,
    /// Manifest `ref` (tag, branch or commit pin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_ref: Option<GitRef>,
    /// Commit recorded in `upstream.lock`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<String>,
    /// Commit the manifest ref resolves to right now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<String>,
    /// Re-pin target: default HEAD for commit pins, newest version
    /// tag for tag pins (`sync --update` applies this).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<GitRef>,
    /// Live URLs whose content drifted from the lock.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub changed: Vec<ChangedUrl>,
    /// URLs that could not be fetched, so drift is unknown.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fetch_errors: Vec<String>,
    /// Error detail when `status` is `error`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl CheckRow {
    /// A row for `package` with the remaining fields empty.
    pub fn bare(package: &Package, status: Status) -> Self {
        Self {
            package: package.clone(),
            status: status.label(),
            repo: None,
            git_ref: None,
            locked: None,
            resolved: None,
            latest: None,
            changed: Vec::new(),
            fetch_errors: Vec::new(),
            detail: None,
        }
    }
}
