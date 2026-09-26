//! Directory-based [`GlyphSource`] for pre-rendered SVG files.

use crate::font_definition::normalize_to_kebab;
use crate::glyph_source::codepoint_strategy::CodepointStrategy;
use crate::glyph_source::descriptor::GlyphDescriptor;
use crate::glyph_source::error::GlyphSourceError;
use crate::glyph_source::filename_parser::SvgFilenameParser;
use crate::glyph_source::name_parser::NameParser;
use crate::glyph_source::parsed_glyph::ParsedGlyph;
use crate::glyph_source::post_processor::SvgPostProcessor;
use crate::glyph_source::pua_allocator::PuaAllocator;
use crate::glyph_source::source::GlyphSource;
use fonts_rs_model::CodePoint;
use fonts_rs_model::GlyphNameMap;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// Semantic name resolver: maps a parsed glyph to its display name.
type NameResolver<'a> = Box<dyn Fn(&ParsedGlyph) -> Option<String> + 'a>;

/// Glyph filter predicate over the parsed codepoint sequence.
type GlyphFilter<'a> = Box<dyn Fn(&[CodePoint]) -> bool + 'a>;

/// `GlyphSource` backed by a directory of pre-rendered SVG files.
///
/// Iterates `svg_dir`, resolves each file to a [`ParsedGlyph`] via the
/// configured [`SvgFilenameParser`], maps it to a semantic name, applies
/// post-processing hooks, and emits the same artifacts as the TTF path
/// via the shared [`GlyphSource::export`] runner.
pub struct SvgDirSource<'a> {
    /// Directory containing the source SVG files.
    svg_dir: PathBuf,
    /// Filename parser resolving files to glyph identities.
    parser: Box<dyn SvgFilenameParser>,
    /// Optional name→codepoint map for [`CodepointStrategy::FromNameMap`].
    codepoint_map: Option<&'a GlyphNameMap>,
    /// Optional semantic name resolver (e.g. a CLDR-derived
    /// codepoint-sequence → name map for emoji). Falls back to
    /// [`ParsedGlyph::raw_name`].
    name_resolver: Option<NameResolver<'a>>,
    /// Optional glyph name prefix prepended to every resolved name
    /// (e.g. `blobmoji-bitmap-flags`), matching `FontFileSource`'s
    /// `glyph_name_prefix` semantics.
    name_prefix: Option<String>,
    /// Optional predicate for excluding glyphs (e.g. flag emoji).
    filter: Option<GlyphFilter<'a>>,
    /// Post-processing hooks applied to each SVG, in order.
    post_processors: Vec<Box<dyn SvgPostProcessor>>,
    /// Strategy for assigning [`GlyphDescriptor::code`] values.
    codepoint_strategy: CodepointStrategy,
}

impl<'a> SvgDirSource<'a> {
    /// Creates a source over `svg_dir` with the [`NameParser`] default.
    pub fn new(svg_dir: impl Into<PathBuf>) -> Self {
        Self {
            svg_dir: svg_dir.into(),
            parser: Box::new(NameParser),
            codepoint_map: None,
            name_resolver: None,
            name_prefix: None,
            filter: None,
            post_processors: Vec::new(),
            codepoint_strategy: CodepointStrategy::default(),
        }
    }

    /// Sets the filename parser.
    pub fn filename_parser(mut self, parser: impl SvgFilenameParser + 'static) -> Self {
        self.parser = Box::new(parser);
        self
    }

    /// Sets the name→codepoint map for [`CodepointStrategy::FromNameMap`].
    pub fn codepoint_map(mut self, map: &'a GlyphNameMap) -> Self {
        self.codepoint_map = Some(map);
        self
    }

    /// Sets the semantic name resolver (e.g. CLDR names for emoji).
    pub fn name_resolver(mut self, resolver: impl Fn(&ParsedGlyph) -> Option<String> + 'a) -> Self {
        self.name_resolver = Some(Box::new(resolver));
        self
    }

