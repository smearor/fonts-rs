//! Input bundle for [`crate::build`].

use std::path::PathBuf;

use crate::Variant;

/// Inputs for the Noto Emoji build pipeline.
///
/// In the multi-crate layout the paths point into the resource crates
/// via `DEP_*` environment variables; the legacy single crate passes
/// its local `resources/` directories instead.
#[derive(Debug, Clone)]
pub struct BuildInput {
    /// Directories containing pre-rendered `emoji_u*.svg` artwork,
    /// merged into a single export (glyphs + sequences crates). Each
    /// directory's files must partition the glyph set - a filename or
    /// resolved-name duplicate across directories collides.
    pub svg_dirs: Vec<PathBuf>,

    /// Directory containing the CLDR/UTS-#51 metadata files
    /// (`annotations.json`, `annotations-derived.json`,
    /// `emoji-test.txt`, `emoji_17_0_ordering.json`).
    pub metadata_dir: PathBuf,

    /// TTF variant to build.
    pub variant: Variant,

    /// `rustc-env` variable receiving the absolute TTF path for
    /// `pixel-drawing` (`NOTO_EMOJI_FONT_PATH` for noto-emoji).
    pub font_path_env: &'static str,
}
