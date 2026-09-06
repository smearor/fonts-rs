//! Mapping between Unicode codepoints and icon names.
//!
//! - `ICONS`: codepoint -> icon name
//! - `REVERSE_ICONS`: icon name -> codepoint

include!(concat!(env!("OUT_DIR"), "/codemap.rs"));
