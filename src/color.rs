//! Color type for nerd-fonts-gtk.
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

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
}
