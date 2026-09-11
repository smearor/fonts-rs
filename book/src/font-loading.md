# Font Loading

The `fonts` module (enabled with the `render` feature) handles loading Nerd Font
files for use with [`pixel-drawing`](https://github.com/smearor/pixel-drawing).

## Font Types

Two fonts are loaded:

1. **Nerd Font** (`SymbolsNerdFont-Regular.ttf`) - Used for rendering Nerd Font
   icons. This is a TTF file.
2. **Label Font** (`JetBrainsMonoNLNerdFont-Regular.woff2`) - Used for rendering
   text labels in images. This is a WOFF2 file that is decompressed to TTF at
   load time.

## Loading Modes

### Embedded Fonts (`embed-fonts` feature)

Font files are compiled into the binary via `include_bytes!`:

```rust
// No runtime file access needed
nerd_fonts_rs::fonts::nerd_font(); // Returns &'static FontVec
nerd_fonts_rs::fonts::label_font(); // Returns &'static FontVec
```

### From Disk (default)

Fonts are loaded from a base directory at runtime. Set the base directory via
`init()`:

```rust
// Use default base directory (relative to executable)
nerd_fonts_rs::init(None);

// Or specify a custom base directory
nerd_fonts_rs::init(Some("/usr/share/fonts/nerd-fonts"));
```

The font files are expected at:

- `{base_dir}/resources/NerdFontsSymbolsOnly/SymbolsNerdFont-Regular.ttf`
- `{base_dir}/resources/JetBrainsMonoNLNerdFont/JetBrainsMonoNLNerdFont-Regular.woff2`

## Caching

Both fonts are loaded once and cached in a `OnceLock`. Subsequent calls return
a reference to the cached `FontVec`:

```rust
use nerd_fonts_rs::fonts::nerd_font;

let font1 = nerd_font().unwrap();
let font2 = nerd_font().unwrap();
assert!(core::ptr::eq(font1 as *const _, font2 as *const _));
```

## WOFF2 Decompression

The label font is stored as WOFF2. The `convert_woff2` function decompresses it
to TTF using the `woff2-patched` crate. If decompression fails, it falls back to
parsing the data directly as TTF.
