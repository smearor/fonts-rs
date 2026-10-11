use serde::Serialize;

/// Check status of one vendored crate, ordered by severity (`Error`
/// worst, `UpToDate` clean).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    /// Everything matches the lockfile and the latest upstream.
    UpToDate,
    /// One or more live URL artifacts/provider inputs changed.
    Changed,
    /// The pinned commit is behind the update target (`sync --update`).
    Behind,
    /// The manifest ref resolves away from the locked commit
    /// (branch moved or manual re-pin) - `sync` restores consistency.
    Stale,
    /// No `upstream.lock` exists yet.
    Unsynced,
    /// The check itself failed (network, manifest, …).
    Error,
}

impl Status {
    /// Stable display label for table and JSON output.
    pub fn label(self) -> &'static str {
        match self {
            Status::UpToDate => "up-to-date",
            Status::Changed => "changed",
            Status::Behind => "behind",
            Status::Stale => "stale",
            Status::Unsynced => "unsynced",
            Status::Error => "error",
        }
    }
}
