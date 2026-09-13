//! Error type for code generators.

use thiserror::Error;

/// Error returned by [`GlyphGenerator`](super::generate::GlyphGenerator) implementations.
#[derive(Debug, Error)]
pub enum GenerateError {
    /// An I/O error occurred while writing the generated output.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A required environment variable was not set.
    #[error("environment variable error: {0}")]
    Env(#[from] std::env::VarError),

    /// An upstream metadata source could not be parsed.
    #[error("metadata parse error ({label}): {source}")]
    Metadata {
        /// Human-readable label identifying the metadata source.
        label: String,
        /// The underlying parse error.
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
