//! Build-time code generation and glyph export for Noto Emoji.
//!
//! Provides [`NotoEmojiDefinition`] implementing the generic
//! [`FontDefinition`] trait plus the family constants. Generic emoji
//! metadata infrastructure (CLDR parsing, emoji-test parsing, the
//! metadata generator) lives in [`fonts_rs_generator::emoji`] and is
//! re-exported here for convenience.

pub mod definition;

pub use definition::EMOJI_RANGES;
pub use definition::FONT_FILE;
pub use definition::NotoEmoji;
pub use definition::NotoEmojiDefinition;
pub use fonts_rs_generator::emoji::ANNOTATIONS_FILE;
pub use fonts_rs_generator::emoji::EMOJI_TEST_FILE;
pub use fonts_rs_generator::emoji::EmojiMetadataGenerator;
pub use fonts_rs_generator::emoji::parse_cldr_annotations;
pub use fonts_rs_generator::emoji::parse_emoji_test;

// Re-export generic types from the framework for backward compatibility.
pub use fonts_rs_generator::EMPTY_SVG;
pub use fonts_rs_generator::Font;
pub use fonts_rs_generator::FontBuild;
pub use fonts_rs_generator::FontDefinition;
pub use fonts_rs_generator::GlyphGenerator;
pub use fonts_rs_generator::MetadataGenerator;
pub use fonts_rs_generator::Svg;
pub use fonts_rs_generator::SvgPathBuilder;
pub use fonts_rs_generator::normalize_to_kebab;
