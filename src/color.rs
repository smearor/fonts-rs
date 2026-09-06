//! Color type for nerd-fonts-gtk.
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

use std::fmt;
use std::str::FromStr;

/// RGBA color with components in the range [0.0, 1.0].
///
/// Used for GTK icon color application and as the canonical color type
/// for the `nerd-fonts-gtk` crate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    /// Red component [0.0, 1.0].
    pub r: f64,
    /// Green component [0.0, 1.0].
    pub g: f64,
    /// Blue component [0.0, 1.0].
    pub b: f64,
    /// Alpha component [0.0, 1.0], defaults to 1.0 (opaque).
    pub a: f64,
}

/// Error returned when parsing an invalid hex color string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorParseError {
    /// The invalid input string.
    pub input: String,
    /// Human-readable reason for the failure.
    pub reason: &'static str,
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid color '{}': {}", self.input, self.reason)
    }
}

impl std::error::Error for ColorParseError {}

impl Color {
    /// Creates a new opaque color from RGB components (alpha = 1.0).
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a new color from RGBA components.
    pub const fn new_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Black.
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);

    /// White.
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);

    /// Convert to `[u8; 4]` for pixel buffer operations.
    pub fn to_u8(self) -> [u8; 4] {
        [
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
            (self.a * 255.0).round() as u8,
        ]
    }

    /// Convert to `gdk::RGBA` (only with `gtk` feature).
    #[cfg(feature = "gtk")]
    pub fn to_gdk_rgba(self) -> gtk4::gdk::RGBA {
        gtk4::gdk::RGBA::new(self.r as f32, self.g as f32, self.b as f32, self.a as f32)
    }

    /// Parses a hex color string into a `Color`.
    ///
    /// Accepts the following formats (with or without leading `#`):
    /// - `#RGB` — shorthand, each digit doubled (e.g. `#F0A` -> `#FF00AA`)
    /// - `#RGBA` — shorthand with alpha
    /// - `#RRGGBB` — opaque
    /// - `#RRGGBBAA` — with alpha
    ///
    /// # Errors
    ///
    /// Returns [`ColorParseError`] if the string is not a valid hex color.
    pub fn from_hex(hex: &str) -> Result<Self, ColorParseError> {
        Self::from_str(hex)
    }

    /// Formats the color as a hex string `#RRGGBBAA`.
    ///
    /// If alpha is 1.0 (opaque), the output is `#RRGGBB` (6 digits).
    /// Otherwise, the full 8-digit form `#RRGGBBAA` is used.
    pub fn to_hex(self) -> String {
        let [r, g, b, a] = self.to_u8();
        if a == 255 {
            format!("#{:02X}{:02X}{:02X}", r, g, b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
        }
    }
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix('#').unwrap_or(s);
        let len = hex.len();

        let (r, g, b, a) = match len {
            3 => {
                let bytes = hex.as_bytes();
                let r = u8::from_str_radix(&format!("{}{}", bytes[0] as char, bytes[0] as char), 16);
                let g = u8::from_str_radix(&format!("{}{}", bytes[1] as char, bytes[1] as char), 16);
                let b = u8::from_str_radix(&format!("{}{}", bytes[2] as char, bytes[2] as char), 16);
                (r, g, b, Ok(255u8))
            }
            4 => {
                let bytes = hex.as_bytes();
                let r = u8::from_str_radix(&format!("{}{}", bytes[0] as char, bytes[0] as char), 16);
                let g = u8::from_str_radix(&format!("{}{}", bytes[1] as char, bytes[1] as char), 16);
                let b = u8::from_str_radix(&format!("{}{}", bytes[2] as char, bytes[2] as char), 16);
                let a = u8::from_str_radix(&format!("{}{}", bytes[3] as char, bytes[3] as char), 16);
                (r, g, b, a)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16);
                let g = u8::from_str_radix(&hex[2..4], 16);
                let b = u8::from_str_radix(&hex[4..6], 16);
                (r, g, b, Ok(255u8))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16);
                let g = u8::from_str_radix(&hex[2..4], 16);
                let b = u8::from_str_radix(&hex[4..6], 16);
                let a = u8::from_str_radix(&hex[6..8], 16);
                (r, g, b, a)
            }
            _ => {
                return Err(ColorParseError {
                    input: s.to_string(),
                    reason: "expected 3, 4, 6, or 8 hex digits",
                });
            }
        };

        match (r, g, b, a) {
            (Ok(r), Ok(g), Ok(b), Ok(a)) => Ok(Self::from([r, g, b, a])),
            _ => Err(ColorParseError {
                input: s.to_string(),
                reason: "contains non-hex characters",
            }),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl From<[u8; 4]> for Color {
    fn from(c: [u8; 4]) -> Self {
        Self {
            r: c[0] as f64 / 255.0,
            g: c[1] as f64 / 255.0,
            b: c[2] as f64 / 255.0,
            a: c[3] as f64 / 255.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_alpha_to_one() {
        let color = Color::new(0.5, 0.25, 0.75);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn new_rgba_preserves_alpha() {
        let color = Color::new_rgba(0.1, 0.2, 0.3, 0.4);
        assert_eq!(color.a, 0.4);
    }

    #[test]
    fn black_constant() {
        assert_eq!(Color::BLACK, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn white_constant() {
        assert_eq!(Color::WHITE, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn to_u8_round_trip() {
        let color = Color::new_rgba(1.0, 0.0, 1.0, 0.0);
        let bytes = color.to_u8();
        assert_eq!(bytes, [255, 0, 255, 0]);
        let restored: Color = bytes.into();
        assert_eq!(restored, color);
    }

    #[test]
    fn to_u8_black() {
        assert_eq!(Color::BLACK.to_u8(), [0, 0, 0, 255]);
    }

    #[test]
    fn to_u8_white() {
        assert_eq!(Color::WHITE.to_u8(), [255, 255, 255, 255]);
    }

    #[test]
    fn from_u8_array() {
        let color: Color = [128, 64, 32, 255].into();
        assert_eq!(color.r, 128.0 / 255.0);
        assert_eq!(color.g, 64.0 / 255.0);
        assert_eq!(color.b, 32.0 / 255.0);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn from_hex_rgb() {
        let color = Color::from_hex("#FF6B4A").unwrap();
        assert_eq!(color, Color::new(1.0, 0x6B as f64 / 255.0, 0x4A as f64 / 255.0));
    }

    #[test]
    fn from_hex_rgba() {
        let color = Color::from_hex("#FF6B4A80").unwrap();
        assert_eq!(color.a, 0x80 as f64 / 255.0);
    }

    #[test]
    fn from_hex_shorthand_rgb() {
        let color = Color::from_hex("#F0A").unwrap();
        assert_eq!(color, Color::new(1.0, 0.0, 0xAA as f64 / 255.0));
    }

    #[test]
    fn from_hex_shorthand_rgba() {
        let color = Color::from_hex("#F0AF").unwrap();
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn from_hex_without_hash() {
        let color = Color::from_hex("FF6B4A").unwrap();
        assert_eq!(color, Color::from_hex("#FF6B4A").unwrap());
    }

    #[test]
    fn from_hex_invalid_length() {
        assert!(Color::from_hex("#FF").is_err());
        assert!(Color::from_hex("#FFGGFF").is_err());
    }

    #[test]
    fn from_hex_invalid_chars() {
        assert!(Color::from_hex("#GGGGGG").is_err());
    }

    #[test]
    fn to_hex_opaque() {
        let color = Color::new(1.0, 0.0, 1.0);
        assert_eq!(color.to_hex(), "#FF00FF");
    }

    #[test]
    fn to_hex_with_alpha() {
        let color = Color::new_rgba(1.0, 0.0, 1.0, 0.5);
        assert_eq!(color.to_hex(), "#FF00FF80");
    }

    #[test]
    fn from_str_round_trip() {
        let original = Color::new_rgba(0x12 as f64 / 255.0, 0x34 as f64 / 255.0, 0x56 as f64 / 255.0, 0x78 as f64 / 255.0);
        let parsed: Color = original.to_hex().parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn display_uses_hex() {
        let color = Color::new(1.0, 0.0, 0.0);
        assert_eq!(format!("{}", color), "#FF0000");
    }
}
