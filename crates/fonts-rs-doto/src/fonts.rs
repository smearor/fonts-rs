//! Font loading with `ab_glyph` for software rendering.
//!
//! The Doto font is loaded from disk or embedded via
//! `embed-fonts` feature and cached in `OnceLock` for the process lifetime.
//!
//! The active variant's TTF path is determined at build time by `build.rs`
//! and exposed via the `DOTO_FONT_PATH` environment variable.

fonts_rs_generator::impl_font_loader!(env: "DOTO_FONT_PATH");
