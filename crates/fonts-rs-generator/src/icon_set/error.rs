//! The [`IconSetBuildError`] error type for custom icon set builds.

use crate::generator::error::GenerateError;
use crate::glyph_source::error::GlyphSourceError;
use fonts_rs_model::IconSetSpecError;
use thiserror::Error;

/// Error returned by [`IconSetBuild`](crate::icon_set::IconSetBuild)
/// operations.
///
/// Implements `miette::Diagnostic`, so `?` converts directly into
/// `miette::Result` - the `build.rs` one-liner works unchanged.
#[derive(Debug, Error, miette::Diagnostic)]
pub enum IconSetBuildError {
    /// The `icons.toml` spec file does not exist.
    #[error("icon set spec not found: {path}")]
    SpecNotFound {
        /// Path that was looked up.
        path: String,
    },

    /// The `icons.toml` spec file failed to parse.
    #[error("failed to parse icon set spec {path}: {parse_error}")]
    SpecParse {
        /// Path of the spec file.
        path: String,
        /// Underlying TOML parse error.
        parse_error: toml::de::Error,
    },

    /// The spec failed schema validation (naming conventions, empty
    /// sources, duplicates, name collisions).
    #[error(transparent)]
    Spec(#[from] IconSetSpecError),

    /// A whitelisted upstream icon matched no file in its source
    /// directory.
    #[error("unknown icon '{name}' in {source_id} (closest matches: {suggestions})")]
    UnknownIcon {
        /// Source identifier (crate name or directory).
        source_id: String,
        /// Whitelisted upstream icon name.
        name: String,
        /// Closest matching file stems in the source, or `none`.
        suggestions: String,
    },

    /// A source location could not be resolved to a directory.
    #[error("could not resolve icon source {source_id}: {hint}")]
    SourceNotFound {
        /// Source identifier (crate name or directory).
        source_id: String,
        /// What was tried (`DEP_*` variables, `cargo metadata`,
        /// filesystem).
        hint: String,
    },

    /// A [`GlyphSource`] export failed.
    #[error("glyph source error: {0}")]
    GlyphSource(#[from] GlyphSourceError),

    /// An I/O error occurred while reading sources or writing output.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization of the glyph entries failed.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Code generation failed.
    #[error("code generation failed: {0}")]
    Generate(#[from] GenerateError),
}
