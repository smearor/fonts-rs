//! Error type for [`GlyphSource`](crate::glyph_source::GlyphSource) implementations.

use read_fonts::ReadError;
use thiserror::Error;

use crate::export_error::ExportError;

/// Errors produced by [`GlyphSource`](crate::glyph_source::GlyphSource)
/// implementations and the shared export runner.
#[derive(Debug, Error)]
pub enum GlyphSourceError {
    /// An I/O error occurred while reading the source or writing artifacts.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The font data could not be parsed.
    #[error("failed to parse font: {0}")]
    FontParse(#[from] ReadError),

    /// An SVG filename could not be resolved to a glyph identity.
    #[error("unresolvable SVG filename: {0}")]
    UnresolvableFilename(String),

    /// A glyph has neither a filename codepoint nor a name-map entry.
    #[error("no codepoint for glyph: {0}")]
    MissingCodepoint(String),

    /// The Private Use Area is exhausted (more glyphs than slots).
    #[error("private use area exhausted")]
    PuaExhausted,

    /// JSON serialization of `metadata.json` failed.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// An error from the underlying font export machinery (`Font`,
    /// `skrifa`, GResource serialization).
    #[error("font export error: {0}")]
    Export(#[from] ExportError),

    /// A post-processing hook failed (malformed SVG/XML).
    #[error("SVG post-processing failed: {0}")]
    PostProcess(String),
}
