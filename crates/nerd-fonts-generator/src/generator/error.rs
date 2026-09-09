//! Error type for code generators.

use thiserror::Error;

/// Error returned by [`NerdFontsGenerator`](super::generate::NerdFontsGenerator) implementations.
#[derive(Debug, Error)]
pub enum GenerateError {
    /// An I/O error occurred while writing the generated output.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
