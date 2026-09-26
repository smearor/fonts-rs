//! GTK font registration macro.
//!
//! Provides the [`impl_gtk_font!`] macro for generating
//! [`GtkFont`](::fonts_rs_model::GtkFont) implementations on font family
//! marker types.
//!
//! The `GtkFont` trait and its error types live in `fonts-rs-model`.
//! Each font crate calls `impl_gtk_font!` in a `src/gtk.rs` file to
//! generate the `GtkFont` impl for its marker type (e.g. `Ean13`,
//! `Code39`, `Bravura`).

/// Generates a [`GtkFont`](::fonts_rs_model::GtkFont) implementation for a font family marker type.
///
/// # Variants
///
/// ## Icons only (no TTF GResource)
///
/// ```ignore
/// fonts_rs_generator::impl_gtk_font!(Bravura);
/// ```
///
/// Generates `register_glyphs()` (registers `icons.gresource`).
/// `register_font()` is a no-op, `load_font()` returns an error.
///
/// ## With TTF GResource
///
/// ```ignore
/// fonts_rs_generator::impl_gtk_font!(Ean13, with_font_gresource);
/// ```
///
/// Generates all three methods. The build script must have written
/// `OUT_DIR/font_resource.rs` containing:
///
/// ```ignore
/// pub const FONT_RESOURCE_PATH: &str = "/io/smearor/fonts/barcode_ean13/LibreBarcodeEAN13Text-Regular.ttf";
/// ```
#[macro_export]
macro_rules! impl_gtk_font {
    // Icons only - no font.gresource
    ($marker:ty) => {
        impl ::fonts_rs_model::GtkFont for $marker {
            fn register_glyphs() -> Result<(), ::fonts_rs_model::RegisterGlyphError> {
                ::gio::resources_register_include!("icons.gresource")?;
                Ok(())
            }

            fn register_font() -> Result<(), ::fonts_rs_model::RegisterFontError> {
                Ok(())
            }

            fn load_font() -> Result<&'static std::path::Path, ::fonts_rs_model::LoadFontError> {
                Err(::fonts_rs_model::LoadFontError::NoFontGresource)
            }
        }
    };
    // With font.gresource - includes generated font_resource.rs
    ($marker:ty, with_font_gresource) => {
        include!(concat!(env!("OUT_DIR"), "/font_resource.rs"));

        impl ::fonts_rs_model::GtkFont for $marker {
            fn register_glyphs() -> Result<(), ::fonts_rs_model::RegisterGlyphError> {
                ::gio::resources_register_include!("icons.gresource")?;
                Ok(())
            }

            fn register_font() -> Result<(), ::fonts_rs_model::RegisterFontError> {
                ::gio::resources_register_include!("font.gresource")?;
                Ok(())
            }

            fn load_font() -> Result<&'static std::path::Path, ::fonts_rs_model::LoadFontError> {
                use std::sync::OnceLock;

                static FONT_TEMP_PATH: OnceLock<std::path::PathBuf> = OnceLock::new();

                if let Some(path) = FONT_TEMP_PATH.get() {
                    return Ok(path);
                }

                <Self as ::fonts_rs_model::GtkFont>::register_font()?;
                let bytes = ::gio::resources_lookup_data(FONT_RESOURCE_PATH, ::gio::ResourceLookupFlags::NONE)?;
                let temp_path = std::env::temp_dir().join(format!("{}-{}.ttf", env!("CARGO_PKG_NAME"), std::process::id()));
                std::fs::write(&temp_path, bytes.as_ref())?;

                let path = FONT_TEMP_PATH.get_or_init(|| temp_path);
                Ok(path)
            }
        }
    };
}
