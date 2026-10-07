use crate::lockfile::LOCKFILE_NAME;
use crate::lockfile::Lockfile;
use crate::paths::crate_dir;
use crate::sha256::sha256_file;
use miette::miette;

/// Runs `cargo xtask verify -p <package>`: checks that every file
/// recorded in `upstream.lock` exists with the recorded SHA-256.
pub fn verify(package: &str) -> miette::Result<()> {
    let crate_dir = crate_dir(package)?;
    let lockfile =
        Lockfile::load(&crate_dir.join(LOCKFILE_NAME))?.ok_or_else(|| miette!("{} not found - run `cargo xtask sync -p {package}` first", LOCKFILE_NAME))?;

    let mut missing = Vec::new();
    let mut mismatched = Vec::new();
    for artifact in &lockfile.artifacts {
        let path = crate_dir.join(&artifact.target);
        if !path.is_file() {
            missing.push(artifact.target.clone());
            continue;
        }
        if sha256_file(&path)? != artifact.sha256 {
            mismatched.push(artifact.target.clone());
        }
    }

    for target in &missing {
        eprintln!("missing:   {target}");
    }
    for target in &mismatched {
        eprintln!("mismatch:  {target}");
    }
    let problems = missing.len() + mismatched.len();
    if problems > 0 {
        return Err(miette!(
            "verify failed: {} missing, {} mismatched of {} artifacts",
            missing.len(),
            mismatched.len(),
            lockfile.artifacts.len()
        ));
    }
    eprintln!("verify ok: {} artifacts match {}", lockfile.artifacts.len(), LOCKFILE_NAME);
    Ok(())
}
