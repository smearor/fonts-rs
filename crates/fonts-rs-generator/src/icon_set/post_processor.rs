//! The [`IconSetPostProcessorExt`] trait - applies spec post-processors
//! to an [`SvgDirSource`].
//!
//! `IconSetPostProcessor` lives in `fonts-rs-model` (it is part of the
//! `icons.toml` wire format); this extension trait provides the
//! generator-side wiring to the concrete `SvgPostProcessor`
//! implementations, which the model cannot see.

use crate::glyph_source::CurrentColor;
use crate::glyph_source::Optimize;
use crate::glyph_source::PaintTarget;
use crate::glyph_source::StripEntityNamespaces;
use crate::glyph_source::SvgDirSource;
use crate::glyph_source::SymbolicClasses;
use fonts_rs_model::IconSetPostProcessor;

/// Applies `icons.toml` post-processor selections to an
/// [`SvgDirSource`].
pub trait IconSetPostProcessorExt {
    /// Adds this post-processor to the source's pipeline.
    fn apply<'a>(&self, source: SvgDirSource<'a>) -> SvgDirSource<'a>;
}

impl IconSetPostProcessorExt for IconSetPostProcessor {
    fn apply<'a>(&self, source: SvgDirSource<'a>) -> SvgDirSource<'a> {
        match self {
            Self::SymbolicClasses => source.post_process(SymbolicClasses),
            Self::CurrentColorFill => source.post_process(CurrentColor::new(PaintTarget::Fill)),
            Self::CurrentColorFillInject => source.post_process(CurrentColor::new(PaintTarget::Fill).inject_missing()),
            Self::CurrentColorStroke => source.post_process(CurrentColor::new(PaintTarget::Stroke)),
            Self::Optimize => source.post_process(Optimize),
            Self::StripEntityNamespaces => source.post_process(StripEntityNamespaces),
        }
    }
}
