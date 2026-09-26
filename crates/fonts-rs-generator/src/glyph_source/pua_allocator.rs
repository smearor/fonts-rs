//! Deterministic Private Use Area codepoint allocation.

use crate::glyph_source::error::GlyphSourceError;
use fonts_rs_model::CodePoint;
use std::collections::HashSet;

/// Allocates stable Private Use Area codepoints (U+E000–U+F8FF) via
/// deterministic hash-of-name with linear probing on collision.
///
/// An icon keeps its codepoint across upstream additions/removals -
/// persisted references (favorites, recents) never silently break.
///
/// Callers must feed names in a deterministic order (e.g. sorted
/// filenames): on hash collision, the first-processed name wins the
/// primary slot, so allocation order determines the result.
#[derive(Debug, Default)]
pub struct PuaAllocator {
    /// Codepoints already assigned.
    occupied: HashSet<u32>,
}

impl PuaAllocator {
    /// First PUA codepoint (inclusive).
    const PUA_START: u32 = 0xE000;
    /// Number of PUA slots (U+E000–U+F8FF = 6,400).
    const PUA_LEN: u32 = 0xF8FF - Self::PUA_START + 1;

    /// Creates an empty allocator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the deterministic PUA codepoint for `name`.
    ///
    /// Each name is allocated exactly once per export run - glyph names
    /// are unique within a family.
    ///
    /// # Errors
    ///
    /// Returns [`GlyphSourceError::PuaExhausted`] when all PUA slots are
    /// occupied.
    pub fn allocate(&mut self, name: &str) -> Result<CodePoint, GlyphSourceError> {
        let hash = Self::hash(name.as_bytes());
        let start_slot = (hash % u64::from(Self::PUA_LEN)) as u32;
        for offset in 0..Self::PUA_LEN {
            let candidate = Self::PUA_START + (start_slot + offset) % Self::PUA_LEN;
            if self.occupied.insert(candidate) {
                return char::from_u32(candidate).map(CodePoint::from_char).ok_or(GlyphSourceError::PuaExhausted);
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
    fn distinct_names_get_distinct_codepoints() {
        let mut allocator = PuaAllocator::new();
        let mut codepoints = HashSet::new();
        for index in 0..1000 {
            let codepoint = allocator.allocate(&format!("icon-{index}")).expect("codepoint");
            assert!(codepoints.insert(codepoint.as_char()));
        }
    }
}
