//! Shared build-time constants and utilities for font crates.
//!
//! This module provides common path constants and a font-file hashing
//! function used by `build.rs` scripts across all font family crates.

/// Default resources directory (relative to crate root).
pub const RESOURCES_DIR: &str = "resources";

/// Metadata JSON path (relative to crate root).
pub const METADATA_PATH: &str = "resources/metadata.json";

/// Font hash cache path (relative to crate root).
pub const HASH_PATH: &str = "resources/.font-hash";

/// Icons GResource XML path (relative to crate root).
pub const ICONS_GRESOURCE_XML: &str = "resources/icons.gresource.xml";

/// Compiled icons GResource output name.
pub const ICONS_GRESOURCE: &str = "icons.gresource";

/// Font GResource XML path (relative to crate root).
pub const FONT_GRESOURCE_XML: &str = "resources/font.gresource.xml";

/// Compiled font GResource output name.
pub const FONT_GRESOURCE: &str = "font.gresource";

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
