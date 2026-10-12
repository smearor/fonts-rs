//! Noto Emoji single-codepoint SVG artwork.
//!
//! Build-resource crate carrying the vendored `2D/svg` single-codepoint
//! artwork. Dependent crates locate the directory through the
//! `DEP_NOTO_EMOJI_GLYPHS_*` variables emitted by `build.rs`; see the
//! crate README or `upstream.toml` for provenance. The shared
//! CLDR/UTS-#51 emoji metadata lives in `fonts-rs-emoji-metadata`.
