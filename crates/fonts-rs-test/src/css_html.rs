//! CSS class HTML example.
//!
//! Provides [`run_css_class_html`] for generating an HTML page with
//! `@font-face` and CSS class names, embedding the TTF as a base64 data URI.

use base64::Engine;
use fonts_rs_model::GtkFont;
use miette::IntoDiagnostic;
use miette::Result;

/// Generates an HTML page with `@font-face` and a CSS class.
///
/// Registers the font's GResource, loads the TTF, embeds it as a base64
/// data URI, and writes an HTML file to the system temp directory.
///
/// # Type Parameters
///
/// - `T`: Font marker type implementing [`GtkFont`].
///
/// # Parameters
///
/// - `font_family`: CSS `font-family` name.
/// - `css_class`: CSS class name for the sample text.
/// - `content_label`: Content type label (e.g. `"Emoji"`, `"EAN-Barcode"`).
/// - `sample_text`: Text to render with the font.
/// - `html_title`: Title for the HTML page.
/// - `output_filename`: Filename for the output HTML file (in temp dir).
///
/// # Errors
///
/// Returns an error if GResource registration, font loading, or file writing fails.
pub fn run_css_class_html<T: GtkFont>(
    font_family: &str,
    css_class: &str,
    content_label: &str,
    sample_text: &str,
    html_title: &str,
    output_filename: &str,
) -> Result<()> {
    T::register_glyphs().into_diagnostic()?;
    T::register_font().into_diagnostic()?;
    let font_path = T::load_font().into_diagnostic()?;

    let font_bytes = std::fs::read(font_path).into_diagnostic()?;
    let font_b64 = base64::engine::general_purpose::STANDARD.encode(&font_bytes);
    let font_data_uri = format!("data:font/ttf;base64,{font_b64}");

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{html_title}</title>
<style>
@font-face {{
    font-family: '{font_family}';
    src: url('{font_data_uri}') format('truetype');
}}
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
<h1>{html_title}</h1>
<p>The {content_label} below is rendered by the browser using <code>@font-face</code>
with the TTF embedded as a base64 data URI from the GResource bundle.</p>
<div class="{css_class}">{sample_text}</div>
</body>
</html>
"#
    );

    let out_path = std::env::temp_dir().join(output_filename);
    std::fs::write(&out_path, &html).into_diagnostic()?;
    println!("HTML written to: {}", out_path.display());
    println!("Open this file in a browser to see the {content_label} rendered with CSS classes.");

    Ok(())
}
