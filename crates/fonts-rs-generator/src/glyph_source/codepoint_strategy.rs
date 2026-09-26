//! Codepoint assignment strategies for directory-based glyph sources.

/// How `SvgDirSource` assigns `GlyphDescriptor::code` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodepointStrategy {
    /// Use the codepoint sequence from the filename parser
    /// (`emoji_u*` sources like Blobmoji). Multi-codepoint sequences
    /// get a stable PUA codepoint via hash-of-name.
    FromFilename,
    /// Look up the codepoint in the name→codepoint map (upstream
    /// codepoint tables, e.g. Tabler's `.ti-*` CSS).
    FromNameMap,
    /// Allocate a stable PUA codepoint via deterministic hash-of-name
    /// (fontless icon sets).
    HashedPua,
    /// No codepoints - icon-theme-only usage; `GtkIconTheme` resolves
    /// names → SVG directly.
    #[default]
    None,
}
