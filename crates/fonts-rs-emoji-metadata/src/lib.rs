//! Shared CLDR/UTS-#51 emoji metadata for emoji font crates.
//!
//! Build-resource crate carrying the vendored emoji metadata files
//! shared by every emoji font family (Noto Emoji, Blobmoji, ...):
//!
//! - `emoji-test.txt` - Unicode 17.0 keyboard/display test data (UTS #51)
//! - `annotations.json` - CLDR 48 English names + keywords
//! - `annotations-derived.json` - CLDR 48 derived annotations
//!   (multi-codepoint sequences: ZWJ, flags, keycaps)
//! - `emoji_17_0_ordering.json` - emoji picker ordering/shortcodes
//!   (googlefonts/emoji-metadata)
//!
//! Dependent crates locate the directory through the
//! `DEP_EMOJI_METADATA_DIR` variable emitted by `build.rs`; see
//! `upstream.toml` for provenance.
