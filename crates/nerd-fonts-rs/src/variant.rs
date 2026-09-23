//! Font variant selector for Nerd Fonts software rendering.
//!
//! Nerd Fonts ships two TTFs: the symbols-only icon font and the
//! JetBrains Mono label font. [`NerdFontVariant`] selects which one
//! [`SoftwareRender::ab_glyph`](fonts_rs_model::SoftwareRender::ab_glyph)
//! returns.

/// Selects which Nerd Font TTF is used for software rendering.
///
/// The default is [`NerdFontVariant::Symbols`], so
/// `NerdFonts::font()` returns the symbols-only icon font.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NerdFontVariant {
    /// Symbols-only Nerd Font (`SymbolsNerdFont-Regular.ttf`) — icons.
    #[default]
    Symbols,
    /// Symbols-only Nerd Font Mono (`SymbolsNerdFontMono-Regular.ttf`)
    /// — single-cell icons for terminal-style layouts.
    SymbolsMono,
    /// JetBrains Mono Nerd Font (`JetBrainsMonoNLNerdFont-Regular.woff2`)
    /// — labels with a full character set.
    Label,
}
