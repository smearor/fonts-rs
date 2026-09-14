//! Shared build-time constants and utilities for font crates.
//!
//! This module provides common path constants and a font-file hashing
//! function used by `build.rs` scripts across all font family crates.

use std::path::PathBuf;

/// Default resources directory (relative to crate root).
///
/// Used for static resources like TTF/OTF font files and static GResource XML
/// manifests (e.g. `font.gresource.xml`). Generated artifacts are written to
/// `OUT_DIR` instead, see [`out_dir`].
pub const RESOURCES_DIR: &str = "resources";

/// Compiled icons GResource output name (placed in `OUT_DIR` by `glib_build_tools`).
pub const ICONS_GRESOURCE: &str = "icons.gresource";

/// Font GResource XML path (relative to crate root, static file).
pub const FONT_GRESOURCE_XML: &str = "resources/font.gresource.xml";

/// Compiled font GResource output name (placed in `OUT_DIR` by `glib_build_tools`).
pub const FONT_GRESOURCE: &str = "font.gresource";

/// Returns the build output directory (`OUT_DIR`).
///
/// All generated artifacts (SVGs, `metadata.json`, `icons.gresource.xml`,
/// `.font-hash`) are written here instead of the source tree, keeping
/// `cargo publish` clean without `--allow-dirty` or `--no-verify`.
pub fn out_dir() -> PathBuf {
    std::env::var("OUT_DIR").map_or_else(|_| PathBuf::from(RESOURCES_DIR), PathBuf::from)
}

/// Metadata JSON path (in `OUT_DIR`).
pub fn metadata_path() -> PathBuf {
    out_dir().join("metadata.json")
}

/// Font hash cache path (in `OUT_DIR`).
pub fn hash_path() -> PathBuf {
    out_dir().join(".font-hash")
}

/// Icons GResource XML path (in `OUT_DIR`).
pub fn icons_gresource_xml() -> PathBuf {
    out_dir().join("icons.gresource.xml")
}

/// Computes a fast FNV-1a-like hash of a font file.
///
/// Used by `build.rs` to detect whether the font has changed and
/// glyph export needs to re-run.
pub fn hash_font_file(path: impl AsRef<std::path::Path>) -> String {
    use std::fs;
    use std::io::Read;

    let mut file = fs::File::open(path).expect("Failed to open font file");
    let mut buffer = [0u8; 8192];
    let mut hash: u64 = 0xcbf29ce484222325;
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        for &byte in &buffer[..n] {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    format!("{hash:016x}")
}
