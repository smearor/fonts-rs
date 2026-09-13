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
use crate::GlyphGenerator;
use crate::RustConstantsGenerator;
use crate::build_constants::FONT_GRESOURCE;
use crate::build_constants::FONT_GRESOURCE_XML;
use crate::build_constants::HASH_PATH;
use crate::build_constants::ICONS_GRESOURCE;
use crate::build_constants::ICONS_GRESOURCE_XML;
use crate::build_constants::METADATA_PATH;
use crate::build_constants::RESOURCES_DIR;
use crate::build_constants::hash_font_file;

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
}

impl FontBuild {
    /// Create a new builder with the given font path.
    pub fn new(font_path: impl Into<PathBuf>) -> Self {
        Self {
            font_path: font_path.into(),
            compile_font_gresource: false,
            extra_hash: None,
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

    /// Run the build pipeline.
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
        println!("cargo:rustc-cfg=is_lib");
        println!("cargo:rerun-if-changed={}", self.font_path.display());

        let metadata_path = Path::new(METADATA_PATH);
        let hash_path = Path::new(HASH_PATH);
        let current_hash = hash_font_file(&self.font_path);
        let current_hash = match &self.extra_hash {
            Some(extra) => format!("{current_hash}-{extra}"),
            None => current_hash,
        };

        let needs_export = !metadata_path.exists() || fs::read_to_string(hash_path).ok().as_deref() != Some(current_hash.as_str());

        if needs_export {
            eprintln!("build.rs: exporting glyphs from {}...", self.font_path.display());
            let count = export(&self.font_path, Path::new(RESOURCES_DIR))?;
            eprintln!("build.rs: exported {count} glyphs");
            fs::write(hash_path, &current_hash)?;
        }

        glib_build_tools::compile_resources(&[RESOURCES_DIR], ICONS_GRESOURCE_XML, ICONS_GRESOURCE);

        if self.compile_font_gresource {
            glib_build_tools::compile_resources(&[RESOURCES_DIR], FONT_GRESOURCE_XML, FONT_GRESOURCE);
        }

        let json = fs::read_to_string(METADATA_PATH)?;
        let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&json).map_err(std::io::Error::other)?;
        CodemapGenerator::run(&entries).map_err(|e| std::io::Error::other(e.to_string()))?;
        RustConstantsGenerator::run(&entries).map_err(|e| std::io::Error::other(e.to_string()))?;

        Ok(())
    }
}
