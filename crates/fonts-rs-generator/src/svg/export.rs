//! SVG icon generation from font glyphs.

use font_types::Matrix;
use skrifa::GlyphId;
use skrifa::MetadataProvider;
use skrifa::instance::LocationRef;
use skrifa::instance::Size;
use skrifa::outline::DrawSettings;

use super::path_builder::SvgPathBuilder;
use super::svg::Svg;
use crate::font::Font;

/// Minimal empty SVG placeholder for glyphs without outlines.
pub const EMPTY_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="0 0 1 1"><path d=""/></svg>"#;

impl<'a> Font<'a> {
    /// Generate the SVG content for a single glyph.
    ///
    /// Returns `None` if the glyph has no outline or a zero-sized bounding box.
    pub fn glyph_to_svg(&self, glyph_id: GlyphId) -> Option<Svg> {
        self.glyph_to_svg_at(glyph_id, LocationRef::default())
    }

    /// Generate the SVG content for a single glyph at the given variation
    /// location.
    ///
    /// Returns `None` if the glyph has no outline or a zero-sized bounding box.
    pub fn glyph_to_svg_at(&self, glyph_id: GlyphId, location: LocationRef<'_>) -> Option<Svg> {
        let mut builder = SvgPathBuilder::new();
        let outlines = self.outline_glyphs();
        let outline = outlines.get(glyph_id)?;
        outline.draw(DrawSettings::unhinted(Size::unscaled(), location), &mut builder).ok()?;

        let bbox = builder.bounds()?;
        let width = bbox.x_max - bbox.x_min;
        let height = bbox.y_max - bbox.y_min;

        if width <= 0.0 || height <= 0.0 {
            return None;
        }

        // Flip Y axis (font coords are bottom-up, SVG is top-down)
        let transform = Matrix::from_elements([1.0, 0.0, 0.0, -1.0, 0.0, bbox.y_max]);

        Some(Svg::new(format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="{} {} {} {}"><g transform="matrix({} {} {} {} {} {})"><path fill="currentColor" d="{}"/></g></svg>"#,
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
        )))
    }

    /// Generate SVG content for a glyph using the font's full height for the viewBox.
    ///
    /// Unlike [`glyph_to_svg`](Self::glyph_to_svg), which crops the viewBox to the
    /// glyph's bounding box, this method uses the font's ascent/descent for the
    /// vertical extent. This ensures that glyphs which only occupy part of the
    /// font's height (e.g. lowercase letters in a 7-segment display font) are not
    /// scaled up disproportionately when rendered at a fixed icon size.
    ///
    /// The glyph is horizontally positioned at its natural x offset within the
    /// font's coordinate space.
    pub fn glyph_to_svg_full_height(&self, glyph_id: GlyphId) -> Option<Svg> {
        self.glyph_to_svg_full_height_at(glyph_id, LocationRef::default())
    }

    /// Generate SVG content for a glyph at the given variation location,
    /// using the font's full height for the viewBox.
    ///
    /// Like [`glyph_to_svg_full_height`](Self::glyph_to_svg_full_height) but
    /// renders the glyph at the specified variation axis location.
    pub fn glyph_to_svg_full_height_at(&self, glyph_id: GlyphId, location: LocationRef<'_>) -> Option<Svg> {
        let mut builder = SvgPathBuilder::new();
        let outlines = self.outline_glyphs();
        let outline = outlines.get(glyph_id)?;
        outline.draw(DrawSettings::unhinted(Size::unscaled(), location), &mut builder).ok()?;

        let bbox = builder.bounds()?;
        let glyph_width = bbox.x_max - bbox.x_min;

        if glyph_width <= 0.0 {
            return None;
        }

        let metrics = self.metrics(Size::unscaled(), location);
        let font_height = metrics.ascent - metrics.descent;

        if font_height <= 0.0 {
            return None;
        }

        // Transform: shift x so glyph starts at 0, flip Y, shift y so baseline
        // is at ascent from the top of the viewBox.
        let transform = Matrix::from_elements([1.0, 0.0, 0.0, -1.0, -bbox.x_min, metrics.ascent]);

        Some(Svg::new(format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}"><g transform="matrix({} {} {} {} {} {})"><path fill="currentColor" d="{}"/></g></svg>"#,
            glyph_width,
            font_height,
            glyph_width,
            font_height,
            transform.xx,
            transform.yx,
            transform.xy,
            transform.yy,
            transform.dx,
            transform.dy,
            builder.path.trim()
        )))
    }
}
