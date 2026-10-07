use crate::lockfile::LOCKFILE_NAME;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
use crate::paths::crate_dir;
use crate::upstream::GitSource;

/// Manifest file name (same convention as sync).
const MANIFEST_NAME: &str = "upstream.toml";

/// Runs `cargo xtask check -p <package>`: resolves the manifest ref
/// and reports whether it moved since the last sync.
pub fn check(package: &str) -> miette::Result<()> {
    let crate_dir = crate_dir(package)?;
    let manifest = Manifest::load(&crate_dir.join(MANIFEST_NAME))?;
    let git = GitSource::new(&manifest.upstream.repo, manifest.upstream.repo_url());
    let commit = git.resolve(&manifest.upstream.git_ref)?;

    let Some(lockfile) = Lockfile::load(&crate_dir.join(LOCKFILE_NAME))? else {
        eprintln!("{package}: no {LOCKFILE_NAME} - never synced");
        return Ok(());
    };
    if lockfile.upstream.commit == commit {
        eprintln!("{package}: up to date ({}@{})", manifest.upstream.git_ref, &commit[..12]);
    } else {
        eprintln!(
            "{package}: update available: {} -> {commit} ({} ref '{}')",
            lockfile.upstream.commit, manifest.upstream.repo, manifest.upstream.git_ref
        );
    }
    Ok(())
}
