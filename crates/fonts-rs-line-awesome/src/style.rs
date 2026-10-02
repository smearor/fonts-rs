//! Style selector for Line Awesome multi-face rendering.
//!
//! Line Awesome ships three upstream TTFs - one face per Font
//! Awesome 5 style. [`LineAwesomeStyle`] selects which face
//! [`SoftwareRender::ab_glyph`](fonts_rs_model::SoftwareRender::ab_glyph)
//! returns and documents the face/weight mapping for `install_pango`.

/// Selects which Line Awesome face is used for software rendering.
///
/// The default is [`LineAwesomeStyle::Solid`] - it carries the full
/// FA5 set (every icon has a solid variant), so `LineAwesome::font()`
/// returns the solid face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LineAwesomeStyle {
    /// Regular face (`la-regular-400.ttf`, weight 400) - the outlined
    /// subset of ~150 icons that share a codepoint with their solid
    /// sibling.
    Regular,
    /// Solid face (`la-solid-900.ttf`, weight 900) - covers the whole
    /// FA5 set. Default.
    #[default]
    Solid,
    /// Brands face (`la-brands-400.ttf`, weight 400) - brand icons.
    Brands,
}
