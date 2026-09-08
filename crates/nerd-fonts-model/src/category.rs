//! Icon category label newtype for type-safe category handling.

use std::fmt;

/// A single category label for an icon.
///
/// Wraps a static string slice with type safety. Use [`IconCategory::as_str`]
/// or `AsRef<str>` to access the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IconCategory(&'static str);

impl IconCategory {
    /// Creates an `IconCategory` from a static string slice.
    pub const fn new(category: &'static str) -> Self {
        Self(category)
    }

    /// Returns the category as a string slice.
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for IconCategory {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl fmt::Display for IconCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl PartialEq<str> for IconCategory {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
