//! Font loading with `ab_glyph` for software rendering.
//!
//! The Code 39 font is loaded from disk or embedded via
//! `embed-fonts` feature and cached in `OnceLock` for the process lifetime.

use crate::Code39;

fonts_rs_generator::impl_font_loader!(Code39, "LibreBarcode39-Regular.ttf");
