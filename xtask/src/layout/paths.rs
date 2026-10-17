use super::Workspace;
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
