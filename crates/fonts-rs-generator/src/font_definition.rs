//! `FontDefinition` trait and `normalize_to_kebab` helper.
//!
//! The central abstraction for the generic export pipeline. Each font family
//! crate implements [`FontDefinition`] to specify its GResource prefix,
//! naming convention, and glyph name normalization logic.

use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;

use fonts_rs_model::FontFamily;
use fonts_rs_model::GlyphEntry;
use fonts_rs_model::ResourcePath;
use quick_xml::se::Serializer;
use serde::Deserialize;
use serde::Serialize;
use skrifa::GlyphId;
use skrifa::MetadataProvider;

use crate::font::Font;
use crate::gresource::model::GResource;
use crate::gresource::model::GResourceFile;
use crate::gresource::model::GResources;
use crate::svg::EMPTY_SVG;

/// Defines a font family's build-time configuration for the generic
/// export pipeline.
///
/// Each font family crate implements this trait to specify its GResource
/// prefix, naming convention, and glyph name normalization logic. The
/// default [`FontDefinition::export_glyphs`] method uses this
/// configuration to produce SVG icons, metadata, and GResource XML for
/// any TTF/OTF font.
pub trait FontDefinition {
    /// GResource prefix for this font family.
    ///
    /// e.g. `/io/smearor/fonts/seven_segment` or `/io/smearor/fonts/nerd_fonts`.
    const GRESOURCE_PREFIX: &'static str;

    /// Icon context subdirectory within the GResource prefix.
    ///
    /// Follows the Freedesktop Icon Theme Specification used by GTK 4's
    /// `GtkIconTheme`: `{prefix}/scalable/{context}/{name}.svg`.
    ///
    /// The `scalable` directory signals that icons are vector (SVG) and
    /// can be rendered at any size. The `context` subdirectory groups
    /// icons by semantic category (e.g. `glyphs`, `status`, `actions`).
    ///
    /// For font glyph icons, `"glyphs"` is the default context.
    /// Font families can override this to use a different context if
    /// needed (e.g. `"emoji"` for Noto Emoji).
    const ICONS_CONTEXT: &'static str = "glyphs";

    /// Unicode codepoint ranges to probe when building the reverse cmap.
    ///
    /// Each tuple is `(start, end)` inclusive. The generic pipeline probes
    /// these ranges to map `GlyphId` -> `CodePoint` for each glyph in the
    /// font.
    ///
    /// Defaults to the BMP (`U+0000`–`U+FFFF`), which covers most fonts.
    /// Font families with glyphs in supplementary planes (e.g. Nerd Fonts
    /// PUA at `U+F0001`–`U+10FFFF`) should override this.
    const CODEPOINT_RANGES: &[(u32, u32)] = &[(0x0000, 0xFFFF)];

    /// The glyph name type used by this font family.
    ///
    /// For simple font families, this is `GlyphName<Self::Family>` — the
    /// shared phantom-typed newtype from `fonts-rs-model`. For Nerd Fonts,
    /// this is `IconName`, which carries additional semantics.
    ///
    /// Must implement `AsRef<str>` for use in GResource paths and phf maps,
    /// `Clone` + `Ord` for sorting, and `serde::Serialize` +
    /// `serde::Deserialize` for metadata persistence.
    type Name: AsRef<str> + Clone + Ord + Serialize + for<'a> Deserialize<'a>;

    /// The `FontFamily` marker type for this font family.
    ///
    /// Used as the phantom type parameter for `GlyphName<Self::Family>`.
    /// For Nerd Fonts, this is `NerdFonts`. For simple families, this is
    /// the family's marker enum (e.g. `SevenSegment`, `Barcode`).
    ///
    /// This associated type is only used when `Self::Name` is `GlyphName<F>`.
    /// It is not used when `Self::Name` is a custom type like `IconName`.
    type Family: FontFamily;

