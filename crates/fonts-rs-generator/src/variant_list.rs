//! Typed list of font variants, generic over a [`FontFamilyConfig`].

use std::marker::PhantomData;

use fonts_rs_model::FontVariant;

use crate::font_family_config::FontFamilyConfig;

/// A typed list of font variants, generic over a [`FontFamilyConfig`].
///
/// Wraps a `&[FontVariant]` slice and carries the font family type `X` so
/// that [`VariantList::detect_active_index`] can use `X::FAMILY_DISPLAY_NAME`
/// for diagnostic messages without passing it as a parameter.
///
/// # Example
///
/// ```ignore
/// const VARIANTS: VariantList<SevenSegmentConfig> = VariantList::new(&[
///     FontVariant::file("classic-regular", "DSEG7Classic-Regular"),
///     // ...
/// ]);
///
/// let idx = VARIANTS.detect_active_index(0)?;
/// ```
pub struct VariantList<'a, X: FontFamilyConfig> {
    /// The variant slice.
    variants: &'a [FontVariant],
    /// Marker for the font family config type.
    _marker: PhantomData<X>,
}

impl<'a, X: FontFamilyConfig> VariantList<'a, X> {
    /// Create a typed variant list from a `&[FontVariant]` slice.
    pub const fn new(variants: &'a [FontVariant]) -> Self {
        Self {
            variants,
            _marker: PhantomData,
        }
    }

    /// Returns the underlying variant slice.
    pub fn as_slice(&self) -> &'a [FontVariant] {
        self.variants
    }

    /// Detect the active variant from Cargo features.
    ///
    /// Scans `CARGO_FEATURE_{NAME}` environment variables for each variant and
    /// returns the index of the active one. If no variant is active, returns
    /// `default_index`. If more than one is active, falls back to `default_index`
    /// with a warning — this allows `--all-features` to work in CI without
    /// breaking on mutually exclusive variant features.
    ///
    /// Uses `X::FAMILY_DISPLAY_NAME` for diagnostic messages.
    pub fn detect_active_index(&self, default_index: usize) -> miette::Result<usize> {
        let family_display_name = X::FAMILY_DISPLAY_NAME;
        let variants = self.variants;
        let active: Vec<usize> = variants.iter().enumerate().filter(|(_, v)| v.is_active()).map(|(i, _)| i).collect();

        if active.is_empty() {
            eprintln!(
                "build.rs: no {family_display_name} variant feature active, defaulting to {}",
                variants[default_index].as_str()
            );
            Ok(default_index)
        } else if active.len() > 1 {
            eprintln!(
                "build.rs: multiple {family_display_name} variant features active, falling back to default: {}",
                variants[default_index].as_str()
            );
            for &i in &active {
                eprintln!("  active: {}", variants[i].as_str());
            }
            Ok(default_index)
        } else {
            Ok(active[0])
        }
    }

    /// Detect the active variant and return it directly.
    ///
    /// Convenience wrapper around [`detect_active_index`](Self::detect_active_index)
    /// that returns the [`FontVariant`] reference instead of the index.
    pub fn detect_and_get_active_variant(&self, default_index: usize) -> miette::Result<&'a FontVariant> {
        let idx = self.detect_active_index(default_index)?;
        Ok(&self.variants[idx])
    }
}
