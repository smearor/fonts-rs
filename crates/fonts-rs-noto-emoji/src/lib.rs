//! Noto Emoji font integration for GTK 4 and pixel-drawing.
//!
//! Umbrella crate re-exporting one of the five Noto Emoji variant
//! crates, selected by Cargo feature: `mono` (NotoEmoji-Regular) or
//! color (COLRv1) / bitmap (CBDT) x flags - `color-flags` (default),
//! `color-noflags`, `bitmap-flags`, `bitmap-noflags`.
//!
//! Select a non-default variant with `default-features = false`:
//!
//! ```toml
//! fonts-rs-noto-emoji = { version = "0.5", default-features = false, features = ["gtk", "bitmap-flags"] }
//! ```
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_noto_emoji::NotoEmoji;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any glyphs
//! if let Err(e) = NotoEmoji::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```

// Variant features select a TTF - enabling more than one is ambiguous
// (features are additive, so the guard must live in code).
#[cfg(not(any(
    feature = "mono",
    feature = "color-flags",
    feature = "color-noflags",
    feature = "bitmap-flags",
    feature = "bitmap-noflags"
)))]
compile_error!(
    "fonts-rs-noto-emoji: no variant selected - enable exactly one of `mono`, `color-flags` (default), `color-noflags`, `bitmap-flags`, `bitmap-noflags`"
);

#[cfg(all(
    feature = "mono",
    any(feature = "color-flags", feature = "color-noflags", feature = "bitmap-flags", feature = "bitmap-noflags")
))]
compile_error!("fonts-rs-noto-emoji: `mono` conflicts with the other variant features - select exactly one variant");

#[cfg(all(feature = "color-flags", any(feature = "color-noflags", feature = "bitmap-flags", feature = "bitmap-noflags")))]
compile_error!("fonts-rs-noto-emoji: `color-flags` conflicts with the other variant features - select exactly one variant (disable default features first)");

#[cfg(all(feature = "color-noflags", any(feature = "bitmap-flags", feature = "bitmap-noflags")))]
compile_error!("fonts-rs-noto-emoji: `color-noflags` conflicts with the other variant features - select exactly one variant");

#[cfg(all(feature = "bitmap-flags", feature = "bitmap-noflags"))]
compile_error!("fonts-rs-noto-emoji: `bitmap-flags` and `bitmap-noflags` are mutually exclusive - select exactly one variant");

#[cfg(feature = "mono")]
pub use fonts_rs_noto_emoji_mono::*;

#[cfg(feature = "color-flags")]
pub use fonts_rs_noto_emoji_color_flags::*;

#[cfg(feature = "color-noflags")]
pub use fonts_rs_noto_emoji_color_noflags::*;

#[cfg(feature = "bitmap-flags")]
pub use fonts_rs_noto_emoji_bitmap_flags::*;

#[cfg(feature = "bitmap-noflags")]
pub use fonts_rs_noto_emoji_bitmap_noflags::*;
