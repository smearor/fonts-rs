//! Font variant type descriptor.
//!
//! Provides [`FontVariantType`] describing how a font variant is realized:
//! via a separate file, via variable font axes, or a combination of both.

use crate::axis_value::AxisValues;
use crate::font_file::FontFile;

/// Describes how a font variant is realized.
///
/// A variant can be:
/// - A separate file (e.g. `DSEG7Classic-Regular.ttf`)
/// - A variation location in a shared file (e.g. `Doto.ttf` at `wght=700, ROND=50`)
/// - A combination of both (a separate file with additional axis settings)
///
/// # Example
///
/// ```
/// use fonts_rs_model::AxisValue;
/// use fonts_rs_model::AxisValues;
/// use fonts_rs_model::FontFile;
/// use fonts_rs_model::FontVariantType;
///
/// // Separate file, no axes
/// const FILE_ONLY: FontVariantType = FontVariantType::file("DSEG7Classic-Regular");
///
/// // Shared file, axes only
/// const AXES_ONLY: FontVariantType = FontVariantType::axes(&[
///     AxisValue::new("wght", 700.0),
///     AxisValue::new("ROND", 50.0),
/// ]);
///
/// // Separate file with axes
/// const COMBINED: FontVariantType = FontVariantType::file_with_axes(
///     "MyFont-Bold",
///     &[AxisValue::new("wdth", 125.0)],
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontVariantType {
    /// The font file for this variant, if separate from the default.
    ///
    /// `None` when the variant uses a shared font file (e.g. a variable font
    /// where the variant is distinguished by axis values alone).
    pub font_file: Option<FontFile>,
    /// Variable font axis values for this variant.
    ///
    /// Empty for non-variable fonts where the variant is distinguished by
    /// file alone.
    pub axis_values: AxisValues,
}

impl FontVariantType {
    /// Creates a [`FontVariantType`] for a separate file with no axes.
    pub const fn file(font_file: &'static str) -> Self {
        Self {
            font_file: Some(FontFile::new(font_file)),
            axis_values: AxisValues::EMPTY,
        }
    }

    /// Creates a [`FontVariantType`] for axis values in a shared file.
    pub const fn axes(axis_values: &'static [crate::axis_value::AxisValue]) -> Self {
        Self {
            font_file: None,
            axis_values: AxisValues::new(axis_values),
        }
    }

    /// Creates a [`FontVariantType`] for a separate file with additional axes.
    pub const fn file_with_axes(font_file: &'static str, axis_values: &'static [crate::axis_value::AxisValue]) -> Self {
        Self {
            font_file: Some(FontFile::new(font_file)),
            axis_values: AxisValues::new(axis_values),
        }
    }

    /// Returns the font file, if any.
    pub fn font_file(&self) -> Option<FontFile> {
        self.font_file
    }

    /// Returns the axis values.
    pub fn axis_values(&self) -> AxisValues {
        self.axis_values
    }
}
