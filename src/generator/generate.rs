//! Trait for code generators that produce output from icon metadata.

use std::fs;
use std::path::PathBuf;

use super::IconEntry;
use super::error::GenerateError;

/// Trait for code generators that produce output from icon metadata.
///
/// Each implementation generates a specific artifact (Rust source, phf maps,
/// CSS) from a slice of [`IconEntry`] entries.
pub trait NerdFontsGenerator {
    /// Generates the output content from icon metadata.
    fn generate(icons: &[IconEntry]) -> Result<String, GenerateError>;
    /// Returns the destination path for the generated output.
    fn output_path() -> PathBuf;
    /// Generates the content and writes it to the output path.
    fn run(icons: &[IconEntry]) -> Result<(), GenerateError> {
        let content = Self::generate(icons)?;
        let path = Self::output_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, content)?;
        Ok(())
    }
}
