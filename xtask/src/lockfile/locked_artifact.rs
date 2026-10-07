use serde::Deserialize;
use serde::Serialize;

/// One `[[artifacts]]` lockfile entry: a vendored file with its
/// content hash.
#[derive(Debug, Deserialize, Serialize)]
pub struct LockedArtifact {
    /// Vendored path below the crate directory.
    pub target: String,
    /// Lowercase hex SHA-256 of the file content.
    pub sha256: String,
    /// Provenance: path inside the upstream checkout, or
    /// `provider:<name>` for generated files.
    pub source: String,
}
