//! Shared Nerd Font integration for GTK4 projects.
//!
//! Provides icon name resolution, font loading, and CSS generation
//! for Nerd Font symbols.

pub mod css;
pub mod icons;
pub mod init;

#[cfg(feature = "gtk")]
pub mod gtk;

#[cfg(feature = "metadata")]
pub mod metadata;

#[cfg(feature = "render")]
pub mod fonts;

#[cfg(feature = "web")]
pub mod web;

// Re-export model types
pub use nerd_fonts_model::CodePoint;
pub use nerd_fonts_model::CodePointParseError;
pub use nerd_fonts_model::IconCategory;
pub use nerd_fonts_model::IconEntry;
pub use nerd_fonts_model::IconKeyword;
pub use nerd_fonts_model::IconName;
pub use nerd_fonts_model::IconSet;
pub use nerd_fonts_model::ResourcePath;

// Re-export icon functions
pub use icons::all_icons;
pub use icons::all_icons_typed;

// Re-export init
pub use init::InitError;
pub use init::InitOptions;
pub use init::init;

// Re-export CSS
pub use css::font_face_css;
pub use css::GtkVersion;
