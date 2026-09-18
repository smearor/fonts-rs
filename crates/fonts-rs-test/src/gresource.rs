//! GResource lookup helper.
//!
//! Provides [`lookup_glyph_svg`] for loading SVG glyph data from a
//! registered GResource bundle.

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::SCALABLE_DIR;

/// Looks up an SVG glyph from the GResource bundle.
///
/// Builds the resource path from the font family's `GRESOURCE_PREFIX`
/// and `ICONS_CONTEXT`, then loads the SVG data via `gio::resources_lookup_data`.
///
/// Returns `None` if the resource cannot be found or is not valid UTF-8.
pub fn lookup_glyph_svg<C: FontFamilyConfig>(glyph_name: &str) -> Option<String> {
    let resource_path = format!("{}/{}/{}/{}.svg", C::GRESOURCE_PREFIX, SCALABLE_DIR, C::ICONS_CONTEXT, glyph_name);
    let bytes = gio::resources_lookup_data(&resource_path, gio::ResourceLookupFlags::NONE).ok()?;
    let svg = std::str::from_utf8(bytes.as_ref()).ok()?;
    Some(svg.to_string())
}