    /// Sets a glyph name prefix prepended to every resolved name.
    ///
    /// Mirrors `ExportConfig::glyph_name_prefix` on the TTF path: the
    /// resolved (or raw fallback) name is kebab-normalized, then prefixed
    /// as `{prefix}-{name}`.
    pub fn name_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.name_prefix = Some(prefix.into());
        self
    }

    /// Sets a glyph filter predicate (e.g. flag-emoji exclusion).
    pub fn filter(mut self, predicate: impl Fn(&[CodePoint]) -> bool + 'a) -> Self {
        self.filter = Some(Box::new(predicate));
        self
    }

    /// Adds a post-processing hook.
    pub fn post_process(mut self, processor: impl SvgPostProcessor + 'static) -> Self {
        self.post_processors.push(Box::new(processor));
        self
    }

    /// Sets the codepoint assignment strategy.
    pub fn codepoint_strategy(mut self, strategy: CodepointStrategy) -> Self {
        self.codepoint_strategy = strategy;
        self
    }
}

impl GlyphSource for SvgDirSource<'_> {
    fn glyphs(&self) -> Result<Vec<GlyphDescriptor>, GlyphSourceError> {
        // Collect and sort by filename - `fs::read_dir` order is not
        // guaranteed, so unsorted input would make PUA probing
        // order-dependent and non-reproducible.
        let mut paths = fs::read_dir(&self.svg_dir)?.collect::<Result<Vec<_>, _>>()?;
        paths.sort_by_key(|entry| entry.file_name());

        let mut allocator = PuaAllocator::new();
        let mut seen_names = HashSet::new();
        let mut descriptors = Vec::new();
        for dir_entry in paths {
            let filename = dir_entry.file_name().to_string_lossy().into_owned();
            let Some(parsed) = self.parser.parse(&filename) else { continue };

            if let (Some(codepoints), Some(filter)) = (&parsed.codepoints, &self.filter)
                && !filter(codepoints)
            {
                continue;
            }

            let Some(base_name) = self.resolve_name(&parsed) else { continue };

            // Resolved names can collide (e.g. CLDR "keycap: #" and
            // "keycap: *" both normalize to "keycap"). Disambiguate with
            // the filename-derived name, then a numeric suffix as
            // backstop. Iteration is sorted, so this is deterministic.
            let mut name = base_name.clone();
            if seen_names.contains(&name)
                && let Some(raw) = normalize_to_kebab(&parsed.raw_name)
            {
                name = format!("{base_name}-{raw}");
            }
            let mut suffix = 2;
            while !seen_names.insert(name.clone()) {
                name = format!("{base_name}-{suffix}");
                suffix += 1;
            }

            descriptors.push(GlyphDescriptor {
                code: self.resolve_codepoint(&parsed, &mut allocator)?,
                name,
                source_key: filename,
                sequence: parsed.codepoints.filter(|codepoints| codepoints.len() > 1),
            });
        }
        Ok(descriptors)
    }

    fn svg_for(&self, glyph: &GlyphDescriptor) -> Result<String, GlyphSourceError> {
        let mut svg = fs::read_to_string(self.svg_dir.join(&glyph.source_key))?;
        for processor in &self.post_processors {
            svg = processor.process(svg)?;
        }
        Ok(svg)
    }
}

