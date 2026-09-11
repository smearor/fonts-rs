//! Shared data types for Nerd Font icon handling.
//!
//! Provides common types used across the nerd-fonts ecosystem:
//! [`CodePoint`], [`IconName`], [`IconEntry`], [`IconSet`], etc.
//!
//! Generic types ([`CodePoint`], [`ResourcePath`], [`GlyphEntry`]) are
//! re-exported from `fonts-rs-model`. Nerd Font specific types
//! ([`IconName`], [`IconSet`], [`IconEntry`]) remain in this crate.

pub mod category;
pub mod codepoint;
pub mod entry;
pub mod keyword;
pub mod name;
pub mod paths;
pub mod set;

// Re-export generic types from the framework.
pub use fonts_rs_model::CodePoint;
pub use fonts_rs_model::CodePointParseError;
pub use fonts_rs_model::GlyphEntry;
pub use fonts_rs_model::ResourcePath;

pub use category::IconCategory;
pub use keyword::IconKeyword;
pub use name::IconName;
pub use paths::GRESOURCE_PREFIX;
pub use paths::ICONS_RESOURCE_PATH;
pub use set::IconSet;

/// Nerd Font icon metadata entry.
///
/// This is a specialization of the generic `GlyphEntry` with `IconName`
/// as the name type.
pub type IconEntry = GlyphEntry<IconName>;
