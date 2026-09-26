//! Emoji metadata infrastructure shared by emoji font families.
//!
//! The data sources are Unicode standards — CLDR `annotations.json` for
//! keywords and semantic names, `emoji-test.txt` for group categories —
//! and work for any font covering the emoji codepoint ranges
//! (e.g. Noto Emoji, Blobmoji).

pub mod cldr;
pub mod emoji_test;
pub mod metadata;

pub use cldr::parse_cldr_annotations;
pub use emoji_test::parse_emoji_test;
pub use metadata::EmojiMetadataGenerator;

/// CLDR annotations file name (relative to `resources/`).
pub const ANNOTATIONS_FILE: &str = "metadata/annotations.json";

/// Unicode emoji-test.txt file name (relative to `resources/`).
pub const EMOJI_TEST_FILE: &str = "metadata/emoji-test.txt";
