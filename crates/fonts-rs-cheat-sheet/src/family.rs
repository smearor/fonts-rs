//! Per-family hooks for the generic cheat sheet.

use fonts_rs_model::CodePoint;
use std::fmt::Display;

/// Family-specific data surface for the cheat sheet UI.
///
/// Implement once per font family; [`crate::run`] builds the complete
/// application from these hooks. The associated [`Name`](Self::Name)
/// type is the family's glyph name - `IconName` for icon fonts,
/// `String` or `GlyphName<F>` for others.
///
/// # Examples
///
/// ```ignore
/// struct NotoEmojiSheet;
///
/// impl CheatSheetFamily for NotoEmojiSheet {
///     type Name = String;
///     // ...
/// }
///
/// fn main() -> miette::Result<glib::ExitCode> {
///     fonts_rs_cheat_sheet::run::<NotoEmojiSheet>()
/// }
/// ```
pub trait CheatSheetFamily: 'static {
    /// Window title and header text, e.g. `"Nerd Fonts - Cheat Sheet"`.
    const TITLE: &'static str;

    /// Noun used in count labels, e.g. `"icons"` or `"emoji"`.
    const NOUN: &'static str;

    /// GTK application ID, e.g. `"io.smearor.fonts_rs.cheat_sheet"`.
    const APP_ID: &'static str;

    /// Search entry placeholder text.
    const SEARCH_PLACEHOLDER: &'static str;

    /// CSS class applied to glyph preview labels so they render in the
    /// family's font (e.g. `"nerd-icon"`). `None` for families whose
    /// glyphs render via the system font (emoji).
    const GLYPH_CSS_CLASS: Option<&'static str> = None;

    /// Label of the name field in the detail sidebar
    /// (e.g. `"CSS Class"` for icon fonts, `"Glyph Name"` otherwise).
    const NAME_FIELD_LABEL: &'static str = "Glyph Name";

    /// The glyph name type - `IconName` for icon fonts, `String` or
    /// `GlyphName<F>` for other families.
    type Name: Clone + Ord + AsRef<str> + Display + 'static;

    /// Family initialization, called once in `main` before the
    /// application starts (GResource registration, font loading, CSS).
    ///
    /// # Errors
    ///
    /// Returns an error if registration or font loading fails.
    fn init() -> miette::Result<()>;

    /// All glyphs as `(codepoint, name)` pairs, sorted by name.
    fn all_glyphs() -> Vec<(CodePoint, Self::Name)>;

    /// Resolves a name string to its codepoint.
    fn codepoint(name: &str) -> Option<CodePoint>;

    /// Parses a name string into the typed name.
    fn parse_name(name: &str) -> Option<Self::Name>;

    /// Display label for grid cells and the detail title
    /// (e.g. the CSS class name for icon fonts).
    fn display_name(name: &Self::Name) -> String;

    /// Subtitle in the detail sidebar (e.g. the icon set label, or the
    /// family display name).
    fn subtitle(name: &Self::Name) -> String;

    /// Metadata search: returns the names matching `query`.
    ///
    /// Families without metadata return an empty vec - the generic
    /// supplement still matches names, hex codepoints, and characters.
    fn search(query: &str) -> Vec<String>;

    /// Keywords for a glyph name (empty without metadata).
    fn keywords(name: &Self::Name) -> Vec<String>;

    /// Categories for a glyph name (empty without metadata).
    fn categories(name: &Self::Name) -> Vec<String>;

    /// Display string of the canonical name when `name` is an alias.
    /// `None` (the default) hides the alias section.
    fn alias_display(_name: &Self::Name) -> Option<String> {
        None
    }

    /// Optional canonical ordering contract for the family.
    ///
    /// Families with an upstream-curated glyph order (e.g. emoji picker
    /// ordering) return their [`GlyphOrdering`] implementation here;
    /// `None` (the default) keeps the alphabetical grid. When `Some`,
    /// ordering positions override the grid sort, group labels appear
    /// in the detail sidebar, and a group jump-list is added to the
    /// top bar.
    fn ordering() -> Option<&'static dyn fonts_rs_model::GlyphOrdering> {
        None
    }

    /// Icon-theme name for a glyph, rendered via an image widget in grid
    /// cells instead of a text label.
    ///
    /// `None` (the default) renders the codepoint character as text.
    /// Return `Some` for families that register full-color SVG glyphs in
    /// the `GtkIconTheme` - e.g. emoji fonts whose bundled TTF family
    /// name would otherwise resolve to a system font.
    fn glyph_icon_name(_name: &Self::Name) -> Option<String> {
        None
    }

    /// SVG document for a glyph name, shown in the detail sidebar.
    fn glyph_svg(name: &Self::Name) -> Option<String>;
}
