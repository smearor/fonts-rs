//! Build-time generated variant information.
//!
//! Noto Emoji is a static font with no variants. This module exposes the
//! generated constants for the GResource prefix and glyph name prefix.

include!(concat!(env!("OUT_DIR"), "/variant.rs"));
