//! Generic glyph source abstraction for the build pipeline.
//!
//! [`GlyphSource`] decouples glyph identity and artwork from the export
//! pipeline: `FontFileSource` extracts outlines from TTF/OTF fonts via
//! `skrifa`, `SvgDirSource` consumes directories of pre-rendered SVGs.
//! Both feed the shared `export` runner, which emits identical
//! artifacts (`metadata.json`, `icons.gresource.xml`, SVG files).
//!
//! Pluggable seams: [`SvgFilenameParser`] for filename → glyph identity,
//! [`SvgPostProcessor`] for artwork transformation, and
//! [`CodepointStrategy`] for codepoint assignment.

pub mod codepoint_strategy;
pub mod combined_source;
pub mod current_color;
pub mod descriptor;
pub mod emoji_u_parser;
pub mod error;
pub mod export_target;
pub mod filename_parser;
pub mod font_file_source;
pub mod name_parser;
pub mod normalize_view_box;
pub mod optimize;
pub mod paint_target;
pub mod parsed_glyph;
pub mod post_processor;
pub mod pua_allocator;
pub mod source;
pub mod strip_entity_namespaces;
pub mod svg_dir_source;
pub mod symbolic_classes;

pub use codepoint_strategy::CodepointStrategy;
pub use combined_source::CombinedSource;
pub use current_color::CurrentColor;
pub use descriptor::GlyphDescriptor;
pub use emoji_u_parser::EmojiUFilenameParser;
pub use error::GlyphSourceError;
pub use export_target::GlyphExportTarget;
pub use filename_parser::SvgFilenameParser;
pub use font_file_source::FontFileSource;
pub use name_parser::NameParser;
pub use normalize_view_box::NormalizeViewBox;
pub use optimize::Optimize;
pub use paint_target::PaintTarget;
pub use parsed_glyph::ParsedGlyph;
pub use post_processor::SvgPostProcessor;
pub use pua_allocator::PuaAllocator;
pub use source::GlyphSource;
pub use strip_entity_namespaces::StripEntityNamespaces;
pub use svg_dir_source::SvgDirSource;
pub use symbolic_classes::SymbolicClasses;
