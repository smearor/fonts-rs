//! CSS string constants for GTK and web usage.
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

pub use crate::icons::paths::GRESOURCE_PREFIX;

/// GTK CSS: `@font-face` rules for Nerd Font symbol fonts and `.nerd-icon` helper classes.
///
/// Load this into a `CssProvider` at startup (done automatically by `init()` with the `gtk` feature).
pub const FONT_FACE_CSS: &str = include_str!("../resources/font-face.css");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gresource_prefix_is_correct() {
        assert_eq!(GRESOURCE_PREFIX, "/io/smearor/nerd_fonts");
    }

    #[test]
    fn font_face_css_contains_font_family() {
        assert!(FONT_FACE_CSS.contains("NerdFontsSymbolsOnly"));
        assert!(FONT_FACE_CSS.contains("NerdFontsSymbolsOnlyMono"));
    }

    #[test]
    fn font_face_css_contains_gresource_url() {
        assert!(FONT_FACE_CSS.contains("resource:///io/smearor/nerd_fonts/SymbolsNerdFont-Regular.ttf"));
        assert!(FONT_FACE_CSS.contains("resource:///io/smearor/nerd_fonts/SymbolsNerdFontMono-Regular.ttf"));
    }

    #[test]
    fn font_face_css_contains_nerd_icon_class() {
        assert!(FONT_FACE_CSS.contains(".nerd-icon"));
        assert!(FONT_FACE_CSS.contains(".nerd-icon-mono"));
    }
}
