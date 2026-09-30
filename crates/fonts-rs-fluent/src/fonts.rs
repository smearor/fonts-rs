//! Font loader for Fluent UI System Icons.
//!
//! The active variant's TTF path is determined at build time by
//! `build.rs` and exposed via the `FLUENT_FONT_PATH` environment
//! variable for pixel-drawing (ab_glyph).

fonts_rs_generator::impl_font_loader!(crate::naming::Fluent, env: "FLUENT_FONT_PATH");
