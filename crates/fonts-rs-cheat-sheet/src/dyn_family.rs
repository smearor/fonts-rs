//! The [`DynCheatSheetFamily`] trait - object-safe runtime surface.

use crate::family_kind::FamilyKind;
use fonts_rs_model::CodePoint;
use fonts_rs_model::GlyphOrdering;

/// Object-safe runtime view of a cheat-sheet family - the string-typed
/// counterpart of [`CheatSheetFamily`](crate::CheatSheetFamily) for
/// multi-family hosts (`run_multi`, the `fonts-rs` facade CLI).
///
/// Every member mirrors its `CheatSheetFamily` counterpart; glyph names
/// are carried as `String` instead of the family's typed `Name`. The
/// bridge between the two is [`FamilyAdapter`](crate::FamilyAdapter),
/// which re-enters the typed world via `parse_name`.
///
/// Concrete `XSheet` types live downstream of this crate, so the
/// multi-family host cannot dispatch over a closed enum - the object
/// surface is the shared point both paths use.
pub trait DynCheatSheetFamily {
    /// Window title and header text, e.g. `"Nerd Fonts - Cheat Sheet"`.
    fn title(&self) -> &'static str;

    /// Noun used in count labels, e.g. `"icons"` or `"emoji"`.
    fn noun(&self) -> &'static str;

    /// GTK application ID for single-family runs
    /// (`run::<F>()` keeps the family's own ID).
    fn app_id(&self) -> &'static str;

    /// Search entry placeholder text.
    fn search_placeholder(&self) -> &'static str;

    /// CSS class applied to glyph preview labels so they render in the
    /// family's font. `None` for families whose glyphs render via the
    /// system font.
    fn glyph_css_class(&self) -> Option<&'static str> {
        None
    }

    /// Label of the name field in the detail sidebar.
    fn name_field_label(&self) -> &'static str {
        "Glyph Name"
    }

    /// Display name of the font or icon set itself - shown in the
    /// "Family" section of the detail sidebar.
    fn family_name(&self) -> &'static str;

    /// What kind of glyph source the family is.
    fn family_kind(&self) -> FamilyKind;

    /// License of the bundled font or icon set; list all options when
    /// dual-licensed.
    fn license(&self) -> &'static str;

    /// Family initialization (GResource registration, font loading,
    /// CSS). Called lazily when a family is first activated in a
    /// multi-family run; must be idempotent.
    ///
    /// # Errors
    ///
    /// Returns an error if registration or font loading fails.
    fn init(&self) -> miette::Result<()>;

    /// All glyphs as `(codepoint, name)` pairs, sorted by name.
    fn all_glyphs(&self) -> Vec<(CodePoint, String)>;

    /// Resolves a name string to its codepoint.
    fn codepoint(&self, name: &str) -> Option<CodePoint>;

    /// Parses a name string into the canonical glyph name string.
    ///
    /// `None` for names the family does not know - callers degrade
    /// gracefully (no detail lookup, no icon-theme preview).
    fn parse_name(&self, name: &str) -> Option<String>;

    /// Display label for grid cells and the detail title.
    fn display_name(&self, name: &str) -> String;

    /// Subtitle in the detail sidebar.
    fn subtitle(&self, name: &str) -> String;

    /// Metadata search: returns the names matching `query`.
    fn search(&self, query: &str) -> Vec<String>;

    /// Keywords for a glyph name (empty without metadata).
    fn keywords(&self, name: &str) -> Vec<String>;

    /// Categories for a glyph name (empty without metadata).
    fn categories(&self, name: &str) -> Vec<String>;

    /// Display string of the canonical name when `name` is an alias.
    /// `None` (the default) hides the alias section.
    fn alias_display(&self, _name: &str) -> Option<String> {
        None
    }

    /// Optional canonical ordering contract for the family.
    fn ordering(&self) -> Option<&'static dyn GlyphOrdering> {
        None
    }

    /// Icon-theme name for a glyph, rendered via an image widget in
    /// grid cells instead of a text label. `None` (the default)
    /// renders the codepoint character as text.
    fn glyph_icon_name(&self, _name: &str) -> Option<String> {
        None
    }

    /// SVG document for a glyph name, shown in the detail sidebar.
    fn glyph_svg(&self, name: &str) -> Option<String>;
}
