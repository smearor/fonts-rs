//! Icon theme and SVG glyph capability.
//!
//! Provides the [`IconThemeFont`] trait for font families whose glyph
//! GResource can serve both `GtkIconTheme` lookups and direct SVG access.

use crate::FontError;
use crate::FontFamily;
use crate::GtkFont;

/// Icon theme lookup and SVG glyph access.
///
/// Requires the family's glyph GResource. All methods have default
/// implementations built on the [`FontFamily`] glyph maps and the
/// [`GtkFont`] registration primitives - opting in is an empty impl
/// emitted by `impl_font_family!`.
pub trait IconThemeFont: FontFamily + GtkFont {
    /// Registers the glyph GResource and adds the family's icon
    /// resource path to the `GtkIconTheme` of `display`.
    ///
    /// After this call, `Image::from_icon_name(...)` resolves glyphs of
    /// this family.
    ///
    /// # Errors
    ///
    /// Returns a [`FontError`] if GResource registration fails.
    fn install_icon_theme(display: &gtk4::gdk::Display) -> Result<(), FontError> {
        Self::ensure_registered()?;
        let icon_theme = gtk4::IconTheme::for_display(display);
        icon_theme.add_resource_path(&Self::icon_theme_path());
        Ok(())
    }

    /// The icon theme resource path, e.g.
    /// `/io/smearor/fonts/barcode_code39/scalable/glyphs`.
    fn icon_theme_path() -> String {
        Self::icons_resource_prefix()
    }

    /// Returns the SVG document for one glyph, or `None` if the name is
    /// unknown or the resource is missing.
    ///
    /// Registers the glyph GResource lazily on first call.
    fn glyph_svg(name: impl AsRef<str>) -> Option<String> {
        Self::ensure_registered().ok()?;
        let resource_path = format!("{}/{}.svg", Self::icons_resource_prefix(), name.as_ref());
        let bytes = gio::resources_lookup_data(&resource_path, gio::ResourceLookupFlags::NONE).ok()?;
        std::str::from_utf8(bytes.as_ref()).ok().map(str::to_string)
    }

    /// Composes `text` into a single SVG document with all glyphs side
    /// by side at `glyph_height` user units. Characters without a glyph
    /// mapping are skipped.
    ///
    /// Each glyph SVG is loaded from the GResource, its `viewBox` and
    /// inner content extracted, and placed as a nested `<svg>` element
    /// at the correct x offset.
    fn compose_svg(text: &str, glyph_height: f64) -> String {
        let _ = Self::ensure_registered();

        let mut segments: Vec<(f64, String, String, String)> = Vec::new();

        for ch in text.chars() {
            let Some(name) = Self::glyph_name(ch) else {
                continue;
            };
            let Some(svg) = Self::glyph_svg(name) else {
                continue;
            };

            let viewbox = extract_svg_viewbox(&svg).to_string();
            let xmlns = extract_svg_xmlns(&svg);
            let content = extract_svg_inner(&svg).to_string();

            let parts: Vec<f64> = viewbox.split_whitespace().filter_map(|s| s.parse().ok()).collect();
            let scaled_width = if parts.len() >= 4 && parts[3] > 0.0 {
                glyph_height * parts[2] / parts[3]
            } else {
                glyph_height * 0.3
            };

            segments.push((scaled_width, viewbox, xmlns, content));
        }

        let total_width: f64 = segments.iter().map(|(w, ..)| *w).sum();
        let total_width = total_width.ceil().max(1.0) as u32;
        let total_height = glyph_height.ceil() as u32;

        let mut inner = String::new();
        let mut x = 0.0_f64;
        for (scaled_width, viewbox, xmlns, content) in &segments {
            let colored = content.replace("currentColor", "#000");
            let w = scaled_width.ceil();
            inner.push_str(&format!(
                "<svg x=\"{x:.0}\" y=\"0\" width=\"{w}\" height=\"{glyph_height:.0}\" viewBox=\"{viewbox}\"{xmlns}>{colored}</svg>"
            ));
            x += scaled_width;
        }

        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{total_width}\" height=\"{total_height}\"><rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>{inner}</svg>"
        )
    }
}

/// Extracts the `xmlns` namespace declarations from the root `<svg>`
/// tag, e.g. ` xmlns="…" xmlns:ns1="…"` (leading space per attribute).
///
/// Glyph SVGs may use prefixed namespaces (`ns1:href` on `<use>`
/// elements). Stripping the root tag for composition would drop the
/// declarations and leave the prefixes unbound, so they are re-attached
/// to each nested `<svg>` element.
fn extract_svg_xmlns(svg: &str) -> String {
    let Some(start) = svg.find("<svg") else {
        return String::new();
    };
    let Some(end) = svg[start..].find('>') else {
        return String::new();
    };
    svg[start..start + end]
        .split_whitespace()
        .filter(|attr| attr.starts_with("xmlns"))
        .fold(String::new(), |mut out, attr| {
            out.push(' ');
            out.push_str(attr);
            out
        })
}

/// Extracts the `viewBox` attribute value from an SVG string.
fn extract_svg_viewbox(svg: &str) -> &str {
    let pattern = "viewBox=\"";
    if let Some(start) = svg.find(pattern) {
        let start = start + pattern.len();
        if let Some(end) = svg[start..].find('"') {
            return &svg[start..start + end];
        }
    }
    "0 0 400 1000"
}

/// Extracts the inner content of an SVG (between the opening tag and `</svg>`).
fn extract_svg_inner(svg: &str) -> &str {
    if let Some(svg_start) = svg.find("<svg")
        && let Some(tag_end) = svg[svg_start..].find('>')
    {
        let content_start = svg_start + tag_end + 1;
        if let Some(close) = svg.rfind("</svg>") {
            return &svg[content_start..close];
        }
    }
    ""
}
