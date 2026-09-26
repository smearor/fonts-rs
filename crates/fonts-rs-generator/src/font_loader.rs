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
///
/// ## With marker type - additionally emits `impl SoftwareRender`
///
/// ```ignore
/// fonts_rs_generator::impl_font_loader!(Code39, "LibreBarcode39-Regular.ttf");
/// fonts_rs_generator::impl_font_loader!(SevenSegment, env: "DSEG7_FONT_PATH");
/// ```
///
/// Emits the `font()` function plus
/// [`SoftwareRender`](::fonts_rs_model::SoftwareRender) with
/// `type Variant = ()` and an inherent `font()` forwarder on the
/// marker, so `Code39::font()` works without a trait import.
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
    // Marker type + literal filename: font() plus `impl SoftwareRender`.
    ($marker:ty, $font_file:literal) => {
        $crate::impl_font_loader!($font_file);

        impl ::fonts_rs_model::SoftwareRender for $marker {
            type Variant = ();

            fn ab_glyph(_variant: Self::Variant) -> Option<&'static ab_glyph::FontVec> {
                font()
            }
        }

        impl $marker {
            /// Returns the cached `ab_glyph` font for the default variant.
            ///
            /// Forwards to [`SoftwareRender::font`](::fonts_rs_model::SoftwareRender::font).
            pub fn font() -> Option<&'static ab_glyph::FontVec> {
                <Self as ::fonts_rs_model::SoftwareRender>::font()
            }
        }
    };
    // Marker type + env var: font() plus `impl SoftwareRender`.
    ($marker:ty, env: $env_var:literal) => {
        $crate::impl_font_loader!(env: $env_var);

        impl ::fonts_rs_model::SoftwareRender for $marker {
            type Variant = ();

            fn ab_glyph(_variant: Self::Variant) -> Option<&'static ab_glyph::FontVec> {
                font()
            }
        }

        impl $marker {
            /// Returns the cached `ab_glyph` font for the default variant.
            ///
            /// Forwards to [`SoftwareRender::font`](::fonts_rs_model::SoftwareRender::font).
            pub fn font() -> Option<&'static ab_glyph::FontVec> {
                <Self as ::fonts_rs_model::SoftwareRender>::font()
            }
        }
    };
}
