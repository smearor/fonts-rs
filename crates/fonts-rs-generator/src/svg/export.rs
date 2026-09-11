//! SVG icon generation from font glyphs.

use skrifa::FontRef;
use skrifa::GlyphId;
use skrifa::MetadataProvider;
use skrifa::instance::LocationRef;
use skrifa::instance::Size;
use skrifa::outline::DrawSettings;

use super::path_builder::SvgPathBuilder;

/// Minimal empty SVG placeholder for glyphs without outlines.
pub const EMPTY_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="0 0 1 1"><path d=""/></svg>"#;

/// Generate the SVG content for a single glyph.
///
/// Returns `None` if the glyph has no outline or a zero-sized bounding box.
pub fn glyph_to_svg(font: &FontRef, glyph_id: GlyphId) -> Option<String> {
    let mut builder = SvgPathBuilder::new();
    let outlines = font.outline_glyphs();
    let outline = outlines.get(glyph_id)?;
    outline
        .draw(DrawSettings::unhinted(Size::unscaled(), LocationRef::default()), &mut builder)
        .ok()?;

    let (x_min, y_min, x_max, y_max) = builder.bounds()?;
    let width = x_max - x_min;
    let height = y_max - y_min;

    if width <= 0.0 || height <= 0.0 {
        return None;
    }

    // Flip Y axis (font coords are bottom-up, SVG is top-down)
    let transform = format!("matrix(1 0 0 -1 0 {})", y_max);

    Some(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="{} {} {} {}"><g transform="{}"><path d="{}"/></g></svg>"#,
        width,
        height,
        x_min,
        y_min,
        width,
        height,
        transform,
        builder.path.trim()
    ))
}
