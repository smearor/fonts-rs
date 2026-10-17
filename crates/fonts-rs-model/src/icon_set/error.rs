//! The [`IconSetSpecError`] error type for `icons.toml` validation.

use thiserror::Error;

/// Error returned by
/// [`IconSetSpec::validate`](crate::icon_set::IconSetSpec::validate) -
/// covers every spec violation that can be detected from the document
/// alone, without touching the filesystem.
#[derive(Debug, Clone, Error)]
pub enum IconSetSpecError {
    /// The `sources` list is empty.
    #[error("icon set must declare at least one [[iconset.sources]] table")]
    EmptySources,

    /// An identifier or name violates its naming convention.
    #[error("invalid {what}: '{value}' - expected {expected}")]
    InvalidIdent {
        /// Which field failed validation (e.g. `iconset.name`).
        what: &'static str,
        /// The offending value.
        value: String,
        /// The expected format (e.g. `PascalCase`, `kebab-case`).
        expected: &'static str,
    },

    /// A field is empty where a value is required.
    #[error("{what} must not be empty")]
    EmptyField {
        /// Which field is empty (e.g. `iconset.display_name`).
        what: &'static str,
    },

    /// The same upstream name is whitelisted twice within one source.
    #[error("duplicate whitelist entry '{name}' in {source_id}")]
    DuplicateIcon {
        /// Source identifier (crate name or directory).
        source_id: String,
        /// Duplicated upstream icon name.
        name: String,
    },

    /// Two whitelist entries resolve to the same exported icon name,
    /// within one source or across sources.
    #[error("icon name collision: '{name}' is exported more than once - use a rename to disambiguate")]
    IconNameCollision {
        /// The colliding exported base name.
        name: String,
    },
}
