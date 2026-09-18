//! SVG HTML example.
//!
//! Provides [`run_svg_html`] for generating an HTML page with inline SVG
//! glyphs from GResource.

use fonts_rs_model::HtmlExport;
use fonts_rs_model::HtmlMode;
use miette::IntoDiagnostic;
use miette::Result;

/// Generates an HTML page with inline SVG glyphs from GResource.
///
/// Delegates to [`HtmlExport::html`] in `HtmlMode::InlineSvg`: registers
/// the font's GResource bundles, loads all glyph SVGs, and writes an
/// HTML file with inline `<svg>` elements to the system temp directory.
///
/// # Type Parameters
///
/// - `F`: Font family implementing [`HtmlExport`].
///
/// # Parameters
///
/// - `html_title`: Title for the HTML page.
/// - `output_filename`: Filename for the output HTML file (in temp dir).
///
/// # Errors
///
/// Returns an error if GResource registration or file writing fails.
pub fn run_svg_html<F: HtmlExport>(html_title: &str, output_filename: &str) -> Result<()> {
    let out_path = std::env::temp_dir().join(output_filename);
    F::html().mode(HtmlMode::InlineSvg).title(html_title).write(&out_path).into_diagnostic()?;

    println!("HTML written to: {}", out_path.display());
    println!("Open this file in a browser to see the SVG glyphs.");

    Ok(())
}
