//! Pango font loading for Nerd Font symbol fonts.
//!
//! Provides [`load_fonts`] which registers the Nerd Font TTF files
//! with the global Pango-Cairo `FontMap` using `add_font_file`. This bypasses
//! the GTK4 CSS `@font-face` parser, which does not reliably load custom fonts.
//!
//! Fonts are loaded into the global font map, so GTK4 widgets automatically
//! see them without needing a widget reference.

use std::sync::OnceLock;

#[cfg(feature = "embed-fonts")]
use std::io::Write;

use pango::prelude::FontMapExt;

use crate::init::InitError;

/// Relative path to the proportional Nerd Font symbol font.
#[cfg(not(feature = "embed-fonts"))]
const SYMBOLS_REGULAR_RELATIVE: &str = "resources/NerdFontsSymbolsOnly/SymbolsNerdFont-Regular.ttf";

/// Relative path to the monospace Nerd Font symbol font.
#[cfg(not(feature = "embed-fonts"))]
const SYMBOLS_MONO_RELATIVE: &str = "resources/NerdFontsSymbolsOnly/SymbolsNerdFontMono-Regular.ttf";

/// System-wide base directory (Debian package install location).
#[cfg(not(feature = "embed-fonts"))]
const SYSTEM_BASE_DIR: &str = "/usr/share/smearor";

/// Standard system font directories searched when `embed-fonts` is disabled.
#[cfg(not(feature = "embed-fonts"))]
fn system_font_dirs() -> Vec<std::path::PathBuf> {
    #[cfg(target_os = "linux")]
    {
        vec![
            std::path::PathBuf::from("/usr/share/fonts"),
            std::path::PathBuf::from("/usr/local/share/fonts"),
        ]
    }
    #[cfg(target_os = "macos")]
    {
        vec![
            std::path::PathBuf::from("/Library/Fonts"),
            std::path::PathBuf::from("/System/Library/Fonts"),
        ]
    }
    #[cfg(target_os = "windows")]
    {
        vec![
            std::path::PathBuf::from("C:\\Windows\\Fonts"),
        ]
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        vec![]
    }
}

/// Returns the user font directory.
#[cfg(not(feature = "embed-fonts"))]
fn user_font_dir() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
            let p = std::path::PathBuf::from(xdg).join("fonts");
            if p.is_dir() {
                return Some(p);
            }
        }
        if let Some(home) = std::env::var_os("HOME") {
            let p = std::path::PathBuf::from(home).join(".local/share/fonts");
            if p.is_dir() {
                return Some(p);
            }
        }
        None
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            let p = std::path::PathBuf::from(home).join("Library/Fonts");
            if p.is_dir() {
                return Some(p);
            }
        }
        None
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            let p = std::path::PathBuf::from(local).join("Microsoft\\Windows\\Fonts");
            if p.is_dir() {
                return Some(p);
            }
        }
        None
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

/// Configured base directory set via [`set_base_dir`].
#[cfg(not(feature = "embed-fonts"))]
static BASE_DIR: OnceLock<Option<String>> = OnceLock::new();

/// Temp file guards kept alive for the process lifetime to prevent early cleanup.
#[cfg(feature = "embed-fonts")]
static TEMP_GUARDS: OnceLock<Vec<tempfile::NamedTempFile>> = OnceLock::new();

/// Loads the Nerd Font symbol fonts into the global Pango-Cairo font map.
///
/// This uses `pangocairo::FontMap::default()` to obtain the global font map,
/// so no widget or window reference is needed. GTK4 widgets automatically
/// see the loaded fonts.
///
/// Call this once at startup, before or after GTK initialization:
///
/// ```no_run
/// use nerd_fonts_rs::load_fonts;
///
/// if let Err(e) = load_fonts() {
///     eprintln!("Failed to load nerd fonts: {e}");
/// }
/// ```
///
/// With the `embed-fonts` feature, the TTF data is compiled into the binary
/// via `include_bytes!` and written to temporary files. The temp file guards
/// are kept alive in a static `OnceLock` for the process lifetime.
///
/// Without `embed-fonts`, the font files are located on disk and passed
/// directly to `add_font_file` — no temporary files are created.
///
/// # Errors
///
/// Returns [`InitError`] if writing temp files, locating font files, or
/// adding them to the font map fails.
pub fn load_fonts() -> Result<(), InitError> {
    let font_map = pangocairo::FontMap::default();

    #[cfg(feature = "embed-fonts")]
    {
        let regular_bytes = include_bytes!("../resources/NerdFontsSymbolsOnly/SymbolsNerdFont-Regular.ttf");
        let mono_bytes = include_bytes!("../resources/NerdFontsSymbolsOnly/SymbolsNerdFontMono-Regular.ttf");

        let (regular_path, regular_guard) = write_temp_font(regular_bytes, "SymbolsNerdFont-Regular")?;
        let (mono_path, mono_guard) = write_temp_font(mono_bytes, "SymbolsNerdFontMono-Regular")?;

        // Store guards to keep temp files alive for the process lifetime.
        let _ = TEMP_GUARDS.set(vec![regular_guard, mono_guard]);

        font_map
            .add_font_file(&regular_path)
            .map_err(|e| InitError::GResourceRegister(e.to_string()))?;
        font_map
            .add_font_file(&mono_path)
            .map_err(|e| InitError::GResourceRegister(e.to_string()))?;
    }

    #[cfg(not(feature = "embed-fonts"))]
    {
        let regular_path = find_font_file(SYMBOLS_REGULAR_RELATIVE)?;
        let mono_path = find_font_file(SYMBOLS_MONO_RELATIVE)?;

        font_map
            .add_font_file(&regular_path)
            .map_err(|e| InitError::GResourceRegister(e.to_string()))?;
        font_map
            .add_font_file(&mono_path)
            .map_err(|e| InitError::GResourceRegister(e.to_string()))?;
    }

    Ok(())
}

