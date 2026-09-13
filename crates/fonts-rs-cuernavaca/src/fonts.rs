//! Font loading with `ab_glyph` for software rendering.
//!
//! The Cuernavaca font is loaded from disk or embedded via
//! `embed-fonts` feature and cached in `OnceLock` for the process lifetime.
//!
//! The font path is determined at build time by `build.rs`
//! and exposed via the `CUERNAVACA_FONT_PATH` environment variable.

fonts_rs_generator::impl_font_loader!(env: "CUERNAVACA_FONT_PATH");
