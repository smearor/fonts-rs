//! Error type for glyph export operations.

use read_fonts::ReadError;
use thiserror::Error;

use crate::GenerateError;

/// Error returned by [`ExportConfig::export_glyphs`](crate::export_config::ExportConfig::export_glyphs)
/// and related glyph export methods.
#[derive(Debug, Error)]
pub enum ExportError {
    /// An I/O error occurred while reading or writing files.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The font data could not be parsed.
    #[error("failed to parse font: {0}")]
    FontParse(#[from] ReadError),

    /// JSON serialization or deserialization failed.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// XML serialization failed.
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::SeError),

    /// Code generation failed.
    #[error("code generation error: {0}")]
    Generate(#[from] GenerateError),
}
