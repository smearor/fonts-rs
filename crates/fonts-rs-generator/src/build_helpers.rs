//! Shared build-time helpers for font crates with variants.
//!
//! Provides common utilities used by `build.rs` scripts across font family
//! crates that support multiple font variants via Cargo features.

use std::path::Path;

/// FNV-1a hash over a string's bytes.
///
/// Mixed into `FontBuild::extra_hash` so changes to external metadata
/// sources (name maps, codepoint tables, stylesheets) force a re-export
/// even when the font file itself is unchanged - the default hash only
/// covers the font file.
pub fn hash_str(content: &str) -> u64 {
    content
        .bytes()
        .fold(0xcbf29ce484222325_u64, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3))
}

/// FNV-1a hash over the sorted filenames and file contents in `svg_dir`.
///
/// Mixed into `FontBuild::extra_hash` so changes to a pre-rendered SVG
/// directory (e.g. `resources/svg/`) force a re-export even when the
/// font file is unchanged - the default hash only covers the font file.
pub fn hash_svg_dir(svg_dir: &Path) -> u64 {
    let mut names: Vec<String> = std::fs::read_dir(svg_dir)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names.iter().fold(0xcbf29ce484222325_u64, |hash, name| {
        let content = std::fs::read(svg_dir.join(name)).unwrap_or_default();
        name.bytes()
            .chain(content.iter().copied())
            .fold(hash, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3))
    })
}

/// Set `cargo:rustc-env` with the absolute font path for `include_bytes!` in lib.rs.
///
/// Constructs the absolute path from `CARGO_MANIFEST_DIR` and the given relative
/// font path, then emits a `cargo:rustc-env` directive so that `include_bytes!`
/// can locate the font file at compile time.
pub fn set_font_path_env(env_var: &str, font_path: &Path) -> miette::Result<()> {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR")
        .or_else(|_| std::env::current_dir().map(|d| d.to_string_lossy().to_string()))
        .map_err(|e| miette::miette!("failed to determine crate directory: {e}"))?;
    let absolute_font_path = Path::new(&crate_dir).join(font_path);
    println!("cargo:rustc-env={env_var}={}", absolute_font_path.display());
    Ok(())
}
