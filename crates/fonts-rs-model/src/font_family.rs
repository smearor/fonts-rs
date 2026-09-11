//! Font family marker trait for compile-time type safety.
//!
//! Each font family crate defines a zero-sized enum implementing [`FontFamily`],
//! which serves as the phantom type parameter for [`GlyphName<F>`](crate::GlyphName).

/// Sealed trait module preventing external implementations of [`FontFamily`].
///
/// Only crates within the workspace can implement `FontFamily` by also
/// implementing `Sealed`, which is private to this module.
pub mod sealed {
    /// Private supertrait that prevents external crates from implementing
    /// [`FontFamily`](super::FontFamily).
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
/// `FontFamily`, ensuring that only crates within the workspace can
/// define font family markers.
pub trait FontFamily: sealed::Sealed {}
