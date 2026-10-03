//! Deterministic Private Use Area codepoint allocation.

use crate::glyph_source::error::GlyphSourceError;
use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::PUA_B_RANGE;
use fonts_rs_model::PUA_RANGE;
use std::collections::HashSet;

/// Allocates stable Private Use Area codepoints via deterministic
/// hash-of-name with linear probing on collision.
///
/// The default range is the BMP PUA (U+E000–U+F8FF, 6,400 slots);
/// [`with_range`](Self::with_range) or [`pua_b`](Self::pua_b) select
/// other arenas - e.g. Supplementary PUA-B for extra styles of a
/// [`CombinedSource`](crate::glyph_source::CombinedSource), so their
/// codepoints stay stable even when the base style's map grows.
///
/// An icon keeps its codepoint across upstream additions/removals -
/// persisted references (favorites, recents) never silently break.
///
/// Callers must feed names in a deterministic order (e.g. sorted
/// filenames): on hash collision, the first-processed name wins the
/// primary slot, so allocation order determines the result.
#[derive(Debug)]
pub struct PuaAllocator {
    /// Codepoints already assigned.
    occupied: HashSet<CodePoint>,
    /// Allocatable range (inclusive).
    range: CodePointRange,
    /// Number of allocatable slots in `range`.
    len: u32,
}

impl Default for PuaAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl PuaAllocator {
    /// Creates an empty allocator over the BMP PUA (U+E000–U+F8FF).
    pub fn new() -> Self {
        Self::from_range(PUA_RANGE)
    }

    /// Creates an empty allocator over Supplementary PUA-B
    /// (U+100000–U+10FFFD).
    ///
    /// PUA-B is disjoint from both the BMP PUA and PUA-A (U+F0000–
    /// U+FFFFD), so style variants hashed here can never collide with
    /// a base style's real or hashed codepoints - the allocation stays
    /// stable no matter how the base map grows.
    pub fn pua_b() -> Self {
        Self::from_range(PUA_B_RANGE)
    }

    /// Creates an empty allocator over `range` (inclusive), or `None`
    /// when `range` is empty (start after end).
    ///
    /// Non-scalar codepoints (the surrogate range U+D800–U+DFFF) are
    /// skipped during probing - they never occupy a slot.
    pub fn with_range(range: CodePointRange) -> Option<Self> {
        (range.end().as_char() >= range.start().as_char()).then(|| Self::from_range(range))
    }

    /// Infallible construction for callers with a known non-empty
    /// range (the built-in arenas are compile-time constants).
    fn from_range(range: CodePointRange) -> Self {
        let start = range.start().as_char() as u32;
        let end = range.end().as_char() as u32;
        Self {
            occupied: HashSet::new(),
            range,
            len: end.saturating_sub(start) + 1,
        }
    }

    /// Returns the deterministic codepoint for `name`.
    ///
    /// Each name is allocated exactly once per export run - glyph names
    /// are unique within a family.
    ///
    /// # Errors
    ///
    /// Returns [`GlyphSourceError::PuaExhausted`] when all slots are
    /// occupied.
    pub fn allocate(&mut self, name: &str) -> Result<CodePoint, GlyphSourceError> {
        let hash = Self::hash(name.as_bytes());
        let start_slot = (hash % u64::from(self.len)) as u32;
        let start = self.range.start().as_char() as u32;
        for offset in 0..self.len {
            let candidate = start + (start_slot + offset) % self.len;
            // Non-scalar candidates (surrogates) are not codepoints -
            // skip without occupying the slot.
            let Ok(candidate) = CodePoint::try_from(candidate) else {
                continue;
            };
            if self.occupied.insert(candidate) {
                return Ok(candidate);
            }
        }
        Err(GlyphSourceError::PuaExhausted)
    }

    /// FNV-1a hash - stable across Rust versions, unlike `DefaultHasher`,
    /// so generated codepoints are reproducible across builds.
    fn hash(bytes: &[u8]) -> u64 {
        bytes
            .iter()
            .fold(0xcbf29ce484222325, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocation_is_deterministic() {
        let mut first = PuaAllocator::new();
        let mut second = PuaAllocator::new();
        for name in ["home", "settings", "arrow-left"] {
            assert_eq!(first.allocate(name).ok(), second.allocate(name).ok());
        }
    }

    #[test]
    fn allocated_codepoints_are_in_pua_range() {
        let mut allocator = PuaAllocator::new();
        let codepoint = allocator.allocate("home").expect("codepoint");
        assert!(('\u{E000}'..='\u{F8FF}').contains(&codepoint.as_char()));
    }

    #[test]
    fn custom_range_allocation_stays_in_range() {
        let mut allocator = PuaAllocator::with_range(CodePointRange::from_char('\u{F0000}', '\u{F00FF}')).expect("range non-empty");
        for index in 0..100 {
            let codepoint = allocator.allocate(&format!("icon-{index}")).expect("codepoint");
            assert!(('\u{F0000}'..='\u{F00FF}').contains(&codepoint.as_char()));
        }
    }

    /// A range spanning the surrogate gap still yields its valid
    /// slots; surrogates are skipped instead of exhausting the range.
    /// U+D7FF–U+E001 has exactly three scalar values.
    #[test]
    fn allocation_skips_surrogate_slots() {
        let mut allocator = PuaAllocator::with_range(CodePointRange::from_char('\u{D7FF}', '\u{E001}')).expect("range non-empty");
        for name in ["a", "b", "c"] {
            let codepoint = allocator.allocate(name).expect("codepoint");
            assert!(!(0xD800..=0xDFFF).contains(&(codepoint.as_char() as u32)));
        }
        // Only U+D7FF, U+E000 and U+E001 are scalar values - now
        // exhausted.
        assert!(matches!(allocator.allocate("d"), Err(GlyphSourceError::PuaExhausted)));
    }

    /// An inverted range yields `None` - the caller decides how to
    /// report the misconfiguration, no panic.
    #[test]
    fn empty_range_returns_none() {
        let allocator = PuaAllocator::with_range(CodePointRange::from_char('\u{E001}', '\u{E000}'));
        assert!(allocator.is_none());
    }

    #[test]
    fn distinct_names_get_distinct_codepoints() {
        let mut allocator = PuaAllocator::new();
        let mut codepoints = HashSet::new();
        for index in 0..1000 {
            let codepoint = allocator.allocate(&format!("icon-{index}")).expect("codepoint");
            assert!(codepoints.insert(codepoint.as_char()));
        }
    }
}
