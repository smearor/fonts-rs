//! Generic model types for font glyph packaging.
//!
//! Provides font-family-agnostic types shared across all `fonts-rs-*` crates:
//! [`CodePoint`], [`ResourcePath`], [`FontFamily`] marker trait,
//! [`GlyphName`] phantom-typed newtype, and [`GlyphEntry`] generic metadata.
//!
//! These types are extracted from `nerd-fonts-model` and generalized so
//! that any font family crate can use them without Nerd Font dependencies.

pub mod axis;
pub mod axis_value;
pub mod category;
pub mod codepoint;
pub mod codepoint_category_map;
pub mod codepoint_keyword_map;
pub mod codepoint_name_map;
pub mod codepoint_range;
pub mod font_family;
pub mod font_file;
pub mod font_variant;
pub mod font_variant_type;
pub mod glyph_entry;
pub mod glyph_name;
pub mod glyph_name_map;
pub mod keyword;
pub mod metadata;
pub mod paths;
pub mod resource_path;

pub use axis::Axis;
pub use axis_value::AxisValue;
pub use axis_value::AxisValues;
pub use category::GlyphCategory;
pub use codepoint::CodePoint;
pub use codepoint::CodePointParseError;
pub use codepoint_category_map::CodePointCategoryMap;
pub use codepoint_keyword_map::CodePointKeywordMap;
pub use codepoint_name_map::CodePointNameMap;
pub use codepoint_range::ASCII_PRINTABLE_RANGE;
pub use codepoint_range::BMP_RANGE;
pub use codepoint_range::CodePointRange;
pub use codepoint_range::PUA_RANGE;
pub use codepoint_range::SUPPLEMENTARY_PUA_RANGE;
pub use font_family::FontFamily;
pub use font_family::sealed;
pub use font_file::FontFile;
pub use font_variant::FontVariant;
pub use font_variant_type::FontVariantType;
pub use glyph_entry::GlyphEntry;
pub use glyph_name::GlyphName;
pub use glyph_name_map::GlyphNameMap;
pub use keyword::GlyphKeyword;
pub use metadata::GlyphAliasMap;
pub use metadata::GlyphCategoryMap;
pub use metadata::GlyphKeywordMap;
pub use metadata::GlyphMetadata;
pub use paths::GRESOURCE_BASE_PREFIX;
pub use resource_path::ResourcePath;
