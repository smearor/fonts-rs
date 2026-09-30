//! Mapping between Unicode codepoints and Fluent glyph names.
//!
//! - `GLYPHS`: codepoint -> glyph name
//! - `REVERSE_GLYPHS`: glyph name -> codepoint
//!
//! Generated at build time from the vendored upstream
//! `FluentSystemIcons-{Style}.json` codepoint map.

include!(concat!(env!("OUT_DIR"), "/codemap.rs"));
