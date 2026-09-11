//! SVG icon generation from font glyphs.

use font_types::Matrix;
use skrifa::GlyphId;
use skrifa::MetadataProvider;
use skrifa::instance::LocationRef;
use skrifa::instance::Size;
use skrifa::outline::DrawSettings;

use super::path_builder::SvgPathBuilder;
use crate::font::Font;

/// Minimal empty SVG placeholder for glyphs without outlines.
pub const EMPTY_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="0 0 1 1"><path d=""/></svg>"#;

impl<'a> Font<'a> {
    /// Generate the SVG content for a single glyph.
    ///
    /// Returns `None` if the glyph has no outline or a zero-sized bounding box.
    pub fn glyph_to_svg(&self, glyph_id: GlyphId) -> Option<String> {
        let mut builder = SvgPathBuilder::new();
        let outlines = self.outline_glyphs();
        let outline = outlines.get(glyph_id)?;
        outline
            .draw(DrawSettings::unhinted(Size::unscaled(), LocationRef::default()), &mut builder)
            .ok()?;

        let bbox = builder.bounds()?;
        let width = bbox.x_max - bbox.x_min;
        let height = bbox.y_max - bbox.y_min;

        if width <= 0.0 || height <= 0.0 {
            return None;
        }

        // Flip Y axis (font coords are bottom-up, SVG is top-down)
        let transform = Matrix::from_elements([1.0, 0.0, 0.0, -1.0, 0.0, bbox.y_max]);

        Some(format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="{} {} {} {}"><g transform="matrix({} {} {} {} {} {})"><path d="{}"/></g></svg>"#,
            width,
            height,
            bbox.x_min,
            bbox.y_min,
            width,
            height,
            transform.xx,
            transform.yx,
            transform.xy,
            transform.yy,
            transform.dx,
            transform.dy,
            builder.path.trim()
        ))
    }
}
