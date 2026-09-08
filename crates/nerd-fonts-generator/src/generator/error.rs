//! Error type for code generators.

/// Error returned by [`NerdFontsGenerator`](super::generate::NerdFontsGenerator) implementations.
#[derive(Debug)]
pub enum GenerateError {
    /// An I/O error occurred while writing the generated output.
    Io(std::io::Error),
}

impl From<std::io::Error> for GenerateError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for GenerateError {}
