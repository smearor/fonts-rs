//! Extension trait for [`FontVariant`] with build-time helpers.

use fonts_rs_model::FontVariant;

use crate::build_constants::RESOURCES_DIR;

/// Build-time extension methods for [`FontVariant`].
pub trait FontVariantExt {
    /// Returns the full font file path relative to the crate root.
    ///
    /// Constructs `{RESOURCES_DIR}/{font_file}.ttf` from the variant's
    /// [`FontVariant::font_file`]. Returns an error if the variant has no
    /// associated font file (e.g. axis-only variants in a shared file).
    ///
    /// # Errors
    ///
    /// Returns a `miette` error if [`FontVariant::font_file`] is `None`.
    fn font_path(&self) -> miette::Result<String>;
}

impl FontVariantExt for FontVariant {
    fn font_path(&self) -> miette::Result<String> {
        let font_file = self.font_file().ok_or_else(|| miette::miette!("variant '{}' has no font file", self.as_str()))?;
        Ok(format!("{}/{}.ttf", RESOURCES_DIR, font_file.as_str()))
    }
}
