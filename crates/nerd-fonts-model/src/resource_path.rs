//! GResource path newtype for type-safe resource path handling.

use std::fmt;

use super::name::IconName;
use super::paths::ICONS_RESOURCE_PATH;

/// A GResource path for a Nerd Font icon.
///
/// Constructed from an icon name, producing a path like
/// `/io/smearor/nerd_fonts/icons/nf-fa-gamepad-symbolic`.
/// Serializes as a string in JSON.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourcePath(String);

impl From<&IconName> for ResourcePath {
    fn from(icon_name: &IconName) -> Self {
        Self(format!("{}/{}", ICONS_RESOURCE_PATH, icon_name))
    }
}

impl fmt::Display for ResourcePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl serde::Serialize for ResourcePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for ResourcePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s))
    }
}
