use serde::Serialize;

/// A drifted URL artifact or provider input.
#[derive(Debug, Serialize)]
pub struct ChangedUrl {
    /// `artifact` for `[[artifacts]]` `url` entries, `input` for
    /// provider `[[inputs]]`.
    pub kind: &'static str,
    /// The fetched URL.
    pub url: String,
    /// SHA-256 recorded in `upstream.lock`.
    pub locked_sha256: String,
    /// SHA-256 of the current response body.
    pub current_sha256: String,
}
