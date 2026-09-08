//! Normalized GTK icon name newtype for type-safe icon name handling.

use std::fmt;

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

    /// Parses an icon name from any common form, applying normalization.
    ///
    /// Accepts short names (e.g. `"fa-gamepad"`), names with `nf-` prefix
    /// (e.g. `"nf-fa-gamepad"`), and full names with `-symbolic` suffix
    /// (e.g. `"nf-fa-gamepad-symbolic"`). All forms normalize to the
    /// canonical `nf-{prefix}-{name}-symbolic` representation.
    ///
    /// Returns `None` if the name does not match a known [`IconSet`] prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use nerd_fonts_model::IconName;
    ///
    /// let a = IconName::parse("fa-gamepad").unwrap();
    /// let b = IconName::parse("nf-fa-gamepad").unwrap();
    /// let c = IconName::parse("nf-fa-gamepad-symbolic").unwrap();
    /// assert_eq!(a, b);
    /// assert_eq!(b, c);
    /// assert_eq!(a.as_ref(), "nf-fa-gamepad-symbolic");
    /// ```
    pub fn parse(name: &str) -> Option<Self> {
        let stripped = name.strip_suffix("-symbolic").unwrap_or(name);
        let stripped = stripped.strip_prefix("nf-").unwrap_or(stripped);
        Self::from_glyph_name(stripped)
    }

    /// Returns the CSS class name (without the `-symbolic` suffix).
    ///
    /// # Examples
    ///
    /// ```
    /// use nerd_fonts_model::IconName;
    ///
    /// let name = IconName::parse("nf-fa-gamepad-symbolic").unwrap();
    /// assert_eq!(name.css_class_name(), "nf-fa-gamepad");
    /// ```
    #[allow(dead_code)]
    pub fn css_class_name(&self) -> &str {
        self.0.strip_suffix("-symbolic").unwrap_or(&self.0)
    }

    /// Returns the [`IconSet`] this icon belongs to.
    ///
    /// Determined by the icon name prefix (e.g. `nf-fa-*` -> `FontAwesome`).
    ///
    /// # Examples
    ///
    /// ```
    /// use nerd_fonts_model::IconName;
    /// use nerd_fonts_model::IconSet;
    ///
    /// let name = IconName::from_glyph_name("fa-gamepad").unwrap();
    /// assert_eq!(name.icon_set(), IconSet::FontAwesome);
    /// ```
    #[allow(dead_code)]
    pub fn icon_set(&self) -> IconSet {
        IconSet::detect(&self.0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_name() {
        let name = IconName::parse("nf-fa-gamepad-symbolic").unwrap();
        assert_eq!(name.as_ref(), "nf-fa-gamepad-symbolic");
    }

    #[test]
    fn parse_short_name() {
        let name = IconName::parse("fa-gamepad").unwrap();
        assert_eq!(name.as_ref(), "nf-fa-gamepad-symbolic");
    }

    #[test]
    fn parse_with_nf_prefix_no_symbolic() {
        let name = IconName::parse("nf-fa-gamepad").unwrap();
        assert_eq!(name.as_ref(), "nf-fa-gamepad-symbolic");
    }

    #[test]
    fn parse_all_forms_equivalent() {
        let a = IconName::parse("fa-gamepad").unwrap();
        let b = IconName::parse("nf-fa-gamepad").unwrap();
        let c = IconName::parse("nf-fa-gamepad-symbolic").unwrap();
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    #[test]
    fn parse_unknown_prefix_returns_none() {
        assert!(IconName::parse("nf-xx-nonexistent").is_none());
    }

    #[test]
    fn parse_empty_returns_none() {
        assert!(IconName::parse("").is_none());
    }
}
