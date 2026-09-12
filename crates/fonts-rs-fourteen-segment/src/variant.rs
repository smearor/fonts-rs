//! Build-time generated variant information.
//!
//! The active DSEG14 variant (style + weight) is selected via Cargo features
//! at build time. This module exposes the generated constants for the
//! active variant.

include!(concat!(env!("OUT_DIR"), "/variant.rs"));
