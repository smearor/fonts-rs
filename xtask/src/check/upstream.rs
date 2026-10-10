use crate::check::CheckRow;
use crate::check::Status;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
use crate::upstream::GitSource;

impl CheckRow {
    /// Resolves the git upstream ref and classifies drift: `stale`
    /// when the ref moved past the lock, `behind` when a newer
    /// update target exists, `up-to-date` otherwise. Fills `repo`,
    /// `git_ref`, `locked`, `resolved` and `latest` on the row.
    pub fn check_upstream(&mut self, manifest: &Manifest, lockfile: Option<&Lockfile>) -> miette::Result<Status> {
        let Some(upstream) = &manifest.upstream else {
            return Ok(Status::UpToDate);
        };
        self.repo = Some(upstream.repo.clone());
        self.git_ref = Some(upstream.git_ref.clone());

        let Some(lockfile) = lockfile else {
            return Ok(Status::Unsynced);
        };
        let locked = lockfile.upstream.as_ref().map(|locked| locked.commit.as_str()).unwrap_or_default();
        self.locked = lockfile.upstream.as_ref().map(|locked| locked.commit.clone());

        let git = GitSource::new(&upstream.repo);
        let resolved = git.resolve(&upstream.git_ref)?;
        self.resolved = Some(resolved.clone());
        if resolved != locked {
            return Ok(Status::Stale);
        }
        match git.update_target(&upstream.git_ref)? {
            Some((latest_ref, latest_commit)) if latest_commit != locked => {
                self.latest = Some(latest_ref);
                Ok(Status::Behind)
            }
            _ => Ok(Status::UpToDate),
        }
    }
}
