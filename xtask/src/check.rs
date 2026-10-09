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
    let Some(upstream) = &manifest.upstream else {
        eprintln!("{package}: no git upstream - URL artifacts only; run `cargo xtask verify -p {package}`");
        return Ok(());
    };
    let git = GitSource::new(&upstream.repo, upstream.repo_url());
    let commit = git.resolve(&upstream.git_ref)?;

    let Some(lockfile) = Lockfile::load(&crate_dir.join(LOCKFILE_NAME))? else {
        eprintln!("{package}: no {LOCKFILE_NAME} - never synced");
        return Ok(());
    };
    let locked_commit = lockfile.upstream.as_ref().map(|locked| locked.commit.as_str()).unwrap_or("");
    if locked_commit == commit {
        eprintln!("{package}: up to date ({}@{})", upstream.git_ref, &commit[..12]);
    } else {
        eprintln!("{package}: update available: {} -> {commit} ({} ref '{}')", locked_commit, upstream.repo, upstream.git_ref);
    }
    Ok(())
}
