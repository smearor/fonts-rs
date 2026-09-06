//! Normalized GTK icon name newtype for type-safe icon name handling.

use std::convert::TryFrom;
use std::fmt;

#[cfg(is_lib)]
use super::codepoint::CodePoint;
#[cfg(is_lib)]
use super::codepoint::CodePointParseError;
use super::set::IconSet;

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
    /// Returns `None` if the normalized name would be empty or does not
    /// match a known [`IconSet`] prefix (e.g. `nf-fa-`, `nf-md-`).
    pub fn from_glyph_name(name: &str) -> Option<Self> {
        let name = name.to_lowercase();
        let name = name.replace('_', "-");
        let name: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).collect();
        let name: String = name.split('-').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("-");
        if name.is_empty() {
            return None;
        }
        let full_name = format!("nf-{}-symbolic", name);
        // Validate that the normalized name matches a known icon set prefix.
        if IconSet::detect(&full_name) == IconSet::Other {
            return None;
        }
        Some(Self(full_name))
    }

    /// Returns the [`IconSet`] this icon belongs to.
    ///
    /// Determined by the icon name prefix (e.g. `nf-fa-*` -> `FontAwesome`).
    ///
    /// # Examples
    ///
    /// ```
    /// use nerd_fonts_gtk::icons::IconName;
    /// use nerd_fonts_gtk::icons::IconSet;
    ///
    /// let name = IconName::from_glyph_name("fa-gamepad").unwrap();
    /// assert_eq!(name.icon_set(), IconSet::FontAwesome);
    /// ```
    pub fn icon_set(&self) -> IconSet {
        IconSet::detect(&self.0)
    }

    /// Resolves this icon name to its Unicode [`CodePoint`].
    ///
    /// Returns `None` if the icon name is not found in the codepoint map.
    ///
    /// # Examples
    ///
    /// ```
    /// use nerd_fonts_gtk::icons::IconName;
    ///
    /// let name = IconName::from_glyph_name("fa-gamepad").unwrap();
    /// let codepoint = name.codepoint();
    /// assert!(codepoint.is_some());
    /// assert_eq!(codepoint.unwrap().as_char(), '\u{F11B}');
    /// ```
    #[cfg(is_lib)]
    pub fn codepoint(&self) -> Option<CodePoint> {
        super::codepoint_map::REVERSE_ICONS
            .get(self.0.as_str())
            .copied()
            .map(CodePoint)
    }
}

/// Convert an [`IconName`] to a [`CodePoint`] via the codepoint map.
#[cfg(is_lib)]
impl TryFrom<&IconName> for CodePoint {
    type Error = CodePointParseError;

    fn try_from(icon_name: &IconName) -> Result<Self, Self::Error> {
        icon_name.codepoint().ok_or(CodePointParseError::IconNotFound)
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
        // Validate that the string matches a known icon set prefix.
        if IconSet::detect(&s) == IconSet::Other {
            return Err(serde::de::Error::custom(format!("unknown icon set prefix for icon name: '{}'", s)));
        }
        Ok(Self(s))
    }
}
