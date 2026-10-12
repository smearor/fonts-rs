//! Noto Emoji single-codepoint SVG artwork and CLDR metadata.
//!
//! Build-resource crate carrying the vendored `2D/svg` single-codepoint
//! artwork plus the CLDR/UTS-#51 metadata files. Dependent crates locate
//! the directories through the `DEP_NOTO_EMOJI_GLYPHS_*` variables
//! emitted by `build.rs`; see the crate README or `upstream.toml` for
//! provenance.
