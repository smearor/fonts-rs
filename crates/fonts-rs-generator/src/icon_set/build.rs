//! The [`IconSetBuild`] build-time entry point for custom icon sets.

use crate::build_helpers::hash_str;
use crate::build_helpers::hash_svg_dir;
use crate::build_runner::FontBuild;
use crate::generator::GlyphGenerator;
use crate::generator::codemap::CodemapGenerator;
use crate::generator::codemap::DefaultNaming;
use crate::generator::rust_constants::RustConstantsGenerator;
use crate::glyph_source::CodepointStrategy;
use crate::glyph_source::CombinedSource;
use crate::glyph_source::GlyphExportTarget;
use crate::glyph_source::GlyphSource;
use crate::glyph_source::PuaAllocator;
use crate::glyph_source::SvgDirSource;
use crate::icon_set::error::IconSetBuildError;
use crate::icon_set::module_generator::IconSetModuleGenerator;
use crate::icon_set::post_processor::IconSetPostProcessorExt;
use crate::icon_set::source_resolver::IconSetSourceResolver;
use fonts_rs_model::GlyphEntry;
use fonts_rs_model::IconSetFile;
use fonts_rs_model::IconSetSourceSpec;
use fonts_rs_model::IconSetSpec;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

/// Pipeline version mixed into the change-detection hash. Bump when
/// the generated `iconset.rs` shape or the post-processor set changes.
const EXPORT_PIPELINE_VERSION: u32 = 1;

/// Build-time entry point for custom icon sets.
///
/// Reads `icons.toml`, resolves each `[[iconset.sources]]` entry,
/// exports the whitelisted glyphs through a shared-PUA
/// [`CombinedSource`], compiles `icons.gresource`, and generates
/// `codemap.rs`, `icons.rs` and `iconset.rs` in `OUT_DIR`:
///
/// ```ignore
/// fn main() -> miette::Result<()> {
///     fonts_rs_generator::IconSetBuild::from_file("icons.toml")?.run()?;
///     Ok(())
/// }
/// ```
pub struct IconSetBuild {
    /// Parsed and validated `icons.toml` spec.
    spec: IconSetSpec,
    /// Absolute path of the spec file (for `cargo:rerun-if-changed`).
    spec_path: PathBuf,
    /// Raw TOML contents (mixed into the change-detection hash).
    spec_raw: String,
}

impl IconSetBuild {
    /// Loads `icons.toml` relative to `CARGO_MANIFEST_DIR` and
    /// validates it.
    ///
    /// # Errors
    ///
    /// Returns [`IconSetBuildError::SpecNotFound`] when the file does
    /// not exist, [`IconSetBuildError::SpecParse`] on malformed TOML,
    /// and [`IconSetBuildError::Spec`] when validation fails.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, IconSetBuildError> {
        let path = path.as_ref();
        let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").map_or_else(|| PathBuf::from("."), PathBuf::from);
        let spec_path = if path.is_absolute() { path.to_path_buf() } else { manifest_dir.join(path) };