    /// Normalize a raw TTF glyph name to this font family's naming
    /// convention.
    ///
    /// Returns `None` if the glyph name is invalid or does not match
    /// the family's naming rules (e.g. unknown prefix for Nerd Fonts).
    ///
    /// # Examples
    ///
    /// - Nerd Fonts: `"fa-gamepad"` -> `"nf-fa-gamepad-symbolic"`
    /// - Seven Segment: `"zero"` -> `"dseg7-0"`
    /// - Barcode: `"code128_start_a"` -> `"barcode-code128-start-a"`
    fn normalize_name(raw_glyph_name: &str) -> Option<Self::Name>;

    /// Whether to skip a raw glyph name during export.
    ///
    /// Returns `true` for names that should not be exported. The default
    /// implementation skips names starting with `.`, `uni`, or `u` (Nerd
    /// Fonts convention). Font families with different naming conventions
    /// should override this.
    fn should_skip(raw_glyph_name: &str) -> bool {
        raw_glyph_name.starts_with('.') || raw_glyph_name.starts_with("uni") || raw_glyph_name.starts_with("u")
    }

    /// Generate the GResource XML manifest file for this font family.
    ///
    /// Creates an `icons.gresource.xml` listing all glyph SVG files under
    /// the font family's GResource prefix.
    fn generate_gresource_xml(entries: &[GlyphEntry<Self::Name>], output_path: &Path) -> std::io::Result<()>
    where
        Self: Sized,
    {
        let mut sorted: Vec<&GlyphEntry<Self::Name>> = entries.iter().collect();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));

        let files: Vec<GResourceFile> = sorted
            .iter()
            .map(|entry| GResourceFile {
                path: format!("scalable/{}/{}.svg", Self::ICONS_CONTEXT, entry.name.as_ref()),
            })
            .collect();

        let manifest = GResources {
            gresource: GResource {
                prefix: Self::GRESOURCE_PREFIX.to_string(),
                files,
            },
        };

        let mut buffer = String::new();
        let mut serializer = Serializer::new(&mut buffer);
        serializer.indent(' ', 2);
        manifest
            .serialize(serializer)
            .map_err(|e| std::io::Error::other(format!("XML serialization failed: {e}")))?;

        let xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{buffer}");
        fs::write(output_path, xml)?;
        Ok(())
    }

    /// Export all glyphs from a TTF/OTF font file as GTK 4 symbolic SVG icons.
    ///
    /// This is the generic export pipeline, using the trait's configuration
    /// to normalize glyph names, construct resource paths, and generate
    /// GResource XML for any font family.
    ///
    /// Generates:
    /// - `<output_dir>/scalable/{context}/*.svg` — one SVG file per glyph
    /// - `<output_dir>/metadata.json` — glyph metadata (name, codepoint, file path)
    /// - `<output_dir>/icons.gresource.xml` — GResource bundle manifest
    ///
    /// # Arguments
    ///
    /// - `font_path` — Path to the TTF/OTF font file
    /// - `output_dir` — Root output directory (typically `resources/`)
    ///
    /// # Returns
    ///
    /// The number of exported glyphs on success, or an `io::Error` on failure.
    fn export_glyphs(font_path: &Path, output_dir: &Path) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        let font_data = fs::read(font_path)?;
        let font = Font::from_data(&font_data).map_err(|e| std::io::Error::other(format!("Failed to parse font: {e}")))?;

        // SVG output directory follows Freedesktop Icon Theme convention:
        // {output_dir}/scalable/{context}/
        let icons_dir = output_dir.join("scalable").join(Self::ICONS_CONTEXT);
        fs::create_dir_all(&icons_dir)?;

        let reverse_cmap = font.build_reverse_cmap::<Self>();

        let mut entries: Vec<GlyphEntry<Self::Name>> = Vec::new();
        let mut seen_names: HashSet<String> = HashSet::new();

        let num_glyphs = font.glyph_names().num_glyphs();

        for glyph_index in 0..num_glyphs {
            let glyph_id = GlyphId::new(glyph_index);

            let name = match font.glyph_names().get(glyph_id) {
                Some(n) if !n.as_str().is_empty() => n.to_string(),
                _ => continue,
            };

            // Skip invalid glyph names (configurable per font family)
            if Self::should_skip(&name) {
                continue;
            }

            let Some(glyph_name) = Self::normalize_name(&name) else {
                continue;
            };

            // Deduplicate
            if seen_names.contains(glyph_name.as_ref()) {
                continue;
            }
            seen_names.insert(glyph_name.as_ref().to_string());

            let codepoint = reverse_cmap.get(&glyph_id).copied();

            let svg = match font.glyph_to_svg(glyph_id) {
                Some(svg) => svg,
                None => EMPTY_SVG.to_string(),
            };

            let filename = icons_dir.join(format!("{}.svg", glyph_name.as_ref()));
            let mut file = fs::File::create(&filename)?;
            file.write_all(svg.as_bytes())?;

            // GResource path follows Freedesktop Icon Theme convention:
            // {prefix}/scalable/{context}/{name}.svg
            let resource_prefix = format!("{}/scalable/{}", Self::GRESOURCE_PREFIX, Self::ICONS_CONTEXT);
            let resource_path = ResourcePath::from_name(&resource_prefix, glyph_name.as_ref());

            entries.push(GlyphEntry {
                code: codepoint,
                name: glyph_name.clone(),
                file: Path::new(&format!("resources/scalable/{}/{}.svg", Self::ICONS_CONTEXT, glyph_name.as_ref())).to_path_buf(),
                resource_path,
            });
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));

        let json_path = output_dir.join("metadata.json");
        let json = serde_json::to_string_pretty(&entries).map_err(std::io::Error::other)?;
        fs::write(&json_path, json)?;

        let xml_path = output_dir.join("icons.gresource.xml");
        Self::generate_gresource_xml(&entries, &xml_path)?;

        Ok(entries.len())
    }
}

