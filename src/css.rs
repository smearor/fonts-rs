//! CSS string constants for GTK and web usage.
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

/// GResource prefix for font files.
pub const GRESOURCE_PREFIX: &str = "/io/smearor/nerd_fonts";

/// GTK CSS: `@font-face` rules for Nerd Font symbol fonts and `.nerd-icon` helper classes.
///
/// Load this into a `CssProvider` at startup (done automatically by `init()` with the `gtk` feature).
pub const FONT_FACE_CSS: &str = include_str!("../resources/font-face.css");
