//! Phantom-typed GTK icon name newtype for icon font families.
//!
//! [`IconName<F>`] is the icon-font counterpart to
//! [`GlyphName`](crate::GlyphName): while `GlyphName` wraps a glyph name
//! for codepoint maps, `IconName` wraps the normalized GTK icon name
//! (`{prefix}-{name}-symbolic`) used by `GtkIconTheme` and CSS classes.

use crate::FontFamilyConfig;
use crate::IconSetDetect;
use crate::ResourcePath;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

/// A normalized GTK icon name for a specific icon font family.
///
/// Follows the naming convention `{F::ICON_PREFIX}{name}-symbolic`
/// (kebab-case, lowercase). The phantom type parameter `F` ties the name
/// to its family: `IconName<NerdFontsConfig>` is a distinct type from
/// `IconName<TablerConfig>` at compile time.
///
/// Constructed from a raw glyph name via [`IconName::from_glyph_name`],
/// which applies normalization:
///
/// 1. Lowercase
/// 2. Replace `_` with `-`
/// 3. Replace non-alphanumeric characters (except `-`) with `-`
/// 4. Collapse consecutive `-`
/// 5. Strip leading/trailing `-`
/// 6. Prefix with `F::ICON_PREFIX` and suffix with `-symbolic`
///
/// The result is validated via
/// [`FontFamilyConfig::is_valid_icon_name`] - single-set families accept
/// any prefixed name; multi-set families (Nerd Fonts) check set
/// membership.
///
/// Serializes as a string in JSON.
///
/// The standard traits are implemented manually so that `F` needs no
/// `Clone`/`Ord`/etc. bounds - the phantom marker never affects
/// equality, ordering, or hashing.
pub struct IconName<F: FontFamilyConfig> {
    /// The normalized icon name (`{prefix}{name}-symbolic`).
    name: String,
    /// Phantom marker for the font family config type.
    _marker: PhantomData<F>,
}

impl<F: FontFamilyConfig> fmt::Debug for IconName<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IconName").field(&self.name).finish()
    }
}

impl<F: FontFamilyConfig> Clone for IconName<F> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            _marker: PhantomData,
        }
    }
}

impl<F: FontFamilyConfig> PartialEq for IconName<F> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<F: FontFamilyConfig> Eq for IconName<F> {}

impl<F: FontFamilyConfig> std::hash::Hash for IconName<F> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<F: FontFamilyConfig> PartialOrd for IconName<F> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<F: FontFamilyConfig> Ord for IconName<F> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl<F: FontFamilyConfig> IconName<F> {
    /// Creates an `IconName` from a raw glyph name, applying normalization.
    ///
    /// Returns `None` if the normalized name would be empty or fails the
    /// family's [`FontFamilyConfig::is_valid_icon_name`] check.
    pub fn from_glyph_name(name: &str) -> Option<Self> {
        let name = name.to_lowercase();
        let name = name.replace('_', "-");
        let name: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).collect();
        let name: String = name.split('-').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("-");
        if name.is_empty() {
            return None;
        }
        let full_name = format!("{}{}-symbolic", F::ICON_PREFIX, name);
        if !F::is_valid_icon_name(&full_name) {
            return None;
        }
        Some(Self {
            name: full_name,
            _marker: PhantomData,
        })
    }

    /// Parses an icon name from any common form, applying normalization.
    ///
    /// Accepts short names (e.g. `"fa-gamepad"`), names with the family
    /// prefix (e.g. `"nf-fa-gamepad"`), and full names with `-symbolic`
    /// suffix (e.g. `"nf-fa-gamepad-symbolic"`). All forms normalize to
    /// the canonical `{prefix}{name}-symbolic` representation.
    ///
    /// Returns `None` if the name fails
    /// [`FontFamilyConfig::is_valid_icon_name`].
    pub fn parse(name: &str) -> Option<Self> {
        let stripped = name.strip_suffix("-symbolic").unwrap_or(name);
        let stripped = stripped.strip_prefix(F::ICON_PREFIX).unwrap_or(stripped);
        Self::from_glyph_name(stripped)
    }

    /// Returns the CSS class name (without the `-symbolic` suffix).
    pub fn css_class_name(&self) -> &str {
        self.name.strip_suffix("-symbolic").unwrap_or(&self.name)
    }
}

impl<F: IconSetDetect> IconName<F> {
    /// Returns the icon set this icon belongs to.
    ///
    /// Only available for families implementing [`IconSetDetect`]
    /// (multi-set icon fonts like Nerd Fonts).
    pub fn icon_set(&self) -> F::Set {
        F::detect_set(&self.name)
    }
}

impl<F: FontFamilyConfig> AsRef<str> for IconName<F> {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl<F: FontFamilyConfig> fmt::Display for IconName<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl<F: FontFamilyConfig> Serialize for IconName<F> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

impl<'de, F: FontFamilyConfig> Deserialize<'de> for IconName<F> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if !F::is_valid_icon_name(&s) {
            return Err(serde::de::Error::custom(format!("invalid icon name for this family: '{s}'")));
        }
        Ok(Self { name: s, _marker: PhantomData })
    }
}

impl<F: FontFamilyConfig> From<&IconName<F>> for ResourcePath {
    fn from(name: &IconName<F>) -> Self {
        ResourcePath::from_name(&F::icons_resource_prefix(), name.as_ref())
    }
}

/// Error returned when parsing an [`IconName`] from a string fails.
///
/// The input does not normalize to a name accepted by the family's
/// [`FontFamilyConfig::is_valid_icon_name`] check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconNameParseError {
    /// The input that failed to parse.
    input: String,
}

impl fmt::Display for IconNameParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid icon name: '{}'", self.input)
    }
}

impl std::error::Error for IconNameParseError {}

impl<F: FontFamilyConfig> FromStr for IconName<F> {
    type Err = IconNameParseError;

    /// Parses an icon name from any common form - see [`IconName::parse`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or_else(|| IconNameParseError { input: s.to_string() })
    }
}
