//! Unicode codepoint newtype — re-exported from `fonts-rs-model`.
//!
//! The `CodePoint` type and its implementations have been moved to the
//! generic `fonts-rs-model` crate. This module re-exports them for
//! backward compatibility.

pub use fonts_rs_model::CodePoint;
pub use fonts_rs_model::CodePointParseError;
