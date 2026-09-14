//! Builder-pattern runner for `build.rs` scripts.
//!
//! Encapsulates the common boilerplate shared across all font crate build scripts:
//! hash-based change detection, conditional glyph export, GResource compilation,
//! and codepoint map / Rust constants generation.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use fonts_rs_model::GlyphEntry;

use crate::CodemapGenerator;
use crate::DefaultNaming;
use crate::GlyphGenerator;
use crate::RustConstantsGenerator;
use crate::build_constants::FONT_GRESOURCE;
use crate::build_constants::FONT_GRESOURCE_XML;
use crate::build_constants::ICONS_GRESOURCE;
use crate::build_constants::RESOURCES_DIR;
use crate::build_constants::hash_font_file;
use crate::build_constants::hash_path;
use crate::build_constants::icons_gresource_xml;
use crate::build_constants::metadata_path;
use crate::build_constants::out_dir;
use crate::gresource::GResourceSpec;

/// Builder for the common `build.rs` pipeline.
///
/// Handles hash-based change detection, conditional glyph export,
/// GResource compilation, and code generator invocation.
///
/// # Example
///
/// ```ignore
/// // In build.rs:
/// FontBuild::new("resources/MyFont-Regular.ttf")
///     .compile_font_gresource()
///     .run(|font_path, resources_dir| {
///         MyDefinition::export_glyphs(font_path, resources_dir)
///     })?;
/// ```
pub struct FontBuild {
    /// Path to the TTF/OTF font file (relative to crate root).
    font_path: PathBuf,
    /// Whether to also compile `font.gresource` in addition to `icons.gresource`.
    compile_font_gresource: bool,
    /// Optional extra hash component for change detection.
    ///
    /// When set, the stored hash is `"{font_hash}-{extra_hash}"` instead of just
    /// `"{font_hash}"`. This forces a re-export when the extra value changes
    /// (e.g. a variant name), even if the font file itself is unchanged.
    extra_hash: Option<String>,
    /// Extra `cargo:rerun-if-changed` paths beyond the font file.
    extra_rerun_if_changed: Vec<PathBuf>,
    /// Extra GResource bundles to compile beyond `icons.gresource` (and optionally `font.gresource`).
    additional_gresources: Vec<GResourceSpec>,
}

impl FontBuild {
    /// Create a new builder with the given font path.
    pub fn new(font_path: impl Into<PathBuf>) -> Self {
        Self {
            font_path: font_path.into(),
            compile_font_gresource: false,
            extra_hash: None,
            extra_rerun_if_changed: Vec::new(),
            additional_gresources: Vec::new(),
        }
    }

    /// Add an extra hash component to change detection.
    ///
    /// When the extra hash changes (e.g. a variant name), glyphs are re-exported
    /// even if the font file itself hasn't changed. Useful for variable fonts
    /// where the same TTF is rendered at different axis locations.
    pub fn extra_hash(mut self, hash: impl Into<String>) -> Self {
        self.extra_hash = Some(hash.into());
        self
    }

    /// Enable compilation of `font.gresource` in addition to `icons.gresource`.
    pub fn compile_font_gresource(mut self) -> Self {
        self.compile_font_gresource = true;
        self
    }

    /// Add an extra `cargo:rerun-if-changed` path.
    ///
    /// Can be called multiple times to register multiple paths.
    pub fn rerun_if_changed(mut self, path: impl Into<PathBuf>) -> Self {
        self.extra_rerun_if_changed.push(path.into());
        self
    }

    /// Add an extra GResource bundle to compile.
    ///
    /// Can be called multiple times to register multiple bundles.
    /// Each bundle is compiled as
    /// `glib_build_tools::compile_resources(&["resources"], xml, output)`.
    pub fn additional_gresource(mut self, xml: impl Into<PathBuf>, output: impl Into<PathBuf>) -> Self {
        self.additional_gresources.push(GResourceSpec::new(xml, output));
        self
    }

