//! Error types for initialization failures.

/// Error returned by [`crate::InitOptions::init`] when initialization fails.
#[derive(Debug, thiserror::Error)]
pub enum InitError {
    /// Failed to register the main nerd-fonts GResource bundle.
    #[error("failed to register nerd-fonts GResource: {0}")]
    GResourceRegister(String),
    /// Failed to register the vendored icon GResource bundle.
    #[error("failed to register nerd font icons: {0}")]
    IconRegister(String),
}
