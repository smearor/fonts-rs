//! SVG generation for Nerd Font glyph export.
//!
//! Provides [`SvgPathBuilder`] for converting font outlines to SVG path data
//! and [`glyph_to_svg`] for generating complete SVG icons from font glyphs.

mod export;
mod path_builder;

pub use export::EMPTY_SVG;
pub use export::glyph_to_svg;
pub use path_builder::SvgPathBuilder;
