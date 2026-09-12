//! Font loading with `ab_glyph` for software rendering.
//!
//! The Code 128 font is loaded from disk or embedded via
//! `embed-fonts` feature and cached in `OnceLock` for the process lifetime.

fonts_rs_generator::impl_font_loader!("LibreBarcode128-Regular.ttf");
