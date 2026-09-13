//! Glyph search keyword newtype for type-safe keyword handling.

use std::fmt;

/// A single search keyword for a glyph.
///
/// Wraps a static string slice with type safety. Use [`GlyphKeyword::as_str`]
/// or `AsRef<str>` to access the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GlyphKeyword(&'static str);

impl GlyphKeyword {
    /// Creates a `GlyphKeyword` from a static string slice.
    pub const fn new(keyword: &'static str) -> Self {
        Self(keyword)
    }

    /// Returns the keyword as a string slice.
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for GlyphKeyword {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl fmt::Display for GlyphKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl PartialEq<str> for GlyphKeyword {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
