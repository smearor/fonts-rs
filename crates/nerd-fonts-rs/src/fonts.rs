//! Font loading with configurable base directory and optional embedding.
//!
//! Fonts are loaded from disk (or embedded via `embed-fonts` feature) and
//! cached in `OnceLock` for the lifetime of the process.

use std::sync::OnceLock;

#[cfg(not(feature = "embed-fonts"))]
use std::path::PathBuf;

use ab_glyph::FontVec;
use tracing::debug;
use tracing::error;
use tracing::trace;

use woff2_patched::decode::convert_woff2_to_ttf;
use woff2_patched::decode::is_woff2;

/// Relative path to the Symbols-only Nerd Font (for icons).
#[cfg(not(feature = "embed-fonts"))]
const NERD_FONT_RELATIVE: &str = "resources/NerdFontsSymbolsOnly/SymbolsNerdFont-Regular.ttf";

/// Relative path to the JetBrains Mono Nerd Font (WOFF2, for labels with full character set).
#[cfg(not(feature = "embed-fonts"))]
const LABEL_FONT_RELATIVE: &str = "resources/JetBrainsMonoNLNerdFont/JetBrainsMonoNLNerdFont-Regular.woff2";

/// System-wide base directory (Debian package install location).
#[cfg(not(feature = "embed-fonts"))]
const SYSTEM_BASE_DIR: &str = "/usr/share/smearor";

/// Configured base directory set via `init()`.
static BASE_DIR: OnceLock<Option<String>> = OnceLock::new();

/// Search candidate paths for a font file: init-provided, system-wide, and executable-relative.
#[cfg(not(feature = "embed-fonts"))]
fn candidate_font_paths(relative: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(base) = BASE_DIR.get().and_then(|opt| opt.as_ref()) {
        paths.push(PathBuf::from(base).join(relative));
    }
    paths.push(PathBuf::from(relative));
    paths.push(PathBuf::from(SYSTEM_BASE_DIR).join(relative));
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            paths.push(dir.join(relative));
        }
    }
    paths
}

/// Read the first matching font file from candidate paths.
#[cfg(not(feature = "embed-fonts"))]
fn read_font_file(relative: &str) -> Result<Vec<u8>, std::io::Error> {
    let candidates = candidate_font_paths(relative);
    for path in &candidates {
        match std::fs::read(path) {
            Ok(data) => {
                trace!("nerd-fonts-gtk: loaded font from {}", path.display());
                return Ok(data);
            }
            Err(_) => continue,
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!(
            "font file not found in any of: {}",
            candidates.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")
        ),
    ))
}

/// Initialize the font loader with a project-specific base directory.
/// Called once at startup before any font access.
pub fn init(base_dir: Option<&str>) {
    let _ = BASE_DIR.set(base_dir.map(|s| s.to_string()));
}

/// Cached Symbols-only Nerd Font loaded from disk.
static NERD_FONT: OnceLock<Option<FontVec>> = OnceLock::new();

/// Cached label font (JetBrains Mono Nerd Font) loaded from disk.
static LABEL_FONT: OnceLock<Option<FontVec>> = OnceLock::new();

/// Get the cached Symbols-only Nerd Font, loading from disk on first access.
pub fn nerd_font() -> Option<&'static FontVec> {
    NERD_FONT
        .get_or_init(|| {
            #[cfg(feature = "embed-fonts")]
            {
                let data = include_bytes!("../resources/NerdFontsSymbolsOnly/SymbolsNerdFont-Regular.ttf").to_vec();
                match FontVec::try_from_vec(data) {
                    Ok(font) => Some(font),
                    Err(e) => {
                        trace!("nerd-fonts-gtk: failed to parse embedded Nerd Font: {}", e);
                        None
                    }
                }
            }
            #[cfg(not(feature = "embed-fonts"))]
            {
                match read_font_file(NERD_FONT_RELATIVE) {
                    Ok(data) => match FontVec::try_from_vec(data) {
                        Ok(font) => Some(font),
                        Err(e) => {
                            trace!("nerd-fonts-gtk: failed to parse Nerd Font: {}", e);
                            None
                        }
                    },
                    Err(e) => {
                        error!("nerd-fonts-gtk: failed to read Nerd Font: {}", e);
                        None
                    }
                }
            }
        })
        .as_ref()
}

/// Get the cached label font (JetBrains Mono Nerd Font), loading from WOFF2 on first access.
pub fn label_font() -> Option<&'static FontVec> {
    LABEL_FONT
        .get_or_init(|| {
            #[cfg(feature = "embed-fonts")]
            {
                let data = include_bytes!("../resources/JetBrainsMonoNLNerdFont/JetBrainsMonoNLNerdFont-Regular.woff2").to_vec();
                convert_woff2(data)
            }
            #[cfg(not(feature = "embed-fonts"))]
            {
                match read_font_file(LABEL_FONT_RELATIVE) {
                    Ok(data) => convert_woff2(data),
                    Err(e) => {
                        error!("nerd-fonts-gtk: failed to read label font: {}", e);
                        None
                    }
                }
            }
        })
        .as_ref()
}

/// Convert WOFF2 data to a `FontVec`, falling back to direct TTF parsing.
fn convert_woff2(data: Vec<u8>) -> Option<FontVec> {
    if !is_woff2(&data) {
        debug!("nerd-fonts-gtk: label font file is not WOFF2, trying as TTF");
        return FontVec::try_from_vec(data).ok();
    }
    match convert_woff2_to_ttf(&mut std::io::Cursor::new(data)) {
        Ok(ttf_data) => match FontVec::try_from_vec(ttf_data) {
            Ok(font) => Some(font),
            Err(e) => {
                trace!("nerd-fonts-gtk: failed to parse decompressed label font: {}", e);
                None
            }
        },
        Err(e) => {
            error!("nerd-fonts-gtk: failed to decompress WOFF2 label font: {}", e);
            None
        }
    }
}

#[cfg(all(test, feature = "embed-fonts"))]
mod tests {
    use super::*;

    #[test]
    fn nerd_font_loads_from_embedded_data() {
        let font = nerd_font();
        assert!(font.is_some(), "embedded Nerd Font should parse successfully");
    }

    #[test]
    fn label_font_loads_from_embedded_woff2() {
        let font = label_font();
        assert!(font.is_some(), "embedded label font (WOFF2) should decompress and parse successfully");
    }

    #[test]
    fn nerd_font_is_cached() {
        let font1 = nerd_font();
        let font2 = nerd_font();
        assert!(font1.is_some());
        assert!(core::ptr::eq(font1.unwrap() as *const _, font2.unwrap() as *const _));
    }

    #[test]
    fn label_font_is_cached() {
        let font1 = label_font();
        let font2 = label_font();
        assert!(font1.is_some());
        assert!(core::ptr::eq(font1.unwrap() as *const _, font2.unwrap() as *const _));
    }
}
