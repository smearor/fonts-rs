//! Noto Emoji multi-codepoint SVG artwork.
//!
//! Build-resource crate carrying the vendored `2D/svg` multi-codepoint
//! sequences: ZWJ compositions, regional-indicator flags and skin-tone
//! modifier sequences. Dependent crates locate the directory through
//! the `DEP_NOTO_EMOJI_SEQUENCES_*` variables emitted by `build.rs`.
