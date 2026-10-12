//! TTF variant selection for the Noto Emoji family.

use fonts_rs_generator::build_constants::RESOURCES_DIR;
use std::path::PathBuf;

/// One TTF variant of the Noto Emoji family.
///
/// `Mono` is the classic monochrome outline font; the other four form
/// the 2x2 matrix color (COLRv1) x bitmap (CBDT) with/without regional
/// flags. In the multi-crate layout each variant compiles into its own
/// crate; the legacy single crate selects it via Cargo features
/// ([`Variant::from_feature_name`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    /// Monochrome outline font (`NotoEmoji-Regular.ttf`).
    Mono,
    /// COLRv1 color font including flag emoji (`Noto-COLRv1.ttf`).
    ColorFlags,
    /// COLRv1 color font without flag emoji (`Noto-COLRv1-noflags.ttf`).
    ColorNoflags,
    /// CBDT bitmap font including flag emoji (`NotoColorEmoji.ttf`).
    BitmapFlags,
    /// CBDT bitmap font without flag emoji (`NotoColorEmoji-noflags.ttf`).
    BitmapNoflags,
}

impl Variant {
    /// Variant name matching the Cargo feature (kebab-case).
    pub fn feature_name(&self) -> &'static str {
        match self {
            Variant::Mono => "mono",
            Variant::ColorFlags => "color-flags",
            Variant::ColorNoflags => "color-noflags",
            Variant::BitmapFlags => "bitmap-flags",
            Variant::BitmapNoflags => "bitmap-noflags",
        }
    }

    /// TTF file stem (no extension) within `resources/`.
    pub fn font_stem(&self) -> &'static str {
        match self {
            Variant::Mono => "NotoEmoji-Regular",
            Variant::ColorFlags => "Noto-COLRv1",
            Variant::ColorNoflags => "Noto-COLRv1-noflags",
            Variant::BitmapFlags => "NotoColorEmoji",
            Variant::BitmapNoflags => "NotoColorEmoji-noflags",
        }
    }

    /// TTF file name within `resources/`.
    pub fn font_file(&self) -> &'static str {
        match self {
            Variant::Mono => "NotoEmoji-Regular.ttf",
            Variant::ColorFlags => "Noto-COLRv1.ttf",
            Variant::ColorNoflags => "Noto-COLRv1-noflags.ttf",
            Variant::BitmapFlags => "NotoColorEmoji.ttf",
            Variant::BitmapNoflags => "NotoColorEmoji-noflags.ttf",
        }
    }

    /// Crate-relative font path (`resources/{file}`).
    pub fn font_path(&self) -> PathBuf {
        PathBuf::from(RESOURCES_DIR).join(self.font_file())
    }

    /// `true` when the variant exports flag emoji.
    pub fn include_flags(&self) -> bool {
        !self.feature_name().contains("noflags")
    }

    /// `true` for CBDT bitmap variants.
    pub fn is_bitmap(&self) -> bool {
        matches!(self, Variant::BitmapFlags | Variant::BitmapNoflags)
    }

    /// `true` for the monochrome outline variant.
    pub fn is_mono(&self) -> bool {
        matches!(self, Variant::Mono)
    }

    /// `true` when `ab_glyph` produces visible output (mono + bitmap).
    ///
    /// COLRv1 glyphs carry empty `glyf` outlines; pixel-drawing falls
    /// back to monochrome silhouettes there.
    pub fn is_renderable(&self) -> bool {
        self.is_bitmap() || self.is_mono()
    }

    /// Maps a Cargo feature / [`FontVariant`] name back to the variant.
    ///
    /// [`FontVariant`]: fonts_rs_model::FontVariant
    pub fn from_feature_name(name: &str) -> Option<Variant> {
        Some(match name {
            "mono" => Variant::Mono,
            "color-flags" => Variant::ColorFlags,
            "color-noflags" => Variant::ColorNoflags,
            "bitmap-flags" => Variant::BitmapFlags,
            "bitmap-noflags" => Variant::BitmapNoflags,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_names_roundtrip() {
        for variant in [
            Variant::Mono,
            Variant::ColorFlags,
            Variant::ColorNoflags,
            Variant::BitmapFlags,
            Variant::BitmapNoflags,
        ] {
            assert_eq!(Variant::from_feature_name(variant.feature_name()), Some(variant));
        }
        assert_eq!(Variant::from_feature_name("bogus"), None);
    }

    #[test]
    fn flags_matrix() {
        assert!(Variant::ColorFlags.include_flags());
        assert!(Variant::BitmapFlags.include_flags());
        assert!(!Variant::ColorNoflags.include_flags());
        assert!(!Variant::BitmapNoflags.include_flags());
        assert!(Variant::Mono.include_flags());
    }

    #[test]
    fn renderable_variants() {
        assert!(Variant::Mono.is_renderable());
        assert!(Variant::BitmapFlags.is_renderable());
        assert!(Variant::BitmapNoflags.is_renderable());
        assert!(!Variant::ColorFlags.is_renderable());
        assert!(!Variant::ColorNoflags.is_renderable());
    }
}
