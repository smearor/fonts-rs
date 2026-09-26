//! Build-time generated variant information.
//!
//! Noto Emoji ships five TTF variants (mono + color/bitmap x flags).
//! This module exposes the generated constants for the GResource prefix
//! and glyph name prefix of the variant selected at build time.

include!(concat!(env!("OUT_DIR"), "/variant.rs"));