/// Normalize a raw glyph name to kebab-case.
///
/// Converts to lowercase, replaces underscores and non-alphanumeric
/// characters with hyphens, collapses consecutive hyphens, and trims
/// leading/trailing hyphens.
///
/// Returns `None` if the result is empty.
///
/// # Examples
///
/// ```
/// use fonts_rs_generator::normalize_to_kebab;
///
/// assert_eq!(normalize_to_kebab("code128_start_a"), Some("code128-start-a".to_string()));
/// assert_eq!(normalize_to_kebab("zero"), Some("zero".to_string()));
/// assert_eq!(normalize_to_kebab("---"), None);
/// ```
pub fn normalize_to_kebab(raw: &str) -> Option<String> {
    let name = raw.to_lowercase().replace('_', "-");
    let name: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).collect();
    let name: String = name.split('-').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("-");
    if name.is_empty() { None } else { Some(name) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_to_kebab_basic() {
        assert_eq!(normalize_to_kebab("code128_start_a"), Some("code128-start-a".to_string()));
    }

    #[test]
    fn normalize_to_kebab_single_word() {
        assert_eq!(normalize_to_kebab("zero"), Some("zero".to_string()));
    }

    #[test]
    fn normalize_to_kebab_empty_after_filtering() {
        assert_eq!(normalize_to_kebab("---"), None);
    }

    #[test]
    fn normalize_to_kebab_uppercase_to_lowercase() {
        assert_eq!(normalize_to_kebab("CamelCase"), Some("camelcase".to_string()));
    }

    #[test]
    fn normalize_to_kebab_special_chars() {
        assert_eq!(normalize_to_kebab("foo.bar.baz"), Some("foo-bar-baz".to_string()));
    }

    #[test]
    fn normalize_to_kebab_collapses_consecutive_hyphens() {
        assert_eq!(normalize_to_kebab("foo--bar"), Some("foo-bar".to_string()));
    }

    #[test]
    fn normalize_to_kebab_trims_hyphens() {
        assert_eq!(normalize_to_kebab("-foo-"), Some("foo".to_string()));
    }
}
