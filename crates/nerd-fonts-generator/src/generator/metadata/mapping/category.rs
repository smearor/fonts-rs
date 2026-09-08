//! Raw category newtype for type-safe category handling in metadata parsing.

use std::fmt;

use serde::Deserialize;

/// A single category label from upstream icon metadata.
///
/// Wraps a `String` with type safety. Use [`RawCategory::as_str`] or
/// `AsRef<str>` to access the inner value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct RawCategory(String);

impl RawCategory {
    /// Creates a `RawCategory` from a string.
    pub fn new(category: String) -> Self {
        Self(category)
    }

    /// Returns the category as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RawCategory {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RawCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<str> for RawCategory {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
