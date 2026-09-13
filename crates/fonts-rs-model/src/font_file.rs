//! Font file identifier type.
//!
//! Provides a [`FontFile`] newtype wrapping a `&'static str` font file name
//! for type-safe font file references in build scripts.

/// A font file name without extension, e.g. `"DSEG7Classic-Regular"`, `"Doto"`.
///
/// Wraps a `&'static str` to provide type-safe font file identification.
/// The file extension (e.g. `.ttf`, `.otf`) is appended by the build script.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontFile(&'static str);

impl FontFile {
    /// Creates a new [`FontFile`] from a static string.
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the font file name as a string slice.
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl From<&'static str> for FontFile {
    fn from(name: &'static str) -> Self {
        Self(name)
    }
}

impl AsRef<str> for FontFile {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for FontFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
