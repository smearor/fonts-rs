//! Optional canonical ordering contract for font families.
//!
//! [`GlyphOrdering`] is the generic equivalent of the emoji picker
//! ordering: any font family with a curated glyph order (emoji picker
//! groups, icon set sections, ...) implements it on top of its
//! build-time generated tables. Consumers - e.g. the cheat sheet -
//! obtain it via `CheatSheetFamily::ordering()` and stay agnostic of
//! whether the family is an emoji font.
//!
//! # Examples
//!
//! ```ignore
//! impl GlyphOrdering for BlobmojiMetadata {
//!     fn order_index(&self, name: &str) -> Option<u32> {
//!         ORDERING.get(name).copied()
//!     }
//!     // ...
//! }
//! ```

use crate::GlyphGroup;
use crate::GroupLabel;

/// Canonical ordering and grouping contract for a font family.
///
/// All data is build-time generated and returned as `'static`
/// references - implementations are meant to be backed by `phf` maps
/// and static slices, so the trait is object-safe and usable as
/// `&'static dyn GlyphOrdering`.
pub trait GlyphOrdering: Sync {
    /// Returns the canonical ordering position of a base glyph.
    ///
    /// Variants (alternates) and unordered glyphs return `None`;
    /// consumers wanting variant-sort should consult
    /// [`GlyphOrdering::base_of`] for a fallback.
    fn order_index(&self, name: &str) -> Option<u32>;

    /// Returns the group label for a glyph, including variants.
    fn group(&self, name: &str) -> Option<GroupLabel>;

    /// Returns the group labels in canonical order.
    fn group_labels(&self) -> &'static [GroupLabel];

    /// Returns the groups in canonical order.
    ///
    /// Each [`GlyphGroup`] lists glyph names in picker order with
    /// variants inlined after their base - the full layout for UIs
    /// that want a native picker view.
    fn groups(&self) -> &'static [GlyphGroup];

    /// Returns the variant glyph names of a base glyph (skin tones,
    /// gender combos, ...), in canonical order. Empty for glyphs
    /// without variants.
    fn alternates(&self, _name: &str) -> &'static [&'static str] {
        &[]
    }

    /// Returns the base glyph name for a variant (e.g. a skin-tone
    /// sequence), or `None` if `name` is a base glyph.
    fn base_of(&self, name: &str) -> Option<&'static str>;

    /// Whether consumers may collapse variants behind their base
    /// glyph. Defaults to `true` - families with alternates usually
    /// want the compact picker presentation.
    fn collapse_variants(&self) -> bool {
        true
    }
}
