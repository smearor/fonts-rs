//! Blobmoji multi-codepoint SVG artwork.
//!
//! Build-resource crate carrying the vendored `svg` multi-codepoint
//! sequences: ZWJ compositions, regional-indicator flags and skin-tone
//! modifier sequences. Dependent crates locate the directory through
//! the `DEP_BLOBMOJI_SEQUENCES_*` variables emitted by `build.rs`; see
//! the crate README or `upstream.toml` for provenance. The shared
//! CLDR/UTS-#51 emoji metadata lives in `fonts-rs-emoji-metadata`.
