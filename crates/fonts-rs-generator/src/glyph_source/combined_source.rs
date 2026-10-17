//! [`GlyphSource`] that merges several sub-sources into one export.
//!
//! [`CombinedSource`] concatenates the glyph lists of its sub-sources
//! and routes [`svg_for`](GlyphSource::svg_for) back to the owning
//! source via a name marker. It exists for families whose upstream set
//! ships several *styles* (fill/outline, regular/filled, …) that a
//! single build should export side by side - e.g. so the cheat sheet
//! can group and collapse them via `GlyphOrdering`.
//!
//! Names must disambiguate across sources (e.g. `eva-{name}-symbolic`
//! vs `eva-{name}-outline-symbolic`); a duplicate resolved name or
//! codepoint fails the build.

use crate::glyph_source::descriptor::GlyphDescriptor;
use crate::glyph_source::error::GlyphSourceError;
use crate::glyph_source::source::GlyphSource;
use std::collections::HashSet;

/// Merges multiple [`GlyphSource`]s into a single export.
///
/// `svg_for` routes a glyph back to exactly the source that produced
/// it: [`glyphs`](GlyphSource::glyphs) tags each descriptor's
/// `source_key` with the owner's registration index. (The `source()`
/// *name marker* remains as fallback routing for descriptors that did
/// not pass through `glyphs()`.) Registration order still matters for
/// `glyphs`: sub-sources allocate codepoints in registration order, so
/// register the style whose codepoints must stay stable first.
///
/// All sources must be of the same concrete type - typically
/// [`SvgDirSource`](crate::glyph_source::SvgDirSource) with one entry
/// per style directory.
pub struct CombinedSource<S: GlyphSource> {
    /// `(name marker, source)` pairs, checked in order for `svg_for`.
    sources: Vec<(String, S)>,
}

impl<S: GlyphSource> CombinedSource<S> {
    /// Creates an empty combined source.
    pub fn new() -> Self {
        Self { sources: Vec::new() }
    }

    /// Adds a sub-source whose glyphs are routed back when `name`
    /// contains `name_marker` (e.g. `"-outline"`). For `svg_for` the
    /// longest matching marker wins; for `glyphs` sources are visited
    /// in registration order.
    pub fn source(mut self, name_marker: impl Into<String>, source: S) -> Self {
        self.sources.push((name_marker.into(), source));
        self
    }
}

impl<S: GlyphSource> Default for CombinedSource<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: GlyphSource> GlyphSource for CombinedSource<S> {
    /// Returns the concatenation of all sub-source glyph lists.
    ///
    /// Each descriptor's `source_key` is tagged with the owning
    /// source's registration index (`"{index}\u{1F}{key}"`) so
    /// `svg_for` routes back to exactly that source - names and
    /// filenames may coincide across sources (e.g. two crates shipping
    /// a `check.svg`). The tag never reaches serialized output:
    /// `GlyphEntry` derives `file`/`resource_path` from the name.
    ///
    /// Fails when two sources resolve to the same glyph name or
    /// codepoint - merged exports must partition the glyph namespace.
    fn glyphs(&self) -> Result<Vec<GlyphDescriptor>, GlyphSourceError> {
        let mut descriptors = Vec::new();
        for (index, (_, source)) in self.sources.iter().enumerate() {
            for mut descriptor in source.glyphs()? {
                descriptor.source_key = format!("{index}\u{1F}{}", descriptor.source_key);
                descriptors.push(descriptor);
            }
        }

        let mut names = HashSet::with_capacity(descriptors.len());
        let mut codes = HashSet::with_capacity(descriptors.len());
        for descriptor in &descriptors {
            if !names.insert(&descriptor.name) {
                return Err(GlyphSourceError::PostProcess(format!("combined sources produced duplicate glyph name '{}'", descriptor.name)));
            }
            if let Some(code) = descriptor.code
                && !codes.insert(code.as_char())
            {
                return Err(GlyphSourceError::PostProcess(format!(
                    "combined sources produced duplicate codepoint U+{:04X} ('{}')",
                    code.as_char() as u32,
                    descriptor.name
                )));
            }
        }
        Ok(descriptors)
    }

    /// Routes to the source that produced the descriptor.
    ///
    /// Descriptors passing through [`glyphs`](Self::glyphs) carry their
    /// owner's registration index in `source_key`; that tag takes
    /// precedence. Foreign descriptors fall back to the longest
    /// matching name marker (the original routing rule).
    fn svg_for(&self, glyph: &GlyphDescriptor) -> Result<String, GlyphSourceError> {
        if let Some((index, key)) = glyph.source_key.split_once('\u{1F}')
            && let Ok(index) = index.parse::<usize>()
            && let Some((_, source)) = self.sources.get(index)
        {
            let mut routed = glyph.clone();
            routed.source_key = key.to_string();
            return source.svg_for(&routed);
        }

        self.sources
            .iter()
            .filter(|(marker, _)| glyph.name.contains(marker.as_str()))
            .max_by_key(|(marker, _)| marker.len())
            .map(|(_, source)| source.svg_for(glyph))
            .unwrap_or_else(|| Err(GlyphSourceError::PostProcess(format!("no combined source matches glyph '{}'", glyph.name))))
    }
}
