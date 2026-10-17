//! The [`FamilyAdapter`] bridge - static family → dyn surface.

use crate::dyn_family::DynCheatSheetFamily;
use crate::family::CheatSheetFamily;
use crate::family_kind::FamilyKind;
use fonts_rs_model::CodePoint;
use fonts_rs_model::GlyphOrdering;
use std::marker::PhantomData;

/// Bridges a static [`CheatSheetFamily`] into [`DynCheatSheetFamily`].
///
/// `Name` is erased to `String`: methods receiving `&str` re-enter the
/// typed world through `F::parse_name`. A name the family does not know
/// degrades gracefully (`None`, or the input string for
/// `display_name`) - never panics.
pub struct FamilyAdapter<F: CheatSheetFamily>(PhantomData<F>);

impl<F: CheatSheetFamily> FamilyAdapter<F> {
    /// Creates the adapter for `F`.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<F: CheatSheetFamily> Default for FamilyAdapter<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: CheatSheetFamily> DynCheatSheetFamily for FamilyAdapter<F> {
    fn title(&self) -> &'static str {
        F::TITLE
    }

    fn noun(&self) -> &'static str {
        F::NOUN
    }

    fn app_id(&self) -> &'static str {
        F::APP_ID
    }

    fn search_placeholder(&self) -> &'static str {
        F::SEARCH_PLACEHOLDER
    }

    fn glyph_css_class(&self) -> Option<&'static str> {
        F::GLYPH_CSS_CLASS
    }

    fn name_field_label(&self) -> &'static str {
        F::NAME_FIELD_LABEL
    }

    fn family_name(&self) -> &'static str {
        F::FAMILY_NAME
    }

    fn family_kind(&self) -> FamilyKind {
        F::FAMILY_KIND
    }

    fn license(&self) -> &'static str {
        F::LICENSE
    }

    fn init(&self) -> miette::Result<()> {
        F::init()
    }

    fn all_glyphs(&self) -> Vec<(CodePoint, String)> {
        F::all_glyphs()
            .into_iter()
            .map(|(codepoint, name)| (codepoint, name.as_ref().to_string()))
            .collect()
    }

    fn codepoint(&self, name: &str) -> Option<CodePoint> {
        F::codepoint(name)
    }

    fn parse_name(&self, name: &str) -> Option<String> {
        F::parse_name(name).map(|parsed| parsed.as_ref().to_string())
    }

    fn display_name(&self, name: &str) -> String {
        F::parse_name(name).map(|parsed| F::display_name(&parsed)).unwrap_or_else(|| name.to_string())
    }

    fn subtitle(&self, name: &str) -> String {
        F::parse_name(name)
            .map(|parsed| F::subtitle(&parsed))
            .unwrap_or_else(|| self.family_name().to_string())
    }

    fn search(&self, query: &str) -> Vec<String> {
        F::search(query)
    }

    fn keywords(&self, name: &str) -> Vec<String> {
        F::parse_name(name).map(|parsed| F::keywords(&parsed)).unwrap_or_default()
    }

    fn categories(&self, name: &str) -> Vec<String> {
        F::parse_name(name).map(|parsed| F::categories(&parsed)).unwrap_or_default()
    }

    fn alias_display(&self, name: &str) -> Option<String> {
        F::parse_name(name).and_then(|parsed| F::alias_display(&parsed))
    }

    fn ordering(&self) -> Option<&'static dyn GlyphOrdering> {
        F::ordering()
    }

    fn glyph_icon_name(&self, name: &str) -> Option<String> {
        F::parse_name(name).and_then(|parsed| F::glyph_icon_name(&parsed))
    }

    fn glyph_svg(&self, name: &str) -> Option<String> {
        F::parse_name(name).and_then(|parsed| F::glyph_svg(&parsed))
    }
}
