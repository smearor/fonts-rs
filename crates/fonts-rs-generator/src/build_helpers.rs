//! Shared build-time helpers for font crates with variants.
//!
//! Provides common utilities used by `build.rs` scripts across font family
//! crates that support multiple font variants via Cargo features.

use std::path::Path;

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
