//! Build-time generated variant information.
//!
//! The active Doto variant (weight + roundness) is selected via Cargo
//! features at build time. This module exposes the generated constants
//! for the active variant.

include!(concat!(env!("OUT_DIR"), "/variant.rs"));
