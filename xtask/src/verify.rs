use crate::layout::Crate;
use crate::layout::Workspace;
use crate::lockfile::LOCKFILE_NAME;
use crate::lockfile::Lockfile;
use crate::sha256::sha256_file;
use miette::miette;

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
        for artifact in &lockfile.artifacts {
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
}
