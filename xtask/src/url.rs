use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;

/// A remote URL (artifact source, git remote, provider input).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Url(
    /// The full URL string.
    String,
);

impl Deref for Url {
    type Target = str;

    /// Borrows the URL as `&str` (e.g. for subprocess args).
    fn deref(&self) -> &str {
        &self.0
    }
}

impl Display for Url {
    /// Writes the plain URL, honoring `Formatter` width/alignment.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for Url {
    /// Wraps a URL string.
    fn from(url: &str) -> Self {
        Self(url.to_string())
    }
}

impl From<String> for Url {
    /// Wraps a URL string.
    fn from(url: String) -> Self {
        Self(url)
    }
}
