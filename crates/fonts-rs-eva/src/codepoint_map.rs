//! Mapping between Unicode codepoints and Eva icon names.
//!
//! - `GLYPHS`: codepoint -> icon name
//! - `REVERSE_GLYPHS`: icon name -> codepoint

include!(concat!(env!("OUT_DIR"), "/codemap.rs"));
