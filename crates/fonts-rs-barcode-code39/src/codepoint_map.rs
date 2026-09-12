//! Mapping between Unicode codepoints and Code 39 glyph names.
//!
//! - `GLYPHS`: codepoint -> glyph name
//! - `REVERSE_GLYPHS`: glyph name -> codepoint

include!(concat!(env!("OUT_DIR"), "/codemap.rs"));
