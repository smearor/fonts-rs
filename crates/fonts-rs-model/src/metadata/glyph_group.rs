//! Glyph group descriptor for ordered glyph iteration.

use crate::GroupLabel;

/// A glyph group in canonical ordering (e.g. an emoji picker group).
///
/// For emoji fonts, groups and glyph order follow
/// `googlefonts/emoji-metadata` (the same ordering used by Android's
/// emoji picker); skin-tone and gender alternates are inlined after
/// their base emoji. Emitted by build-time codegen via
/// [`GlyphGroup::new`].
#[derive(Debug, Clone, Copy)]
pub struct GlyphGroup {
    /// Display label of the group (e.g. `"Smileys and emotions"`).
    pub name: GroupLabel,
    /// Exported glyph names in picker order.
    pub glyphs: &'static [&'static str],
}

impl GlyphGroup {
    /// Creates a group. Called by the generated `GROUPS` table.
    pub const fn new(name: GroupLabel, glyphs: &'static [&'static str]) -> Self {
        Self { name, glyphs }
    }
}
