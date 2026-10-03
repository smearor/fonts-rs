//! Classification of a font or icon family shown in the detail sidebar.

use std::fmt::Display;
use std::fmt::Formatter;

/// What kind of glyph source a cheat-sheet family ships.
///
/// Displayed in the "Family" section of the detail sidebar so users
/// can tell e.g. a general-purpose icon set apart from an emoji font
/// or a niche special font.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilyKind {
    /// A general-purpose text font.
    Font,
    /// An icon font aggregating many icon sets in one typeface (e.g.
    /// Nerd Fonts Symbols).
    IconFont,
    /// A curated set of icons from a single design system (e.g.
    /// Lucide, Heroicons, Fluent).
    IconSet,
    /// An emoji font (e.g. Noto Emoji, Blobmoji).
    Emoji,
    /// A special-purpose font that does not fit the other categories
    /// (e.g. barcode, segmented displays, redacted).
    SpecialFont,
}

impl FamilyKind {
    /// The human-readable label used by [`Display`].
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Font => "Font",
            Self::IconFont => "Icon Font",
            Self::IconSet => "Icon Set",
            Self::Emoji => "Emoji",
            Self::SpecialFont => "Special Font",
        }
    }
}

impl Display for FamilyKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
