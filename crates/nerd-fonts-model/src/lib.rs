//! Shared data types for Nerd Font icon handling.
//!
//! Provides common types used across the nerd-fonts ecosystem:
//! [`CodePoint`], [`IconName`], [`IconEntry`], [`IconSet`], etc.

pub mod category;
pub mod codepoint;
pub mod entry;
pub mod keyword;
pub mod name;
pub mod paths;
pub mod resource_path;
pub mod set;

pub use category::IconCategory;
pub use codepoint::CodePoint;
pub use codepoint::CodePointParseError;
pub use entry::IconEntry;
pub use keyword::IconKeyword;
pub use name::IconName;
pub use paths::GRESOURCE_PREFIX;
pub use paths::ICONS_RESOURCE_PATH;
pub use resource_path::ResourcePath;
pub use set::IconSet;