impl SvgDirSource<'_> {
    /// Resolves the codepoint for a parsed glyph per the configured
    /// [`CodepointStrategy`].
    ///
    /// Multi-codepoint sequences (ZWJ emoji, keycaps) have no single
    /// codepoint - `FromFilename` allocates a stable PUA codepoint for
    /// them so `code` stays unique in the generated `phf` maps. The
    /// real sequence is preserved in [`GlyphDescriptor::sequence`].
    fn resolve_codepoint(&self, parsed: &ParsedGlyph, allocator: &mut PuaAllocator) -> Result<Option<CodePoint>, GlyphSourceError> {
        match self.codepoint_strategy {
            CodepointStrategy::FromFilename => match &parsed.codepoints {
                Some(codepoints) if codepoints.len() == 1 => Ok(codepoints.first().copied()),
                Some(_) => allocator.allocate(&parsed.raw_name).map(Some),
                None => Ok(None),
            },
            CodepointStrategy::FromNameMap => Ok(self.codepoint_map.and_then(|map| map.get(&parsed.raw_name).copied())),
            CodepointStrategy::HashedPua => allocator.allocate(&parsed.raw_name).map(Some),
            CodepointStrategy::None => Ok(None),
        }
    }

    /// Resolves the semantic name via the configured `name_resolver`,
    /// falling back to the raw filename stem.
    ///
    /// The result is kebab-normalized and, when `name_prefix` is set,
    /// emitted as `{prefix}-{name}`. Returns `None` when normalization
    /// yields an empty name - the glyph is then skipped.
    fn resolve_name(&self, parsed: &ParsedGlyph) -> Option<String> {
        let base = self
            .name_resolver
            .as_ref()
            .and_then(|resolver| resolver(parsed))
            .unwrap_or_else(|| parsed.raw_name.clone());
        let kebab = normalize_to_kebab(&base)?;
        Some(match &self.name_prefix {
            Some(prefix) => format!("{prefix}-{kebab}"),
            None => kebab,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// Creates a temporary SVG directory with the given filenames.
    ///
    /// `name` must be unique per test - tests run in parallel within
    /// the same process.
    fn with_svg_dir(name: &str, filenames: &[&str], test: impl FnOnce(&Path)) {
        let dir = std::env::temp_dir().join(format!("svg_dir_source_test_{}_{}", std::process::id(), name));
        fs::create_dir_all(&dir).expect("create temp dir");
        for filename in filenames {
            fs::write(dir.join(filename), "<svg xmlns=\"http://www.w3.org/2000/svg\"/>").expect("write svg");
        }
        test(&dir);
        fs::remove_dir_all(&dir).expect("remove temp dir");
    }

    #[test]
    fn glyphs_are_sorted_by_filename() {
        with_svg_dir("sorted", &["zeta.svg", "alpha.svg", "mid.svg"], |dir| {
            let source = SvgDirSource::new(dir);
            let glyphs = source.glyphs().expect("glyphs");
            let names: Vec<&str> = glyphs.iter().map(|glyph| glyph.name.as_str()).collect();
            assert_eq!(names, ["alpha", "mid", "zeta"]);
        });
    }

    #[test]
    fn unparseable_files_are_skipped() {
        with_svg_dir("skipped", &["icon.svg", "README.md", ".gitkeep"], |dir| {
            let source = SvgDirSource::new(dir);
            let glyphs = source.glyphs().expect("glyphs");
            assert_eq!(glyphs.len(), 1);
            assert_eq!(glyphs[0].name, "icon");
        });
    }

    #[test]
    fn filter_excludes_glyphs() {
        with_svg_dir("filter", &["emoji_u1f600.svg", "emoji_u1f1e6_1f1fa.svg"], |dir| {
            let source = SvgDirSource::new(dir)
                .filename_parser(crate::glyph_source::EmojiUFilenameParser)
                .filter(|codepoints| codepoints.len() == 1);
            let glyphs = source.glyphs().expect("glyphs");
            assert_eq!(glyphs.len(), 1);
            assert_eq!(glyphs[0].sequence, None);
        });
    }

    #[test]
    fn sequences_get_pua_code_and_sequence_field() {
        with_svg_dir("sequences", &["emoji_u1f600.svg", "emoji_u1f468_200d_1f469.svg"], |dir| {
            let source = SvgDirSource::new(dir)
                .filename_parser(crate::glyph_source::EmojiUFilenameParser)
                .codepoint_strategy(CodepointStrategy::FromFilename);
            let glyphs = source.glyphs().expect("glyphs");
            assert_eq!(glyphs.len(), 2);

            let single = glyphs.iter().find(|glyph| glyph.source_key == "emoji_u1f600.svg").expect("single");
            assert_eq!(single.code.map(|code| code.as_char()), Some('\u{1F600}'));
            assert_eq!(single.sequence, None);

            let sequence = glyphs.iter().find(|glyph| glyph.source_key == "emoji_u1f468_200d_1f469.svg").expect("sequence");
            let code = sequence.code.expect("pua code");
            assert!(('\u{E000}'..='\u{F8FF}').contains(&code.as_char()));
            assert_eq!(sequence.sequence.as_ref().map(Vec::len), Some(3));
        });
    }

    #[test]
    fn svg_for_reads_and_post_processes() {
        with_svg_dir("svg_for", &["icon.svg"], |dir| {
            let source = SvgDirSource::new(dir).post_process(crate::glyph_source::Optimize);
            let glyphs = source.glyphs().expect("glyphs");
            let svg = source.svg_for(&glyphs[0]).expect("svg");
            assert!(svg.contains("<svg"));
        });
    }
}
