//! Unicode codepoint newtype for type-safe codepoint handling.

use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// A Unicode codepoint wrapped in a newtype for type safety.
///
/// Serializes as an uppercase hex string (e.g. `"F11B"`) in JSON.
/// Can be constructed from a `char` and parsed back from a hex string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodePoint(char);

/// Error returned when parsing a `CodePoint` from a string.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CodePointParseError {
    /// The string is not valid hexadecimal.
    #[error("invalid hex codepoint: {0}")]
    InvalidHex(#[from] ParseIntError),
    /// The hex value is not a valid Unicode scalar value.
    #[error("invalid Unicode scalar value")]
    InvalidScalar,
    /// The icon name was not found in the codepoint map.
    #[error("icon name not found in codepoint map")]
    #[allow(dead_code)]
    IconNotFound,
}

impl CodePoint {
    /// Returns the underlying `char`.
    pub fn as_char(&self) -> char {
        self.0
    }
}

impl From<char> for CodePoint {
    fn from(ch: char) -> Self {
        Self(ch)
    }
}

impl fmt::Display for CodePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04X}", self.0 as u32)
    }
}

impl FromStr for CodePoint {
    type Err = CodePointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let codepoint = u32::from_str_radix(s, 16).map_err(CodePointParseError::InvalidHex)?;
        char::from_u32(codepoint).map(Self).ok_or(CodePointParseError::InvalidScalar)
    }
}

impl Serialize for CodePoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for CodePoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        CodePoint::from_str(&s).map_err(serde::de::Error::custom)
    }
}
