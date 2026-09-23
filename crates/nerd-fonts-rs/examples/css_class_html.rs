//! Minimal example: generate an HTML page using the `.nerd-icon` CSS classes.
//!
//! Nerd Fonts deliberately does not implement `HtmlExport` (`@font-face`
//! is unreliable in the GTK4 CSS parser). This page uses the same
//! `.nerd-icon` / `.nerd-icon-mono` classes that [`icon_css`] generates
//! for GTK — the Symbols Nerd Font must be installed on the system for
//! the icons to render.
//!
//! Run with:
//! ```sh
//! cargo run --example css_class_html --features gtk
//! ```

use miette::IntoDiagnostic;
use nerd_fonts_rs::icon_css;

/// Sample text:  (U+F005 star),  (U+F004 heart),  (U+F06D fire),  (U+F135 rocket)
const SAMPLE_TEXT: &str = "\u{F005}\u{F004}\u{F06D}\u{F135}";

fn main() -> miette::Result<()> {
    let css = icon_css();

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Nerd Fonts — CSS Class</title>
<style>
{css}
body {{
    font-family: sans-serif;
    max-width: 700px;
    margin: 2rem auto;
    padding: 0 1rem;
}}
.nerd-icon {{
    text-align: center;
    margin: 2rem 0;
}}
</style>
</head>
<body>
<h1>Nerd Fonts — CSS Class</h1>
<p>The icons below use the <code>.nerd-icon</code> CSS class referencing
the <code>NerdFontsSymbolsOnly</code> font family. The font must be
installed on the system (or loaded via Pango in GTK) — no
<code>@font-face</code> rule is embedded.</p>
<div class="nerd-icon">{SAMPLE_TEXT}</div>
</body>
</html>
"#
    );

    let out_path = std::env::temp_dir().join("nerd_fonts_css_class.html");
    std::fs::write(&out_path, html).into_diagnostic()?;

    println!("HTML written to: {}", out_path.display());
    println!("Open this file in a browser to see the CSS-class icons.");

    Ok(())
}