    /// Run the build pipeline with default code generation.
    ///
    /// The `export` closure is called only when the font file has changed
    /// (detected via FNV-1a hash comparison). It receives the font path
    /// and resources directory as `&Path` arguments.
    ///
    /// After export (if needed), GResources are compiled and codepoint
    /// maps / Rust constants are generated from `metadata.json`.
    ///
    /// # Errors
    ///
    /// Returns an error if any file operation or code generation fails.
    pub fn run<F, E>(self, export: F) -> Result<(), E>
    where
        F: FnOnce(&Path, &Path) -> Result<usize, E>,
        E: From<std::io::Error>,
    {
        self.run_with(export, |json| {
            let entries: Vec<GlyphEntry<String>> = serde_json::from_str(json).map_err(std::io::Error::other)?;
            CodemapGenerator::<DefaultNaming>::run(&entries).map_err(|e| std::io::Error::other(e.to_string()))?;
            RustConstantsGenerator::run(&entries).map_err(|e| std::io::Error::other(e.to_string()))?;
            Ok(())
        })
    }

    /// Run the build pipeline with a custom code generation closure.
    ///
    /// Like [`run`](Self::run), but the `generate` closure replaces the default
    /// `CodemapGenerator` + `RustConstantsGenerator` code generation. The closure
    /// receives the raw `metadata.json` content as a string slice and can
    /// deserialize it as any type, run arbitrary generators, etc.
    ///
    /// # Errors
    ///
    /// Returns an error if any file operation or code generation fails.
    pub fn run_with<F, G, E>(self, export: F, generate: G) -> Result<(), E>
    where
        F: FnOnce(&Path, &Path) -> Result<usize, E>,
        G: FnOnce(&str) -> Result<(), E>,
        E: From<std::io::Error>,
    {
        println!("cargo:rustc-cfg=is_lib");
        println!("cargo:rerun-if-changed={}", self.font_path.display());

        for path in &self.extra_rerun_if_changed {
            println!("cargo:rerun-if-changed={}", path.display());
        }

        let out = out_dir();
        let metadata_path = metadata_path();
        let hash_path = hash_path();
        let icons_xml = icons_gresource_xml();
        let current_hash = hash_font_file(&self.font_path);
        let current_hash = match &self.extra_hash {
            Some(extra) => format!("{current_hash}-{extra}"),
            None => current_hash,
        };

        let needs_export = !metadata_path.exists() || fs::read_to_string(&hash_path).ok().as_deref() != Some(current_hash.as_str());

        if needs_export {
            eprintln!("build.rs: exporting glyphs from {}...", self.font_path.display());
            let count = export(&self.font_path, &out)?;
            eprintln!("build.rs: exported {count} glyphs");
            fs::write(&hash_path, &current_hash)?;
        }

        let out_str = out.to_str().ok_or_else(|| std::io::Error::other("invalid UTF-8 in OUT_DIR"))?;
        let icons_xml_str = icons_xml
            .to_str()
            .ok_or_else(|| std::io::Error::other("invalid UTF-8 in icons gresource xml path"))?;
        glib_build_tools::compile_resources(&[out_str], icons_xml_str, ICONS_GRESOURCE);

        if self.compile_font_gresource {
            glib_build_tools::compile_resources(&[RESOURCES_DIR], FONT_GRESOURCE_XML, FONT_GRESOURCE);
        }

        for gresource in &self.additional_gresources {
            let xml = gresource
                .xml
                .to_str()
                .ok_or_else(|| std::io::Error::other("invalid UTF-8 in gresource xml path"))?;
            let output = gresource
                .output
                .to_str()
                .ok_or_else(|| std::io::Error::other("invalid UTF-8 in gresource output path"))?;
            glib_build_tools::compile_resources(&[RESOURCES_DIR], xml, output);
        }

        let json = fs::read_to_string(&metadata_path)?;
        generate(&json)?;

        Ok(())
    }
}
