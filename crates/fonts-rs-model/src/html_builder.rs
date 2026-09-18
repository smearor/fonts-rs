//! Fluent builder for self-contained HTML export pages.
//!
//! [`HtmlBuilder`] covers both HTML export modes: `HtmlMode::FontFace`
//! (`@font-face` with embedded TTF) and `HtmlMode::InlineSvg` (glyph
//! grid of inline `<svg>` elements).

use crate::FontError;
use crate::HtmlExport;
use crate::HtmlMode;
use std::marker::PhantomData;
use std::path::Path;

/// Builder for self-contained HTML export pages.
///
/// Created via [`HtmlExport::html`]; covers both HTML examples:
/// `HtmlMode::FontFace` (the `css_class_html` example) and
/// `HtmlMode::InlineSvg` (the `svg_html` example).
///
/// `F` is a ZST marker — the builder stores no `F` value, so
/// `PhantomData<fn() -> F>` keeps the parameter used (covariant, no
/// ownership/drop semantics).
pub struct HtmlBuilder<F: HtmlExport> {
    /// Page title used for `<title>` and `<h1>`.
    title: Option<String>,
    /// Export mode.
    mode: HtmlMode,
    /// Sample text rendered via the CSS class (FontFace mode).
    sample_text: Option<String>,
    /// CSS class name for the sample text (FontFace mode).
    css_class: Option<String>,
    /// Codepoints to include in the glyph grid (InlineSvg mode).
    glyphs: Option<Vec<char>>,
    /// Covariant marker for the font family type.
    _marker: PhantomData<fn() -> F>,
}

impl<F: HtmlExport> HtmlBuilder<F> {
    /// Creates a builder with defaults: title = `F::FAMILY_DISPLAY_NAME`,
    /// mode = `HtmlMode::FontFace`, css_class = `F::FONT_FAMILY_NAME`
    /// with `_` replaced by `-`, all glyphs for the InlineSvg grid.
    pub(crate) fn new() -> Self {
        Self {
            title: None,
            mode: HtmlMode::default(),
            sample_text: None,
            css_class: None,
            glyphs: None,
            _marker: PhantomData,
        }
    }

    /// Page title used for `<title>` and `<h1>`.
    /// Default: `F::FAMILY_DISPLAY_NAME`.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Export mode. Default: `HtmlMode::FontFace`.
    pub fn mode(mut self, mode: HtmlMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sample text rendered via the CSS class (FontFace mode).
    /// Default: `F::FAMILY_DISPLAY_NAME`.
    pub fn sample_text(mut self, text: impl Into<String>) -> Self {
        self.sample_text = Some(text.into());
        self
    }

    /// CSS class name for the sample text (FontFace mode).
    /// Default: `F::FONT_FAMILY_NAME` with `_` replaced by `-`.
    pub fn css_class(mut self, class: impl Into<String>) -> Self {
        self.css_class = Some(class.into());
        self
    }

    /// Restricts the glyph grid to these codepoints (InlineSvg mode).
    /// Default: all glyphs.
    pub fn glyphs(mut self, codepoints: impl IntoIterator<Item = char>) -> Self {
        self.glyphs = Some(codepoints.into_iter().collect());
        self
    }

    /// Renders the HTML document to a string.
    ///
    /// # Errors
    ///
    /// Returns a [`FontError`] if registration or font loading fails.
    pub fn render(self) -> Result<String, FontError> {
        match self.mode {
            HtmlMode::FontFace => self.render_font_face(),
            HtmlMode::InlineSvg => self.render_inline_svg(),
        }
    }

    /// Renders and writes the document to `path`.
    ///
    /// # Errors
    ///
    /// Returns a [`FontError`] if rendering or writing fails.
    pub fn write(self, path: impl AsRef<Path>) -> Result<(), FontError> {
        let html = self.render()?;
        std::fs::write(path, html)?;
        Ok(())
    }

    /// Resolved page title.
    fn page_title(&self) -> String {
        self.title.clone().unwrap_or_else(|| F::FAMILY_DISPLAY_NAME.to_string())
    }

    /// Renders the `@font-face` page: embedded TTF plus a sample text
    /// styled by a CSS class.
    fn render_font_face(&self) -> Result<String, FontError> {
        let font_face = F::css_font_face()?;
        let title = self.page_title();
        let font_family = F::CSS_FONT_FAMILY;
        let css_class = self.css_class.clone().unwrap_or_else(|| F::FONT_FAMILY_NAME.replace('_', "-"));
        let sample_text = self.sample_text.clone().unwrap_or_else(|| F::FAMILY_DISPLAY_NAME.to_string());

        Ok(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{title}</title>
<style>
{font_face}
.{css_class} {{
    font-family: '{font_family}';
    font-size: 192px;
    text-align: center;
    margin: 2rem 0;
}}
body {{
    font-family: sans-serif;
    max-width: 700px;
    margin: 2rem auto;
    padding: 0 1rem;
}}
</style>
</head>
<body>
<h1>{title}</h1>
<p>The text below is rendered by the browser using <code>@font-face</code>
with the TTF embedded as a base64 data URI from the GResource bundle.</p>
<div class="{css_class}">{sample_text}</div>
</body>
</html>
"#
        ))
    }

    /// Renders the inline-SVG page: a glyph grid of `<svg>` elements,
    /// no font required.
    fn render_inline_svg(&self) -> Result<String, FontError> {
        F::ensure_registered()?;
        let title = self.page_title();

        let codepoints: Vec<char> = match &self.glyphs {
            Some(glyphs) => glyphs.clone(),
            None => F::all_glyphs().map(|(ch, _)| ch).collect(),
        };

        let mut svg_grid = String::new();
        let mut count = 0_usize;
        for ch in codepoints {
            let Some(name) = F::glyph_name(ch) else {
                continue;
            };
            let Some(svg) = F::glyph_svg(name) else {
                continue;
            };
            let svg = svg.replace("currentColor", "#333");
            svg_grid.push_str(&format!(
                r#"<div class="glyph-cell"><div class="glyph-svg">{svg}</div><div class="glyph-name">{name}</div></div>"#
            ));
            count += 1;
        }

        Ok(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{title}</title>
<style>
body {{
    font-family: sans-serif;
    max-width: 900px;
    margin: 2rem auto;
    padding: 0 1rem;
}}
h1 {{
    text-align: center;
}}
.glyph-grid {{
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
    gap: 12px;
    margin-top: 2rem;
}}
.glyph-cell {{
    text-align: center;
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
}}
.glyph-svg {{
    width: 48px;
    height: 48px;
    margin: 0 auto;
}}
.glyph-svg svg {{
    width: 100%;
    height: 100%;
}}
.glyph-name {{
    font-size: 10px;
    color: #666;
    margin-top: 4px;
    word-break: break-all;
}}
</style>
</head>
<body>
<h1>{title}</h1>
<p>Each glyph below is an inline SVG loaded from the GResource bundle.
No TTF font or <code>@font-face</code> is required — the SVGs are
pre-rendered vector outlines embedded directly in the HTML.</p>
<div class="glyph-grid">
{svg_grid}
</div>
<p>{count} glyphs.</p>
</body>
</html>
"#
        ))
    }
}
