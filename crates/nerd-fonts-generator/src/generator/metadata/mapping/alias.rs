//! Raw alias newtype for type-safe alias handling in metadata parsing.

use std::borrow::Borrow;
use std::fmt;

use serde::Deserialize;

/// A single alias name from upstream icon metadata.
///
/// Wraps a `String` with type safety. Use [`RawAlias::as_str`] or
/// `AsRef<str>` to access the inner value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct RawAlias(String);

impl RawAlias {
    /// Creates a `RawAlias` from a string.
    pub fn new(alias: String) -> Self {
        Self(alias)
    }

    /// Returns the alias as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RawAlias {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for RawAlias {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RawAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<str> for RawAlias {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
