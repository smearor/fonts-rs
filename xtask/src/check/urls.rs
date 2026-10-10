use crate::check::ChangedUrl;
use crate::check::CheckRow;
use crate::fetch::FetchCache;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
use crate::sha256::sha256_hex;
use crate::upstream::is_commit_sha;

impl CheckRow {
    /// Re-fetches live URL artifacts and provider inputs and reports
    /// content drift on the row. Immutable URLs (commit pins,
    /// version-pinned registries, content-addressed CDNs, release
    /// assets) are skipped — they cannot drift by construction.
    /// Fetch failures land in `fetch_errors`: an unfetchable source
    /// must never masquerade as `up-to-date`.
    pub fn check_urls(&mut self, manifest: &Manifest, lockfile: Option<&Lockfile>, fetch: &FetchCache) {
        let Some(lockfile) = lockfile else { return };

        for artifact in &manifest.artifacts {
            let Some(url) = &artifact.url else { continue };
            if artifact.source.is_some() || is_immutable_url(url) {
                continue;
            }
            let Some(locked) = lockfile.artifacts.iter().find(|locked| locked.source == *url) else {
                continue;
            };
            self.compare_url(url, &locked.sha256, "artifact", fetch);
        }
        for input in &lockfile.inputs {
            if is_immutable_url(&input.url) {
                continue;
            }
            self.compare_url(&input.url, &input.sha256, "input", fetch);
        }
    }

    /// Fetches `url` and compares the body hash to `locked_sha256`:
    /// drift goes to `changed`, fetch failures to `fetch_errors`.
    fn compare_url(&mut self, url: &str, locked_sha256: &str, kind: &'static str, fetch: &FetchCache) {
        let body = match fetch.get(url) {
            Ok(body) => body,
            Err(error) => {
                self.fetch_errors.push(format!("{url}: {error:?}"));
                return;
            }
        };
        let current = sha256_hex(&body);
        if current != locked_sha256 {
            self.changed.push(ChangedUrl {
                kind,
                url: url.to_string(),
                locked_sha256: locked_sha256.to_string(),
                current_sha256: current,
            });
        }
    }
}

/// Whether a URL pins immutable content and can never drift:
/// commit-SHA-pinned raw files, version-pinned registries,
/// content-addressed CDNs and GitHub release assets.
fn is_immutable_url(url: &str) -> bool {
    url.contains("/releases/download/")
        || url.contains("fonts.gstatic.com/")
        || (url.contains("unpkg.com/") && url.contains('@'))
        || url.split('/').any(is_commit_sha)
}
