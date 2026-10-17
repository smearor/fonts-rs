//! The [`IconSetPostProcessor`] enum - serde-selectable SVG post-processors.

use serde::Deserialize;
use serde::Serialize;

/// A post-processor selectable per `[[iconset.sources]]` table via its
/// kebab-case tag, e.g. `post_processors = ["symbolic-classes"]`.
///
/// Maps to the `SvgPostProcessor` implementations in
/// `fonts-rs-generator`; the wiring lives there (this type is pure
/// data - the model crate cannot depend on the generator).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum IconSetPostProcessor {
    /// `symbolic-classes` - adds GTK symbolic-recoloring classes
    /// (`SvgPostProcessor::SymbolicClasses`).
    SymbolicClasses,
    /// `current-color-fill` - forces fill attributes to `currentColor`.
    CurrentColorFill,
    /// `current-color-fill-inject` - like `current-color-fill`, plus
    /// injects `fill="currentColor"` where no fill attribute exists.
    CurrentColorFillInject,
    /// `current-color-stroke` - forces stroke attributes to
    /// `currentColor`.
    CurrentColorStroke,
    /// `optimize` - strips comments and redundant whitespace.
    Optimize,
    /// `strip-entity-namespaces` - removes `xmlns:entity`-style
    /// namespaces some editors emit.
    StripEntityNamespaces,
}
