//! Font family marker trait for compile-time type safety.
//!
//! Each font family crate defines a zero-sized enum implementing
//! [`FontFamilyMarker`], which serves as the phantom type parameter for
//! [`GlyphName<F>`](crate::GlyphName).

/// Sealed trait module preventing external implementations of
/// [`FontFamilyMarker`].
///
/// Only crates within the workspace can implement `FontFamilyMarker` by
/// also implementing `Sealed`, which is private to this module.
pub mod sealed {
    /// Private supertrait that prevents external crates from implementing
    /// [`FontFamilyMarker`](super::FontFamilyMarker).
    pub trait Sealed {}
}

/// Marker trait for font family identity.
///
/// Implemented by each font family crate as a zero-sized enum (e.g.
/// `SevenSegment`, `Barcode`). Used as the phantom type parameter in
/// [`GlyphName<F>`](crate::GlyphName) to ensure type safety between font
/// families at compile time.
///
/// The `Sealed` supertrait prevents external crates from implementing
/// `FontFamilyMarker`, ensuring that only crates within the workspace can
/// define font family markers.
///
/// For the runtime API surface (glyph maps, capability traits), see
/// [`FontFamily`](crate::FontFamily).
pub trait FontFamilyMarker: sealed::Sealed {}
