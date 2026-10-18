//! Generic model types for font glyph packaging.
//!
//! Provides font-family-agnostic types shared across all `fonts-rs-*` crates:
//! [`CodePoint`], [`ResourcePath`], [`FontFamilyMarker`] marker trait,
//! [`GlyphName`] phantom-typed newtype, and [`GlyphEntry`] generic metadata.
//!
//! The high-level runtime API is the [`FontFamily`] core trait plus the
//! capability traits [`IconThemeFont`], [`PangoFont`], [`HtmlExport`], and
//! [`SoftwareRender`] - see [`prelude`] for a one-line import.
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
#[cfg(feature = "gtk")]
pub mod font_error;
pub mod font_family;
pub mod font_family_config;
pub mod font_family_marker;
pub mod font_file;
pub mod font_variant;
pub mod font_variant_type;
pub mod glyph_entry;
pub mod glyph_name;
pub mod glyph_name_map;
#[cfg(feature = "gtk")]
pub mod gtk;
#[cfg(feature = "gtk")]
pub mod html_builder;
#[cfg(feature = "gtk")]
pub mod html_export;
#[cfg(feature = "gtk")]
pub mod html_svg_export;
pub mod icon_css;
pub mod icon_name;
pub mod icon_set;
pub mod icon_set_detect;
#[cfg(feature = "gtk")]
pub mod icon_theme_font;
pub mod keyword;
pub mod layout;
pub mod metadata;
#[cfg(feature = "gtk")]
pub mod pango_font;
pub mod paths;
pub mod prelude;
pub mod resource_path;
#[cfg(feature = "render")]
pub mod software_render;
pub mod upstream_icon;
pub mod upstream_icons;

#[cfg(feature = "gtk")]
pub use gtk4;
#[cfg(feature = "gtk")]
pub use pango;

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
pub use codepoint_range::PUA_B_RANGE;
pub use codepoint_range::PUA_RANGE;
pub use codepoint_range::SUPPLEMENTARY_PUA_RANGE;
#[cfg(feature = "gtk")]
pub use font_error::FontError;
pub use font_family::FontFamily;
pub use font_family_config::FontFamilyConfig;
pub use font_family_marker::FontFamilyMarker;
pub use font_family_marker::sealed;
pub use font_file::FontFile;
pub use font_variant::FontVariant;
pub use font_variant_type::FontVariantType;
pub use glyph_entry::GlyphEntry;
pub use glyph_name::GlyphName;
pub use glyph_name_map::GlyphNameMap;
#[cfg(feature = "gtk")]
pub use gtk::GtkFont;
#[cfg(feature = "gtk")]
pub use gtk::LoadFontError;
#[cfg(feature = "gtk")]
pub use gtk::RegisterFontError;
#[cfg(feature = "gtk")]
pub use gtk::RegisterGlyphError;
#[cfg(feature = "gtk")]
pub use html_builder::HtmlBuilder;
#[cfg(feature = "gtk")]
pub use html_export::HtmlExport;
#[cfg(feature = "gtk")]
pub use html_svg_export::HtmlSvgExport;
pub use icon_css::icon_css;
pub use icon_name::IconName;
pub use icon_name::IconNameParseError;
pub use icon_set::IconSetFile;
pub use icon_set::IconSetPostProcessor;
pub use icon_set::IconSetSourceLocation;
pub use icon_set::IconSetSourceSpec;
pub use icon_set::IconSetSpec;
pub use icon_set::IconSetSpecError;
pub use icon_set::IconWhitelistEntry;
pub use icon_set_detect::IconSetDetect;
#[cfg(feature = "gtk")]
pub use icon_theme_font::IconThemeFont;
pub use keyword::GlyphKeyword;
pub use layout::Package;
pub use metadata::GlyphAliasMap;
pub use metadata::GlyphCategoryMap;
pub use metadata::GlyphGroup;
pub use metadata::GlyphKeywordMap;
pub use metadata::GlyphMetadata;
pub use metadata::GlyphOrdering;
pub use metadata::GroupLabel;
#[cfg(feature = "gtk")]
pub use pango_font::PangoFont;
pub use paths::GRESOURCE_BASE_PREFIX;
pub use paths::ICONS_CONTEXT_EMOJI;
pub use paths::ICONS_CONTEXT_GLYPHS;
pub use paths::SCALABLE_DIR;
pub use resource_path::ResourcePath;
#[cfg(feature = "render")]
pub use software_render::SoftwareRender;
pub use upstream_icon::UpstreamIcon;
pub use upstream_icons::UpstreamIcons;
