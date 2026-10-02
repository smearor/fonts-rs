//! GTK font registration trait.
//!
//! Provides the [`GtkFont`] trait for font families with GResource-backed
//! registration. Each font crate implements this trait on its marker type
//! via the `impl_gtk_font!` macro in `fonts-rs-generator`.
//!
//! Submodules:
//!
//! - [`gtk_font`]: the [`GtkFont`] trait plus the provided
//!   `load_font_from_gresource` helper method.
//! - [`load_font_error`], [`register_font_error`],
//!   [`register_glyph_error`]: the trait's error types.
//! - [`load_css`]: free [`load_css`] helper shared by every family's
//!   CSS init path.

mod gtk_font;
mod load_css;
mod load_font_error;
mod register_font_error;
mod register_glyph_error;

pub use gtk_font::GtkFont;
pub use load_css::load_css;
pub use load_font_error::LoadFontError;
pub use register_font_error::RegisterFontError;
pub use register_glyph_error::RegisterGlyphError;
