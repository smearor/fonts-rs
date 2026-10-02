//! Mapping between Unicode codepoints and Line Awesome glyph names.
//!
//! - `GLYPHS`: codepoint -> glyph name
//! - `REVERSE_GLYPHS`: glyph name -> codepoint
//!
//! Generated at build time from the vendored stem-keyed
//! `codepoints.json` map. Regular/solid style pairs share one
//! codepoint (Font Awesome 5 model) - the forward map resolves a shared
//! codepoint to the canonical (regular-style) name while
//! `REVERSE_GLYPHS` keeps every exported name resolvable.

include!(concat!(env!("OUT_DIR"), "/codemap.rs"));
