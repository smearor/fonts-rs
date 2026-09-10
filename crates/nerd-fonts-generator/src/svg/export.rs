//! SVG icon generation from font glyphs.

use super::path_builder::SvgPathBuilder;

/// Minimal empty SVG placeholder for glyphs without outlines.
pub const EMPTY_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="0 0 1 1"><path d=""/></svg>"#;

/// Generate the SVG content for a single glyph.
///
/// Returns `None` if the glyph has no outline or a zero-sized bounding box.
pub fn glyph_to_svg(face: &ttf_parser::Face, glyph_id: ttf_parser::GlyphId) -> Option<String> {
    let mut builder = SvgPathBuilder::new();
    let bbox = face.outline_glyph(glyph_id, &mut builder)?;

    let width = bbox.x_max - bbox.x_min;
    let height = bbox.y_max - bbox.y_min;

    if width <= 0 || height <= 0 {
        return None;
    }

    // Flip Y axis (font coords are bottom-up, SVG is top-down)
    let transform = format!("matrix(1 0 0 -1 0 {})", bbox.y_max);

    Some(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="{} {} {} {}"><g transform="{}"><path d="{}"/></g></svg>"#,
        width,
        height,
        bbox.x_min,
        bbox.y_min,
        width,
        height,
        transform,
        builder.path.trim()
    ))
}
