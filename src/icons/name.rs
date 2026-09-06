//! Normalized GTK icon name newtype for type-safe icon name handling.

use std::fmt;

/// A normalized GTK icon name derived from a Nerd Font glyph name.
///
/// Follows the naming convention `nf-{prefix}-{name}-symbolic` (kebab-case,
/// lowercase). Constructed from a raw glyph name via [`IconName::from_glyph_name`],
/// which applies normalization:
///
/// 1. Lowercase
/// 2. Replace `_` with `-`
/// 3. Replace non-alphanumeric characters (except `-`) with `-`
/// 4. Collapse consecutive `-`
/// 5. Strip leading/trailing `-`
/// 6. Prefix with `nf-` and suffix with `-symbolic`
///
/// Serializes as a string in JSON.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IconName(String);

impl IconName {
    /// Creates an `IconName` from a raw glyph name, applying normalization.
    ///
    /// Returns `None` if the normalized name would be empty (e.g. when the
    /// input is empty or consists only of non-alphanumeric characters).
    pub fn from_glyph_name(name: &str) -> Option<Self> {
        let name = name.to_lowercase();
        let name = name.replace('_', "-");
        let name: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).collect();
        let name: String = name.split('-').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("-");
        if name.is_empty() {
            return None;
        }
        Some(Self(format!("nf-{}-symbolic", name)))
    }
}

impl AsRef<str> for IconName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for IconName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl serde::Serialize for IconName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for IconName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s))
    }
}
