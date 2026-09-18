//! SVG HTML example.
//!
//! Provides [`run_svg_html`] for generating an HTML page with inline SVG
//! glyphs from GResource.

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GtkFont;
use fonts_rs_model::SCALABLE_DIR;
use miette::IntoDiagnostic;
use miette::Result;

/// Generates an HTML page with inline SVG glyphs from GResource.
///
/// Registers the font's SVG GResource, loads all glyph SVGs, and writes
/// an HTML file with inline `<svg>` elements to the system temp directory.
///
/// # Type Parameters
///
/// - `T`: Font marker type implementing [`GtkFont`].
/// - `C`: Font config type implementing [`FontFamilyConfig`].
///
/// # Parameters
///
/// - `html_title`: Title for the HTML page.
/// - `glyphs`: Slice of `(codepoint, glyph_name)` pairs to include.
/// - `output_filename`: Filename for the output HTML file (in temp dir).
///
/// # Errors
///
/// Returns an error if GResource registration or file writing fails.
pub fn run_svg_html<T, C>(html_title: &str, gresource_prefix: &str, glyphs: &[(char, &str)], output_filename: &str) -> Result<()>
where
    T: GtkFont,
    C: FontFamilyConfig,
{
    T::register_glyphs().into_diagnostic()?;

    let prefix = gresource_prefix;

    let mut svg_items = Vec::new();

    for (_codepoint, name) in glyphs {
        let resource_path = format!("{}/{}/{}/{}.svg", prefix, SCALABLE_DIR, C::ICONS_CONTEXT, name);

        match gio::resources_lookup_data(&resource_path, gio::ResourceLookupFlags::NONE) {
            Ok(bytes) => {
                let svg = std::str::from_utf8(bytes.as_ref()).unwrap_or("");
                let svg = svg.replace("currentColor", "#333");
                svg_items.push((name.to_string(), svg.to_string()));
            }
            Err(e) => {
                eprintln!("Failed to load SVG for '{name}': {e}");
            }
        }
    }

    let mut svg_grid = String::new();
    for (name, svg) in &svg_items {
        svg_grid.push_str(&format!(
            r#"<div class="glyph-cell"><div class="glyph-svg">{svg}</div><div class="glyph-name">{name}</div></div>"#
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{html_title}</title>
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
<h1>{html_title}</h1>
<p>Each glyph below is an inline SVG loaded from the GResource bundle.
No TTF font or <code>@font-face</code> is required — the SVGs are
pre-rendered vector outlines embedded directly in the HTML.</p>
<div class="glyph-grid">
{svg_grid}
</div>
</body>
</html>
"#
    );

    let out_path = std::env::temp_dir().join(output_filename);
    std::fs::write(&out_path, &html).into_diagnostic()?;
    println!("HTML written to: {}", out_path.display());
    println!("Open this file in a browser to see all {} SVG glyphs.", svg_items.len());

    Ok(())
}