/// Sets the base directory for locating font files on disk.
///
/// Only used when the `embed-fonts` feature is disabled.
#[cfg(not(feature = "embed-fonts"))]
pub fn set_base_dir(dir: Option<String>) {
    let _ = BASE_DIR.set(dir);
}

/// Writes embedded font bytes to a temp file and returns the path plus guard.
///
/// The returned `NamedTempFile` guard must be kept alive to prevent the
/// temp file from being deleted. Store it in a static `OnceLock`.
#[cfg(feature = "embed-fonts")]
fn write_temp_font(data: &[u8], prefix: &str) -> Result<(std::path::PathBuf, tempfile::NamedTempFile), InitError> {
    let mut temp = tempfile::Builder::new()
        .prefix(prefix)
        .suffix(".ttf")
        .tempfile()
        .map_err(|e| InitError::GResourceRegister(format!("failed to create temp file: {e}")))?;

    temp.write_all(data)
        .map_err(|e| InitError::GResourceRegister(format!("failed to write font data: {e}")))?;

    let path = temp.path().to_path_buf();
    Ok((path, temp))
}

/// Finds a font file on disk by searching candidate paths.
///
/// Search order: init-provided base dir, relative path, system-wide, exe-relative.
#[cfg(not(feature = "embed-fonts"))]
fn find_font_file(relative: &str) -> Result<std::path::PathBuf, InitError> {
    let candidates = candidate_font_paths(relative);
    for path in &candidates {
        if path.exists() {
            return Ok(path.clone());
        }
    }
    Err(InitError::GResourceRegister(format!(
        "font file not found in any of: {}",
        candidates.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")
    )))
}

/// Builds candidate paths for a font file.
///
/// Search order:
/// 1. Init-provided base dir (joined with the relative resource path)
/// 2. Relative path (working directory)
/// 3. System-wide smearor dir (joined with the relative resource path)
/// 4. Executable-relative path
/// 5. Standard system and user font directories (bare filename and subdirectory)
#[cfg(not(feature = "embed-fonts"))]
fn candidate_font_paths(relative: &str) -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();

    // 1. Init-provided base dir
    if let Some(base) = BASE_DIR.get().and_then(|opt| opt.as_ref()) {
        paths.push(std::path::PathBuf::from(base).join(relative));
    }

    // 2. Relative path (working directory)
    paths.push(std::path::PathBuf::from(relative));

    // 3. System-wide smearor dir
    paths.push(std::path::PathBuf::from(SYSTEM_BASE_DIR).join(relative));

    // 4. Executable-relative path
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            paths.push(dir.join(relative));
        }
    }

    // 5. Standard system and user font directories
    let rel_path = std::path::Path::new(relative);
    let filename = rel_path.file_name().unwrap_or_default();
    let parent_name = rel_path.parent().and_then(|p| p.file_name());

    let mut font_dirs = system_font_dirs();
    if let Some(user_dir) = user_font_dir() {
        font_dirs.push(user_dir);
    }

    for dir in font_dirs {
        // Try flat: font_dir/SymbolsNerdFont-Regular.ttf
        paths.push(dir.join(filename));
        // Try subdirectory: font_dir/NerdFontsSymbolsOnly/SymbolsNerdFont-Regular.ttf
        if let Some(parent) = parent_name {
            paths.push(dir.join(parent).join(filename));
        }
    }

    paths
}
