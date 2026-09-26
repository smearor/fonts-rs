//! CSS class HTML example.
//!
//! Provides [`run_css_class_html`] for generating an HTML page with
//! `@font-face` and CSS class names, embedding the TTF as a base64 data URI.

use fonts_rs_model::HtmlExport;
use miette::IntoDiagnostic;
use miette::Result;

/// Generates an HTML page with `@font-face` and a CSS class.
///
/// Delegates to [`HtmlExport::html`] + `write_font_face`: registers
/// the font's GResource bundles, embeds the TTF as a base64 data URI,
/// and writes an HTML file to the system temp directory. The CSS class
/// name defaults to `F::FONT_FAMILY_NAME` with `_` replaced by `-`.
///
/// # Type Parameters
///
/// - `F`: Font family implementing [`HtmlExport`].
///
/// # Parameters
///
/// - `content_label`: Content type label (e.g. `"Emoji"`, `"EAN-Barcode"`).
/// - `sample_text`: Text to render with the font.
/// - `html_title`: Title for the HTML page.
/// - `output_filename`: Filename for the output HTML file (in temp dir).
///
/// # Errors
///
/// Returns an error if GResource registration, font loading, or file writing fails.
pub fn run_css_class_html<F: HtmlExport>(content_label: &str, sample_text: &str, html_title: &str, output_filename: &str) -> Result<()> {
    let out_path = std::env::temp_dir().join(output_filename);
    F::html()
        .title(html_title)
        .sample_text(sample_text)
        .write_font_face(&out_path)
        .into_diagnostic()?;

    println!("HTML written to: {}", out_path.display());
    println!("Open this file in a browser to see the {content_label} rendered with CSS classes.");

    Ok(())
}
