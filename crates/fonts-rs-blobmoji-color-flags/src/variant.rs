//! Build-time generated variant information.
//!
//! Blobmoji ships four TTF variants (color x flags matrix). This module
//! exposes the generated constants for the GResource prefix and glyph
//! name prefix of the variant selected at build time.

include!(concat!(env!("OUT_DIR"), "/variant.rs"));
