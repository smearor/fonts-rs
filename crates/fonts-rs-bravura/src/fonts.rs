//! Font loading with `ab_glyph` for software rendering.
//!
//! The Bravura font is loaded from disk or embedded via
//! `embed-fonts` feature and cached in `OnceLock` for the process lifetime.
//!
//! The font's OTF path is determined at build time by `build.rs`
//! and exposed via the `BRAVURA_FONT_PATH` environment variable.

fonts_rs_generator::impl_font_loader!(env: "BRAVURA_FONT_PATH");
