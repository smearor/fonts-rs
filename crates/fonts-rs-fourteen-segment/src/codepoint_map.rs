//! Mapping between Unicode codepoints and DSEG14 glyph names.
//!
//! - `GLYPHS`: codepoint -> glyph name
//! - `REVERSE_GLYPHS`: glyph name -> codepoint

include!(concat!(env!("OUT_DIR"), "/codemap.rs"));
