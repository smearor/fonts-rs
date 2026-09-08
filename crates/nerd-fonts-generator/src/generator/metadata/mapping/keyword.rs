//! Raw keyword newtype for type-safe keyword handling in metadata parsing.

use std::fmt;

use serde::Deserialize;

/// A single keyword from upstream icon metadata.
///
/// Wraps a `String` with type safety. Use [`RawKeyword::as_str`] or
/// `AsRef<str>` to access the inner value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct RawKeyword(String);

impl RawKeyword {
    /// Creates a `RawKeyword` from a string.
    pub fn new(keyword: String) -> Self {
        Self(keyword)
    }

    /// Returns the keyword as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RawKeyword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RawKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<str> for RawKeyword {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
