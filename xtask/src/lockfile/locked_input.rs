use serde::Deserialize;
use serde::Serialize;

/// One `[[inputs]]` lockfile entry: an external non-git resource a
/// provider fetched during sync (e.g. an iconify set JSON or a
/// website bundle).
#[derive(Debug, Deserialize, Serialize)]
pub struct LockedInput {
    /// The fetched URL.
    pub url: String,
    /// Lowercase hex SHA-256 of the response body.
    pub sha256: String,
    /// Provider that fetched this input.
    pub provider: String,
}
