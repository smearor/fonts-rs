//! Web CSS generation for web instances.
//!
//! Provides the generated `nerdfont.css` with per-icon `content: "\XXXX"` mappings
//! for use in web-based widget rendering.
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

/// Web CSS with `@font-face` and per-icon `content: "\XXXX"` mappings.
///
/// Generated from `SymbolsNerdFont-Regular.ttf` glyph table.
/// Include this in web instance HTML pages.
pub const WEB_NERDFONT_CSS: &str = include_str!("../resources/web/nerdfont.css");
