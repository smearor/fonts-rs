use crate::layout::Crate;
use crate::layout::Workspace;
use crate::lockfile::LOCKFILE_NAME;
use crate::lockfile::Lockfile;
use crate::sha256::sha256_file;
use miette::IntoDiagnostic;
use miette::miette;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

impl Workspace {
    /// Runs `cargo xtask verify [-p <package>]`: checks that every
    /// file recorded in `upstream.lock` exists with the recorded
    /// SHA-256. Without `-p`, verifies all vendored crates and fails
    /// when any of them reports missing or mismatched artifacts.
    pub fn verify(&self) -> miette::Result<()> {
        let mut failures = Vec::new();
        for vendored in self.crates() {
            match vendored.verify() {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("{}: {error:?}", vendored.package);
                    failures.push(vendored.package.to_string());
                }
            }
        }
        if !failures.is_empty() {
            return Err(miette!("verify failed for: {}", failures.join(", ")));
        }
        Ok(())
    }
}

impl Crate {
    /// Verifies this vendored crate against its `upstream.lock`.
    pub fn verify(&self) -> miette::Result<()> {
        let lockfile = Lockfile::load(&self.dir.join(LOCKFILE_NAME))?.ok_or_else(|| miette!("{} not found - run `cargo xtask sync` first", LOCKFILE_NAME))?;

        let mut missing = Vec::new();
        let mut mismatched = Vec::new();
        let mut locked: HashSet<&str> = HashSet::new();
        for artifact in &lockfile.artifacts {
            locked.insert(artifact.target.as_str());
            let path = self.dir.join(&artifact.target);
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
        for unaccounted in self.unaccounted_files(&locked)? {
            eprintln!("warning: unaccounted file (no lock entry): {}", unaccounted.display());
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

    /// Files under `resources/` that have no `[[artifacts]]` lock
    /// entry — unknown hand-adds, reported as warnings.
    fn unaccounted_files(&self, locked: &HashSet<&str>) -> miette::Result<Vec<PathBuf>> {
        let mut unaccounted = Vec::new();
        collect_files(&self.dir.join("resources"), &self.dir, locked, &mut unaccounted)?;
        unaccounted.sort();
        Ok(unaccounted)
    }
}

/// Recursively collects files under `dir` whose `crate_dir`-relative
/// path is absent from `locked`.
fn collect_files(dir: &Path, crate_dir: &Path, locked: &HashSet<&str>, unaccounted: &mut Vec<PathBuf>) -> miette::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir).into_diagnostic()? {
        let path = entry.into_diagnostic()?.path();
        if path.is_dir() {
            collect_files(&path, crate_dir, locked, unaccounted)?;
        } else if path.is_file() {
            let relative = path.strip_prefix(crate_dir).into_diagnostic()?;
            if !relative.to_str().is_some_and(|target| locked.contains(target)) {
                unaccounted.push(relative.to_path_buf());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Package;
    use crate::lockfile::LockedArtifact;
    use tempfile::TempDir;

    /// The vendored file used across the verify tests.
    const TARGET: &str = "resources/icon.svg";

    /// A temp crate dir holding one vendored file plus a matching
    /// `upstream.lock`.
    fn fixture() -> (TempDir, Crate) {
        let temp = TempDir::new().unwrap();
        let file = temp.path().join(TARGET);
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(&file, b"<svg/>").unwrap();
        let lockfile = Lockfile {
            artifacts: vec![LockedArtifact {
                target: TARGET.to_string(),
                sha256: sha256_file(&file).unwrap(),
                source: "icon.svg".to_string(),
            }],
            ..Lockfile::default()
        };
        lockfile.save(&temp.path().join(LOCKFILE_NAME)).unwrap();
        let vendored = Crate {
            package: Package::from("test-crate"),
            dir: temp.path().to_path_buf(),
        };
        (temp, vendored)
    }

    /// A clean tree passes verification.
    #[test]
    fn verify_ok_for_clean_tree() {
        let (_temp, vendored) = fixture();
        assert!(vendored.verify().is_ok());
    }

    /// A hand-edited vendored file fails verification.
    #[test]
    fn verify_fails_on_hand_edit() {
        let (temp, vendored) = fixture();
        std::fs::write(temp.path().join(TARGET), b"<svg>edited</svg>").unwrap();
        assert!(vendored.verify().is_err());
    }

    /// A deleted vendored file fails verification.
    #[test]
    fn verify_fails_on_deleted_file() {
        let (temp, vendored) = fixture();
        std::fs::remove_file(temp.path().join(TARGET)).unwrap();
        assert!(vendored.verify().is_err());
    }

    /// A file without a lock entry warns but does not fail.
    #[test]
    fn verify_warns_but_passes_on_unaccounted_file() {
        let (temp, vendored) = fixture();
        std::fs::write(temp.path().join("resources/hand-added.svg"), b"<svg/>").unwrap();
        assert!(vendored.verify().is_ok());
    }
}
