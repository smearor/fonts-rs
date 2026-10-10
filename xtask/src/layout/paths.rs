use super::Package;
use super::Workspace;
use miette::miette;
use std::path::Path;
use std::path::PathBuf;

impl Workspace {
    /// The workspace root (the parent of the xtask manifest dir).
    pub fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask manifest dir has a parent")
            .to_path_buf()
    }
}

impl Package {
    /// The crate directory of this package (`crates/<name>`),
    /// failing when it does not exist.
    pub fn crate_dir(&self) -> miette::Result<PathBuf> {
        let dir = Workspace::root().join("crates").join(&**self);
        if dir.is_dir() {
            Ok(dir)
        } else {
            Err(miette!("Package '{self}' not found at {}", dir.display()))
        }
    }
}