        let spec_raw = fs::read_to_string(&spec_path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                IconSetBuildError::SpecNotFound {
                    path: spec_path.display().to_string(),
                }
            } else {
                IconSetBuildError::Io(error)
            }
        })?;
        let spec = toml::from_str::<IconSetFile>(&spec_raw)
            .map_err(|parse_error| IconSetBuildError::SpecParse {
                path: spec_path.display().to_string(),
                parse_error,
            })?
            .iconset;
        spec.validate()?;
        Ok(Self { spec, spec_path, spec_raw })
    }

    /// Resolves all sources, verifies the whitelists cover real files,
    /// exports the merged glyph set, compiles `icons.gresource` and
    /// writes `codemap.rs`, `icons.rs` and `iconset.rs` into `OUT_DIR`.
    ///
    /// # Errors
    ///
    /// Returns [`IconSetBuildError::UnknownIcon`] when a whitelisted
    /// name matches no SVG in its source,
    /// [`IconSetBuildError::SourceNotFound`] when a location cannot be
    /// resolved, and the export/codegen errors otherwise.
    pub fn run(&self) -> Result<(), IconSetBuildError> {
        let resolver = IconSetSourceResolver::new();
        let mut svg_dirs = Vec::with_capacity(self.spec.sources.len());
        for source in &self.spec.sources {
            svg_dirs.push(resolver.resolve(&source.location)?);
        }
        for (source, svg_dir) in self.spec.sources.iter().zip(&svg_dirs) {
            self.check_whitelist(source, svg_dir)?;
        }

        let allocator = Rc::new(RefCell::new(PuaAllocator::new()));
        let mut combined = CombinedSource::new();
        for (source, svg_dir) in self.spec.sources.iter().zip(&svg_dirs) {
            let whitelist = source.whitelist();
            let renames = source.rename_map();
            let mut svg_source = SvgDirSource::new(svg_dir)
                .name_filter(move |parsed| whitelist.contains(&parsed.raw_name))
                .codepoint_strategy(CodepointStrategy::HashedPua)
                .pua_salt(source.location.salt())
                .pua_allocator(allocator.clone())
                .name_resolver(move |parsed| {
                    let exported = renames.get(&parsed.raw_name).unwrap_or(&parsed.raw_name);
                    Some(format!("{exported}-symbolic"))
                })
                .name_prefix(&self.spec.icon_prefix);
            for processor in &source.post_processors {
                svg_source = processor.apply(svg_source);
            }
            combined = combined.source("", svg_source);
        }

        let target = GlyphExportTarget {
            gresource_prefix: self.spec.gresource_prefix.clone(),
            icons_context: self.spec.icons_context.clone(),
        };

        let mut build = FontBuild::svg_only()
            .extra_hash(EXPORT_PIPELINE_VERSION.to_string())
            .extra_hash(format!("{:016x}", hash_str(&self.spec_raw)))
            .rerun_if_changed(&self.spec_path);
        for svg_dir in &svg_dirs {
            build = build.extra_hash(format!("{:016x}", hash_svg_dir(svg_dir))).rerun_if_changed(svg_dir);
        }

        let spec = &self.spec;
        build.run_with(
            |_font_path, resources_dir| Ok(combined.export_to(&target, resources_dir)?),
            |json| {
                let entries: Vec<GlyphEntry<String>> = serde_json::from_str(json)?;
                CodemapGenerator::<DefaultNaming>::run(&entries)?;
                RustConstantsGenerator::run(&entries)?;
                IconSetModuleGenerator::run(spec)?;
                Ok(())
            },
        )
    }

    /// Verifies every whitelisted upstream name matches an `*.svg`
    /// file stem in the resolved source directory - before any export
    /// work runs.
    fn check_whitelist(&self, source: &IconSetSourceSpec, svg_dir: &Path) -> Result<(), IconSetBuildError> {
        let mut available = HashSet::new();
        for entry in fs::read_dir(svg_dir)? {
            let path = entry?.path();
            if path.extension().is_some_and(|ext| ext == "svg")
                && let Some(stem) = path.file_stem().and_then(|stem| stem.to_str())
            {
                available.insert(stem.to_string());
            }
        }
        for icon in &source.icons {
            let name = icon.upstream_name();
            if !available.contains(name) {
                return Err(IconSetBuildError::UnknownIcon {
                    source_id: source.location.label(),
                    name: name.to_string(),
                    suggestions: closest_names(name, &available, 3),
                });
            }
        }
        Ok(())
    }
}

/// Levenshtein edit distance between two strings.
fn edit_distance(a: &str, b: &str) -> usize {
    let b_len = b.chars().count();
    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut cur = vec![0; b_len + 1];
    for (i, ca) in a.chars().enumerate() {
        cur[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            cur[j + 1] = (prev[j] + usize::from(ca != cb)).min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b_len]
}

/// The `n` closest names to `name` among `candidates`, joined for
/// error messages - `none` when the source is empty.
fn closest_names(name: &str, candidates: &HashSet<String>, n: usize) -> String {
    let mut sorted: Vec<&String> = candidates.iter().collect();
    sorted.sort_by_key(|candidate| edit_distance(name, candidate));
    let closest: Vec<&str> = sorted.iter().take(n).map(|name| name.as_str()).collect();
    if closest.is_empty() { "none".to_string() } else { closest.join(", ") }
}
