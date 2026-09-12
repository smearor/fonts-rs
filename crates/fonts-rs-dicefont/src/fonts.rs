//! Font loading with `ab_glyph` for software rendering.
//!
//! The Dicefont font is loaded from disk or embedded via
//! `embed-fonts` feature and cached in `OnceLock` for the process lifetime.
//!
//! The font's TTF path is determined at build time by `build.rs`
//! and exposed via the `DICEFONT_FONT_PATH` environment variable.

fonts_rs_generator::impl_font_loader!(env: "DICEFONT_FONT_PATH");
