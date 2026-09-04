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
