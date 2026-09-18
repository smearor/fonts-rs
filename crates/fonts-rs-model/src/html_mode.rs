//! Export mode for the HTML builder.

/// Export mode for [`HtmlBuilder`](crate::HtmlBuilder).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HtmlMode {
    /// `@font-face` rule with base64-embedded TTF plus a sample text
    /// styled by a CSS class.
    #[default]
    FontFace,
    /// Glyph grid of inline `<svg>` elements — no font required.
    InlineSvg,
}
