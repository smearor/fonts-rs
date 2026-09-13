//! Unicode codepoint range newtype for type-safe range handling.

use crate::CodePoint;

/// A contiguous Unicode codepoint range from `start` to `end` (inclusive).
///
/// Used by font exporters to define which Unicode codepoint ranges
/// to probe when building reverse glyph maps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodePointRange {
    /// The start of the range (inclusive).
    start: CodePoint,
    /// The end of the range (inclusive).
    end: CodePoint,
}

impl CodePointRange {
    /// Creates a new codepoint range from `start` to `end` (inclusive).
    pub const fn new(start: CodePoint, end: CodePoint) -> Self {
        Self { start, end }
    }

    /// Creates a new codepoint range from `char` values.
    ///
    /// Convenience constructor for `const` contexts, avoiding the need to
    /// wrap each `char` in `CodePoint::from_char` manually.
    pub const fn from_char(start: char, end: char) -> Self {
        Self {
            start: CodePoint::from_char(start),
            end: CodePoint::from_char(end),
        }
    }

    /// Returns the start of the range (inclusive).
    pub const fn start(&self) -> CodePoint {
        self.start
    }

    /// Returns the end of the range (inclusive).
    pub const fn end(&self) -> CodePoint {
        self.end
    }
}

/// ASCII printable range: U+0020 (space) through U+007E (tilde).
pub const ASCII_PRINTABLE_RANGE: &[CodePointRange] = &[CodePointRange::from_char('\u{0020}', '\u{007E}')];

/// Basic Multilingual Plane: U+0000 through U+FFFF.
pub const BMP_RANGE: CodePointRange = CodePointRange::from_char('\u{0000}', '\u{FFFF}');

/// Private Use Area (BMP): U+E000 through U+F8FF.
pub const PUA_RANGE: CodePointRange = CodePointRange::from_char('\u{E000}', '\u{F8FF}');

/// Supplementary Private Use Area: U+F0001 through U+10FFFF.
pub const SUPPLEMENTARY_PUA_RANGE: CodePointRange = CodePointRange::from_char('\u{F0001}', '\u{10FFFF}');
