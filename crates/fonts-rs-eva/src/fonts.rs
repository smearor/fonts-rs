//! Font loading with `ab_glyph` for software rendering.
//!
//! The Eva webfont is loaded from disk or embedded via the
//! `embed-fonts` feature and cached in `OnceLock` for the process
//! lifetime. Both variants share the same `Eva-Icons.ttf` — the variant
//! only affects which codepoints the generated maps resolve.
//!
//! The font's TTF path is determined at build time by `build.rs`
//! and exposed via the `EVA_FONT_PATH` environment variable.

fonts_rs_generator::impl_font_loader!(crate::naming::Eva, env: "EVA_FONT_PATH");
