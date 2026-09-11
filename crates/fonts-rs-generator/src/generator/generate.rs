//! Trait for code generators that produce output from glyph metadata.

use super::error::GenerateError;
use fonts_rs_model::GlyphEntry;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Trait for code generators that produce output from glyph metadata.
///
/// Each implementation generates a specific artifact (Rust source, phf maps)
/// from a slice of [`GlyphEntry`] entries.
///
/// Generic over the font family's name type `N`.
pub trait GlyphGenerator<N: AsRef<str> + Clone + Ord + Serialize + for<'a> Deserialize<'a>> {
    /// Generates the output content from glyph metadata.
    fn generate(entries: &[GlyphEntry<N>]) -> Result<String, GenerateError>;

    /// Returns the destination path for the generated output.
    fn output_path() -> Result<PathBuf, GenerateError>;

    /// Generates the content and writes it to the output path.
    fn run(entries: &[GlyphEntry<N>]) -> Result<(), GenerateError> {
        let content = Self::generate(entries)?;
        let path = Self::output_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, content)?;
        Ok(())
    }
}
