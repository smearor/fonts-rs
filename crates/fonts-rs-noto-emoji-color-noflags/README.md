# fonts-rs-noto-emoji-color-noflags

Noto Emoji `color-noflags` variant (`Noto-COLRv1-noflags`, flag emoji
filtered out) integration for GTK 4 and pixel-drawing.

Provides glyph name resolution with CLDR-based semantic names, GResource
registration with full-color pre-rendered SVG artwork, and keyword/
category metadata for search. The bundled COLRv1 TTF is used for
`pixel-drawing` (`render`), but its `glyf` outlines are empty - use the
`mono` or `bitmap-*` variants for visible `ab_glyph` output
(`RENDERABLE` is `false` here).

Part of the Noto Emoji variant matrix - see `fonts-rs-noto-emoji` for
the feature-selected umbrella crate.

## Quick Start

```rust,ignore
// Register GResource + font (gtk feature)
fonts_rs_noto_emoji_color_noflags::NotoEmoji::install()?;

// Resolve a codepoint
let cp = fonts_rs_noto_emoji_color_noflags::NotoEmoji::codepoint("grinning-face");
```

## Features

- `gtk` (default) - GTK 4 icon theme + GResource integration
- `render` - `ab_glyph` font loading (COLRv1 renders no pixels)
- `embed-fonts` - embed the TTF via `include_bytes!`
- `metadata` - CLDR keywords, categories, shortcodes and picker ordering

## License

The crate code is MIT-licensed; the bundled font is licensed under the
SIL Open Font License 1.1 (`resources/OFL.txt`).
