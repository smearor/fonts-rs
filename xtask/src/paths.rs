use miette::miette;
use std::path::Path;
use std::path::PathBuf;

/// Returns the workspace root (the parent of the xtask manifest dir).
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask manifest dir has a parent")
        .to_path_buf()
}

/// Returns the directory of workspace member `package` (`crates/<name>`).
pub fn crate_dir(package: &str) -> miette::Result<PathBuf> {
    let dir = workspace_root().join("crates").join(package);
    if dir.is_dir() {
        Ok(dir)
    } else {
        Err(miette!("Package '{package}' not found at {}", dir.display()))
    }
}
