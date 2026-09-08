//! Type-safe mapping types for icon metadata.
//!
//! Provides newtypes for raw keyword, category, and alias values,
//! and wrapper types for the mappings from icon names to these values.

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod alias;
pub mod alias_mapping;
pub mod category;
pub mod category_mapping;
pub mod keyword;
pub mod keyword_mapping;
pub mod metadata_mapping;

pub use alias::RawAlias;
pub use alias_mapping::AliasMapping;
pub use category::RawCategory;
pub use category_mapping::CategoryMapping;
pub use keyword::RawKeyword;
pub use keyword_mapping::KeywordMapping;
pub use metadata_mapping::MetadataMapping;
