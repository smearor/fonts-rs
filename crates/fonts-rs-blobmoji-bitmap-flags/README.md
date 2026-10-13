# fonts-rs-blobmoji-bitmap-flags

Blobmoji `bitmap-flags` variant (`NotoColorEmoji`, CBDT bitmap strikes,
including flag emoji) integration for GTK 4 and pixel-drawing.

Provides glyph name resolution with CLDR-based semantic names, GResource
registration with full-color pre-rendered SVG artwork, and keyword/
category metadata for search. The bundled CBDT TTF produces colored
`ab_glyph` output for `pixel-drawing` (`RENDERABLE` is `true`).

Part of the Blobmoji variant matrix - see `fonts-rs-blobmoji` for
the feature-selected umbrella crate.

## Quick Start

```rust,ignore
// Register GResource + font (gtk feature)
fonts_rs_blobmoji_bitmap_flags::Blobmoji::install()?;

// Resolve a codepoint
let cp = fonts_rs_blobmoji_bitmap_flags::Blobmoji::codepoint("grinning-face");
```

## Features

- `gtk` (default) - GTK 4 icon theme + GResource integration
- `render` - `ab_glyph` font loading (CBDT strikes render in color)
- `embed-fonts` - embed the TTF via `include_bytes!`
- `metadata` - CLDR keywords, categories, shortcodes and picker ordering

## License

The crate code is MIT-licensed; the bundled font is licensed under the
SIL Open Font License 1.1 (`resources/OFL.txt`).
