//! Font loading with `ab_glyph` for software rendering.
//!
//! The EAN13 font is loaded from disk or embedded via
//! `embed-fonts` feature and cached in `OnceLock` for the process lifetime.

fonts_rs_generator::impl_font_loader!("LibreBarcodeEAN13Text-Regular.ttf");
