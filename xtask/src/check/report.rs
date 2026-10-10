use crate::check::CheckRow;
use crate::upstream::is_commit_sha;
use std::fmt::Display;
use std::fmt::Formatter;

/// Aggregate check report over all vendored crates; `Display`
/// renders the per-crate table plus change details and the summary
/// line.
#[derive(Debug, Default)]
pub struct Report {
    /// One row per checked crate, in crate order.
    pub rows: Vec<CheckRow>,
}

impl From<Vec<CheckRow>> for Report {
    /// Wraps checked rows into a report.
    fn from(rows: Vec<CheckRow>) -> Self {
        Self { rows }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:<32} {:<12} {:<12} {:<14} status", "package", "ref", "locked", "latest")?;
        let mut counts = [0usize; 6];
        for row in &self.rows {
            let locked = row.locked.as_deref().map(shorten).unwrap_or("-");
            let latest = row.latest.as_deref().or(row.resolved.as_deref()).map(shorten).unwrap_or("-");
            let git_ref = row.git_ref.as_deref().map(shorten).unwrap_or("(urls)");
            writeln!(f, "{:<32} {:<12} {:<12} {:<14} {}", row.package, git_ref, locked, latest, row.status)?;
            for changed in &row.changed {
                writeln!(f, "    {} changed: {}", changed.kind, changed.url)?;
            }
            for fetch_error in &row.fetch_errors {
                writeln!(f, "    fetch failed: {fetch_error}")?;
            }
            if let Some(detail) = &row.detail {
                writeln!(f, "    {detail}")?;
            }
            let index = match row.status {
                "up-to-date" => 0,
                "changed" => 1,
                "behind" => 2,
                "stale" => 3,
                "unsynced" => 4,
                _ => 5,
            };
            counts[index] += 1;
        }
        writeln!(
            f,
            "\n{} crates: {} up-to-date, {} behind, {} stale, {} changed, {} unsynced, {} errors",
            self.rows.len(),
            counts[0],
            counts[2],
            counts[3],
            counts[1],
            counts[4],
            counts[5]
        )
    }
}

/// Truncates commit SHAs to 10 chars; leaves tags/branches as-is.
fn shorten(value: &str) -> &str {
    if is_commit_sha(value) { &value[..10] } else { value }
}
