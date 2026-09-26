//! `FontFamilyConfig` for the Nerd Fonts font family.
//!
//! The config type lives in `nerd-fonts-model` so that the generator
//! and model crates can name [`IconName`](nerd_fonts_model::IconName)
//! (phantom-typed over `NerdFontsConfig`). Re-exported here for
//! backwards compatibility.

pub use nerd_fonts_model::NerdFontsConfig;
