//! Macro for generating a cached font loader function.
//!
//! See `impl_font_loader` macro.

/// Generates a `font() -> Option<&'static ab_glyph::FontVec>` function
/// that loads a TTF font from embedded bytes (`embed-fonts` feature)
/// or from disk.
///
/// The font is cached in a `OnceLock` for the process lifetime.
///
/// # Variants
///
/// ## Literal filename (relative to `resources/`)
///
/// ```ignore
/// fonts_rs_generator::impl_font_loader!("LibreBarcode128-Regular.ttf");
/// ```
///
/// ## Env var path (absolute path set by `build.rs`)
///
/// ```ignore
/// fonts_rs_generator::impl_font_loader!(env: "DSEG7_FONT_PATH");
/// ```
#[macro_export]
macro_rules! impl_font_loader {
    // Literal filename: resolved relative to `resources/` directory.
    ($font_file:literal) => {
        /// Font file name (relative to `resources/`).
        const FONT_FILE: &str = $font_file;

        static FONT: std::sync::OnceLock<Option<ab_glyph::FontVec>> = std::sync::OnceLock::new();

        /// Get the cached font, loading from disk or embedded on first access.
        pub fn font() -> Option<&'static ab_glyph::FontVec> {
            FONT.get_or_init(|| {
                #[cfg(feature = "embed-fonts")]
                {
                    let data = include_bytes!(concat!("../resources/", $font_file)).to_vec();
                    ab_glyph::FontVec::try_from_vec(data).ok()
                }
                #[cfg(not(feature = "embed-fonts"))]
                {
                    std::env::current_dir()
                        .ok()
                        .map(|d| d.join("resources").join(FONT_FILE))
                        .and_then(|p| std::fs::read(p).ok())
                        .and_then(|data| ab_glyph::FontVec::try_from_vec(data).ok())
                }
            })
            .as_ref()
        }
    };
    // Env var: absolute path set by build.rs via `cargo:rustc-env`.
    (env: $env_var:literal) => {
        static FONT: std::sync::OnceLock<Option<ab_glyph::FontVec>> = std::sync::OnceLock::new();

        /// Get the cached font, loading from disk or embedded on first access.
        pub fn font() -> Option<&'static ab_glyph::FontVec> {
            FONT.get_or_init(|| {
                #[cfg(feature = "embed-fonts")]
                {
                    let data = include_bytes!(env!($env_var)).to_vec();
                    ab_glyph::FontVec::try_from_vec(data).ok()
                }
                #[cfg(not(feature = "embed-fonts"))]
                {
                    std::fs::read(env!($env_var))
                        .ok()
                        .and_then(|data| ab_glyph::FontVec::try_from_vec(data).ok())
                }
            })
            .as_ref()
        }
    };
}
