# fonts-rs-noto-emoji-mono

Noto Emoji `mono` variant (`NotoEmoji-Regular`) integration for GTK 4
and pixel-drawing.

Provides glyph name resolution with CLDR-based semantic names, GResource
registration with full-color pre-rendered SVG artwork, keyword/category
metadata for search, and `ab_glyph` pixel-drawing of the bundled
monochrome outline TTF.

Part of the Noto Emoji variant matrix - see `fonts-rs-noto-emoji` for
the feature-selected umbrella crate.

## Quick Start

```rust,ignore
// Register GResource + font (gtk feature)
fonts_rs_noto_emoji_mono::NotoEmoji::install()?;

// Resolve a codepoint
let cp = fonts_rs_noto_emoji_mono::NotoEmoji::codepoint("grinning-face");
```

## Features

- `gtk` (default) - GTK 4 icon theme + GResource integration
- `render` - `ab_glyph` pixel-drawing of the bundled TTF (`RENDERABLE`)
- `embed-fonts` - embed the TTF via `include_bytes!`
- `metadata` - CLDR keywords, categories, shortcodes and picker ordering

## License

The crate code is MIT-licensed; the bundled font is licensed under the
SIL Open Font License 1.1 (`resources/OFL.txt`).
