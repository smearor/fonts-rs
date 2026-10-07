use serde::Deserialize;

/// `[upstream.license]`: the license file vendored verbatim into
/// `resources/`.
#[derive(Debug, Deserialize)]
pub struct License {
    /// Path of the license file inside the upstream checkout.
    pub source: String,
    /// Vendored path below the crate directory.
    pub target: String,
}
