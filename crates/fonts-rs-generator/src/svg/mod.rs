//! SVG generation for font glyph export.
//!
//! Provides [`SvgPathBuilder`] for converting font outlines to SVG path data
//! and [`Font::glyph_to_svg`] for generating complete SVG icons from font glyphs.

mod export;
mod path_builder;
mod svg;

pub use export::EMPTY_SVG;
pub use path_builder::SvgPathBuilder;
pub use svg::Svg;
