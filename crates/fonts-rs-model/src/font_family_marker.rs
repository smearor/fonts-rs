//! Font family marker trait for compile-time type safety.
//!
//! Each font family crate defines a zero-sized enum implementing
//! [`FontFamilyMarker`], which serves as the phantom type parameter for
//! [`GlyphName<F>`](crate::GlyphName).

/// Sealed trait module for [`FontFamilyMarker`].
///
/// `Sealed` is publicly re-exported (`pub use font_family_marker::sealed;`
/// at crate root), so downstream crates *can* implement
/// `FontFamilyMarker` — intentionally, for application-defined marker
/// types such as custom icon sets built via `fonts-rs-generator`'s
/// `IconSetBuild`. The seal pattern still marks the trait as
/// internal-by-convention: outside of generated code, new marker types
/// should live in the fonts-rs workspace.
pub mod sealed {
    /// Supertrait of [`FontFamilyMarker`](super::FontFamilyMarker).
    ///
    /// Publicly re-exported — external crates may implement it for
    /// application-defined marker types (custom icon sets).
    pub trait Sealed {}
}

/// Marker trait for font family identity.
///
/// Implemented by each font family crate as a zero-sized enum (e.g.
/// `SevenSegment`, `Barcode`). Used as the phantom type parameter in
/// [`GlyphName<F>`](crate::GlyphName) to ensure type safety between font
/// families at compile time.
///
/// Application crates may also implement it for generated icon-set
/// markers (see `fonts-rs-generator`'s `IconSetBuild`) — the public
/// `sealed` re-export makes that possible.
///
/// For the runtime API surface (glyph maps, capability traits), see
/// [`FontFamily`](crate::FontFamily).
pub trait FontFamilyMarker: sealed::Sealed {}
