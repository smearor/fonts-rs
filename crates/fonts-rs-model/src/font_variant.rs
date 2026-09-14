//! Font variant identifier type.
//!
//! Provides a [`FontVariant`] combining a variant name (e.g. `"classic-regular"`)
//! with a [`FontVariantType`] describing
//! how the variant is realized (separate file, axes, or both).

use crate::axis_value::AxisValue;
use crate::axis_value::AxisValues;
use crate::font_file::FontFile;
use crate::font_variant_type::FontVariantType;

/// A font variant, combining a Cargo feature name with its realization.
///
/// The variant name uses kebab-case (e.g. `"classic-regular"`) and corresponds
/// to a Cargo feature of the same name. The [`FontVariantType`] describes
/// whether the variant uses a separate font file, variable font axes, or both.
///
/// # Example
///
/// ```
/// use fonts_rs_model::AxisValue;
/// use fonts_rs_model::FontVariant;
///
/// const VARIANTS: &[FontVariant] = &[
///     FontVariant::file("classic-regular", "DSEG7Classic-Regular"),
///     FontVariant::axes("ultra-light-square", &[
///         AxisValue::new("wght", 100.0),
///         AxisValue::new("ROND", 0.0),
///     ]),
/// ];
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontVariant {
    /// The Cargo feature / variant name (kebab-case, e.g. `"classic-regular"`).
    name: &'static str,
    /// How this variant is realized (file, axes, or both).
    variant_type: FontVariantType,
}

impl FontVariant {
    /// Creates a [`FontVariant`] from a separate font file with no axes.
    pub const fn file(name: &'static str, font_file: &'static str) -> Self {
        Self {
            name,
            variant_type: FontVariantType::file(font_file),
        }
    }

    /// Creates a [`FontVariant`] from axis values in a shared font file.
    pub const fn axes(name: &'static str, axis_values: &'static [AxisValue]) -> Self {
        Self {
            name,
            variant_type: FontVariantType::axes(axis_values),
        }
    }

    /// Creates a [`FontVariant`] from a separate font file with additional axes.
    pub const fn file_with_axes(name: &'static str, font_file: &'static str, axis_values: &'static [AxisValue]) -> Self {
        Self {
            name,
            variant_type: FontVariantType::file_with_axes(font_file, axis_values),
        }
    }

    /// Returns the variant name as a string slice.
    pub fn as_str(&self) -> &'static str {
        self.name
    }

    /// Returns the variant name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the variant slug with hyphens replaced by underscores.
    ///
    /// e.g. `"classic-regular"` → `"classic_regular"`.
    pub fn slug(&self) -> String {
        self.name.replace('-', "_")
    }

    /// Returns the Cargo feature environment variable name for this variant.
    ///
    /// e.g. `"classic-regular"` → `"CARGO_FEATURE_CLASSIC_REGULAR"`.
    pub fn cargo_feature_env_var(&self) -> String {
        format!("CARGO_FEATURE_{}", self.name.replace('-', "_").to_uppercase())
    }

    /// Returns `true` if this variant's Cargo feature is active.
    pub fn is_active(&self) -> bool {
        std::env::var(self.cargo_feature_env_var()).is_ok()
    }

    /// Returns the font file, if any.
    pub fn font_file(&self) -> Option<FontFile> {
        self.variant_type.font_file()
    }

    /// Returns the axis values.
    pub fn axis_values(&self) -> AxisValues {
        self.variant_type.axis_values()
    }

    /// Returns the variant type.
    pub fn variant_type(&self) -> FontVariantType {
        self.variant_type
    }
}

impl AsRef<str> for FontVariant {
    fn as_ref(&self) -> &str {
        self.name
    }
}

impl std::fmt::Display for FontVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}
