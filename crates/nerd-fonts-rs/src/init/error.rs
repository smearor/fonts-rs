//! Error types for initialization failures.

/// Error returned by [`crate::InitOptions::init`] when initialization fails.
#[derive(Debug, thiserror::Error)]
pub enum InitError {
    /// Failed to register the nerd-fonts GResource bundles.
    #[error("failed to register nerd-fonts GResource: {0}")]
    GResourceRegister(String),
}
