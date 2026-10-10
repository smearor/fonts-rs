mod changed_url;
mod check_row;
mod report;
mod status;
mod upstream;
mod urls;

pub use changed_url::ChangedUrl;
pub use check_row::CheckRow;
pub use report::Report;
pub use status::Status;

use crate::fetch::FetchCache;
use crate::layout::Crate;
use crate::layout::Workspace;
use crate::lockfile::LOCKFILE_NAME;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;

/// Manifest file name (same convention as sync).
const MANIFEST_NAME: &str = "upstream.toml";

impl Workspace {
    /// Runs `cargo xtask check [-p <package>] [--json]`: resolves
    /// every selected manifest's upstream ref and re-fetches live URL
    /// sources, then reports per-crate drift against `upstream.lock`.
    pub fn check(&self, json: bool) -> miette::Result<()> {
        let fetch = FetchCache::default();
        let mut rows = Vec::new();
        for vendored in self.crates() {
            match vendored.check(&fetch) {
                Ok(row) => rows.push(row),
                // Single-package runs propagate hard errors; aggregate
                // runs fold them into error rows so one broken crate
                // never hides the rest of the report.
                Err(error) if self.is_single() => return Err(error),
                Err(error) => {
                    let mut row = CheckRow::bare(&vendored.package, Status::Error);
                    row.detail = Some(format!("{error:?}"));
                    rows.push(row);
                }
            }
        }

        if json {
            println!("{}", serde_json::to_string_pretty(&rows).unwrap_or_default());
            return Ok(());
        }
        eprint!("{}", Report::from(rows));
        Ok(())
    }
}

impl Crate {
    /// Checks this vendored crate and returns its report row.
    pub fn check(&self, fetch: &FetchCache) -> miette::Result<CheckRow> {
        let manifest = Manifest::load(&self.dir.join(MANIFEST_NAME))?;
        let mut row = CheckRow::bare(&self.package, Status::UpToDate);

        let lockfile = Lockfile::load(&self.dir.join(LOCKFILE_NAME))?;
        let mut status = row.check_upstream(&manifest, lockfile.as_ref())?;
        row.check_urls(&manifest, lockfile.as_ref(), fetch);
        if !row.changed.is_empty() {
            status = status.max(Status::Changed);
        }
        row.status = status.label();
        Ok(row)
    }
}
