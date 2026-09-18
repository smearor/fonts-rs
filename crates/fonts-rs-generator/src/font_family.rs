//! Font family runtime API macro.
//!
//! Provides the [`impl_font_family!`] macro for generating the runtime
//! font family API on a marker type: the
//! [`FontFamilyConfig`](::fonts_rs_model::FontFamilyConfig) delegation,
//! the [`FontFamily`](::fonts_rs_model::FontFamily) core impl wiring the
//! generated phf maps, the declared capability impls, and inherent
//! forwarding methods so call sites need no trait imports.
//!
//! Each font crate calls `impl_font_family!` once in a `src/family.rs`
//! file. The other macros keep their roles and stay compositional:
//! [`impl_gtk_font!`](crate::impl_gtk_font) provides the `GtkFont`
//! registration primitives, [`impl_font_loader!`](crate::impl_font_loader)
//! provides `SoftwareRender`.

/// Generates the runtime font family API for a marker type.
///
/// # Invocation
///
/// ```ignore
/// fonts_rs_generator::impl_font_family!(
///     Code39,
///     Code39Definition,
///     glyphs = codepoint_map::GLYPHS,
///     reverse = codepoint_map::REVERSE_GLYPHS,
///     name = Code39Name,
///     css_font_family = "LibreBarcode39", // optional
///     capabilities = [icon_theme, pango, html],
/// );
/// ```
///
/// `css_font_family` is optional: it overrides
/// [`FontFamily::CSS_FONT_FAMILY`](::fonts_rs_model::FontFamily::CSS_FONT_FAMILY)
/// when the TTF's family name differs from `FAMILY_DISPLAY_NAME`.
///
/// # Emitted items
///
/// - `impl FontFamilyConfig for $marker` — delegates all constants to
///   `$definition`.
/// - `impl FontFamily for $marker` — `type Name = $name`, glyph lookups
///   via the `$glyphs` (`phf::Map<char, &'static str>`) and `$reverse`
///   (`phf::Map<&'static str, char>`) maps.
/// - One empty impl per declared capability — all capability methods
///   are trait defaults. Valid capabilities: `icon_theme`
///   ([`IconThemeFont`](::fonts_rs_model::IconThemeFont)), `pango`
///   ([`PangoFont`](::fonts_rs_model::PangoFont)), `html`
///   ([`HtmlExport`](::fonts_rs_model::HtmlExport)).
/// - Inherent forwarding methods for every declared capability plus
///   `install()`/`ensure_registered()`, so concrete call sites need no
///   trait import.
///
/// Capability impls and forwarders are `#[cfg(feature = "gtk")]`-gated;
/// the core impl is feature-free. `SoftwareRender` is not a capability
/// here — it is emitted by [`impl_font_loader!`](crate::impl_font_loader).
#[macro_export]
macro_rules! impl_font_family {
    ($marker:ident, $definition:ty,
     glyphs = $glyphs:expr,
     reverse = $reverse:expr,
     name = $name:ty,
     css_font_family = $css_font_family:expr,
     capabilities = [$($capability:ident),* $(,)?]
     $(,)?) => {
        $crate::impl_font_family!(@impl $marker, $definition,
            glyphs = $glyphs,
            reverse = $reverse,
            name = $name,
            css_font_family = $css_font_family,
            capabilities = [$($capability),*]);
    };
    ($marker:ident, $definition:ty,
     glyphs = $glyphs:expr,
     reverse = $reverse:expr,
     name = $name:ty,
     capabilities = [$($capability:ident),* $(,)?]
     $(,)?) => {
        $crate::impl_font_family!(@impl $marker, $definition,
            glyphs = $glyphs,
            reverse = $reverse,
            name = $name,
            css_font_family = Self::FAMILY_DISPLAY_NAME,
            capabilities = [$($capability),*]);
    };
    (@impl $marker:ident, $definition:ty,
     glyphs = $glyphs:expr,
     reverse = $reverse:expr,
     name = $name:ty,
     css_font_family = $css_font_family:expr,
     capabilities = [$($capability:ident),* $(,)?]) => {
        impl ::fonts_rs_model::FontFamilyConfig for $marker {
            const FONT_FAMILY_NAME: &'static str = <$definition as ::fonts_rs_model::FontFamilyConfig>::FONT_FAMILY_NAME;
            const FAMILY_DISPLAY_NAME: &'static str = <$definition as ::fonts_rs_model::FontFamilyConfig>::FAMILY_DISPLAY_NAME;
            const GRESOURCE_PREFIX: &'static str = <$definition as ::fonts_rs_model::FontFamilyConfig>::GRESOURCE_PREFIX;
            const ICONS_CONTEXT: &'static str = <$definition as ::fonts_rs_model::FontFamilyConfig>::ICONS_CONTEXT;
            const CODEPOINT_RANGES: &[::fonts_rs_model::CodePointRange] =
                <$definition as ::fonts_rs_model::FontFamilyConfig>::CODEPOINT_RANGES;
        }

        impl ::fonts_rs_model::FontFamily for $marker {
            type Name = $name;

            const CSS_FONT_FAMILY: &'static str = $css_font_family;

            fn glyph_name(ch: char) -> Option<&'static str> {
                $glyphs.get(&ch).copied()
            }

            fn codepoint(name: &str) -> Option<char> {
                $reverse.get(name).copied()
            }

            fn all_glyphs() -> impl Iterator<Item = (char, &'static str)> {
                $glyphs.entries().map(|(ch, name)| (*ch, *name))
            }
        }

        $($crate::impl_font_family!(@capability $marker $capability);)*

        #[cfg(feature = "gtk")]
        impl $marker {
            /// Explicit fail-fast setup for application startup.
            ///
            /// Forwards to [`GtkFont::install`](::fonts_rs_model::GtkFont::install).
            pub fn install() -> Result<(), ::fonts_rs_model::FontError> {
                <Self as ::fonts_rs_model::GtkFont>::install()
            }

            /// Idempotent registration of all GResource bundles.
            ///
            /// Forwards to [`GtkFont::ensure_registered`](::fonts_rs_model::GtkFont::ensure_registered).
            pub fn ensure_registered() -> Result<(), ::fonts_rs_model::FontError> {
                <Self as ::fonts_rs_model::GtkFont>::ensure_registered()
            }
        }

        $($crate::impl_font_family!(@forward $marker $capability);)*
    };

    (@capability $marker:ident icon_theme) => {
        #[cfg(feature = "gtk")]
        impl ::fonts_rs_model::IconThemeFont for $marker {}
    };
    (@capability $marker:ident pango) => {
        #[cfg(feature = "gtk")]
        impl ::fonts_rs_model::PangoFont for $marker {}
    };
    (@capability $marker:ident html) => {
        #[cfg(feature = "gtk")]
        impl ::fonts_rs_model::HtmlExport for $marker {}
    };
    (@capability $marker:ident $unknown:ident) => {
        ::core::compile_error!(concat!(
            "impl_font_family!: unknown capability `",
            stringify!($unknown),
            "` — expected one of: icon_theme, pango, html"
        ));
    };

    (@forward $marker:ident icon_theme) => {
        #[cfg(feature = "gtk")]
        impl $marker {
            /// Registers the glyph GResource and adds the family's icon
            /// resource path to the `GtkIconTheme` of `display`.
            ///
            /// Forwards to [`IconThemeFont::install_icon_theme`](::fonts_rs_model::IconThemeFont::install_icon_theme).
            pub fn install_icon_theme(
                display: &::fonts_rs_model::gtk4::gdk::Display,
            ) -> Result<(), ::fonts_rs_model::FontError> {
                <Self as ::fonts_rs_model::IconThemeFont>::install_icon_theme(display)
            }

            /// The icon theme resource path of this family.
            ///
            /// Forwards to [`IconThemeFont::icon_theme_path`](::fonts_rs_model::IconThemeFont::icon_theme_path).
            pub fn icon_theme_path() -> String {
                <Self as ::fonts_rs_model::IconThemeFont>::icon_theme_path()
            }

            /// Returns the SVG document for one glyph.
            ///
            /// Forwards to [`IconThemeFont::glyph_svg`](::fonts_rs_model::IconThemeFont::glyph_svg).
            pub fn glyph_svg(name: impl AsRef<str>) -> Option<String> {
                <Self as ::fonts_rs_model::IconThemeFont>::glyph_svg(name)
            }

            /// Composes `text` into a single SVG document.
            ///
            /// Forwards to [`IconThemeFont::compose_svg`](::fonts_rs_model::IconThemeFont::compose_svg).
            pub fn compose_svg(text: &str, glyph_height: f64) -> String {
                <Self as ::fonts_rs_model::IconThemeFont>::compose_svg(text, glyph_height)
            }
        }
    };
    (@forward $marker:ident pango) => {
        #[cfg(feature = "gtk")]
        impl $marker {
            /// Loads all TTFs of the family into `font_map`.
            ///
            /// Forwards to [`PangoFont::install_pango`](::fonts_rs_model::PangoFont::install_pango).
            pub fn install_pango(font_map: &::fonts_rs_model::pango::FontMap) -> Result<(), ::fonts_rs_model::FontError> {
                <Self as ::fonts_rs_model::PangoFont>::install_pango(font_map)
            }
        }
    };
    (@forward $marker:ident html) => {
        #[cfg(feature = "gtk")]
        impl $marker {
            /// Returns a `@font-face` CSS rule embedding the
            /// default-variant TTF as a base64 data URI.
            ///
            /// Forwards to [`HtmlExport::css_font_face`](::fonts_rs_model::HtmlExport::css_font_face).
            pub fn css_font_face() -> Result<String, ::fonts_rs_model::FontError> {
                <Self as ::fonts_rs_model::HtmlExport>::css_font_face()
            }

            /// Starts an `HtmlBuilder` for exporting a self-contained
            /// HTML page.
            ///
            /// Forwards to [`HtmlExport::html`](::fonts_rs_model::HtmlExport::html).
            pub fn html() -> ::fonts_rs_model::HtmlBuilder<Self> {
                <Self as ::fonts_rs_model::HtmlExport>::html()
            }
        }
    };
    (@forward $marker:ident $unknown:ident) => {};
}
