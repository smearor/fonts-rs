//! Minimal example: generate an HTML page with inline SVG icons from GResource.
//!
//! Nerd Fonts deliberately does not implement `HtmlExport` (`@font-face`
//! is unreliable in the GTK4 CSS parser), so this example builds the
//! inline-SVG page directly via `NerdFonts::glyph_svg`.
//!
//! Run with:
//! ```sh
//! cargo run --example svg_html --features gtk
//! ```

use miette::IntoDiagnostic;
use nerd_fonts_rs::NerdFonts;

fn main() -> miette::Result<()> {
    NerdFonts::ensure_registered().into_diagnostic()?;

    let mut svg_grid = String::new();
    let mut count = 0_usize;
    for (_, name) in NerdFonts::all_glyphs() {
        let Some(svg) = NerdFonts::glyph_svg(&name) else {
            continue;
        };
        let svg = svg.replace("currentColor", "#333");
        svg_grid.push_str(&format!(
            r#"<div class="glyph-cell"><div class="glyph-svg">{svg}</div><div class="glyph-name">{name}</div></div>"#
        ));
        count += 1;
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Nerd Fonts — SVG Glyphs</title>
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
<h1>Nerd Fonts — SVG Glyphs</h1>
<p>Each icon below is an inline SVG loaded from the GResource bundle.
No TTF font or <code>@font-face</code> is required — the SVGs are
pre-rendered vector outlines embedded directly in the HTML.</p>
<div class="glyph-grid">
{svg_grid}
</div>
<p>{count} icons.</p>
</body>
</html>
"#
    );

    let out_path = std::env::temp_dir().join("nerd_fonts_svg_html.html");
    std::fs::write(&out_path, html).into_diagnostic()?;

    println!("HTML written to: {}", out_path.display());
    println!("Open this file in a browser to see the SVG icons.");

    Ok(())
}
