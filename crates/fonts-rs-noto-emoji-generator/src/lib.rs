//! Build-time code generation and glyph export for Noto Emoji.
//!
//! Provides:
//! - [`NotoEmojiDefinition`] implementing the generic [`FontDefinition`] trait
//! - CLDR annotation parsing for emoji keywords and semantic names
//! - Unicode emoji-test.txt parsing for emoji categories
//! - [`NotoEmojiMetadataGenerator`] for generating phf::Map metadata tables

pub mod cldr;
pub mod definition;
pub mod emoji_test;
pub mod generator;

pub use cldr::parse_cldr_annotations;
pub use definition::ANNOTATIONS_FILE;
pub use definition::EMOJI_RANGES;
pub use definition::EMOJI_TEST_FILE;
pub use definition::FONT_FILE;
pub use definition::GLYPH_PREFIX;
pub use definition::GRESOURCE_PREFIX;
pub use definition::NotoEmoji;
pub use definition::NotoEmojiDefinition;
pub use emoji_test::parse_emoji_test;
pub use generator::metadata::NotoEmojiMetadataGenerator;

// Re-export generic types from the framework for backward compatibility.
pub use fonts_rs_generator::EMPTY_SVG;
pub use fonts_rs_generator::Font;
pub use fonts_rs_generator::FontBuild;
pub use fonts_rs_generator::FontDefinition;
pub use fonts_rs_generator::GlyphGenerator;
pub use fonts_rs_generator::MetadataGenerator;
pub use fonts_rs_generator::SvgPathBuilder;
pub use fonts_rs_generator::normalize_to_kebab;
