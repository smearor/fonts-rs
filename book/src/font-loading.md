# Font Loading

The `fonts` module (enabled with the `render` feature) handles loading font
files for use with [`pixel-drawing`](https://github.com/smearor/pixel-drawing)
or other software rendering via `ab_glyph`.

## `impl_font_loader!` Macro

Each font family crate uses the `impl_font_loader!` macro from
`fonts-rs-generator` to generate a cached font loading function:

```rust
// Literal: relative to resources/
fonts_rs_generator::impl_font_loader!("Doto.ttf");

// Env var: absolute path set by build.rs via cargo:rustc-env
fonts_rs_generator::impl_font_loader!(env: "DOTO_FONT_PATH");
```

The macro generates a `font() -> Option<&'static ab_glyph::FontVec>` function
that loads the TTF font file and caches it in a `OnceLock`. Subsequent calls
return a reference to the cached `FontVec`.

## Loading Modes

### Embedded Fonts (`embed-fonts` feature)

Font files are compiled into the binary via `include_bytes!`:

```rust
// No runtime file access needed
fonts_rs_doto::fonts::font(); // Returns Option<&'static FontVec>
```

### From Disk (default)

Fonts are loaded from a path set by `build.rs` via `cargo:rustc-env`.
The `impl_font_loader!(env: ...)` variant reads the path from the
environment variable at runtime.

## Caching

Fonts are loaded once and cached in a `OnceLock`. Subsequent calls return
a reference to the cached `FontVec`:

```rust
use fonts_rs_doto::fonts::font;

let font1 = font().unwrap();
let font2 = font().unwrap();
assert!(core::ptr::eq(font1 as *const _, font2 as *const _));
```

## Nerd Fonts (nerd-fonts-rs)

The `nerd-fonts-rs` crate provides additional font loading for Nerd Fonts
specifically:

1. **Nerd Font** (`SymbolsNerdFont-Regular.ttf`) - TTF file for rendering
   Nerd Font icons
2. **Label Font** (`JetBrainsMonoNLNerdFont-Regular.woff2`) - WOFF2 file
   decompressed to TTF at load time for text labels

The label font is stored as WOFF2. The `convert_woff2` function decompresses
it to TTF using the `woff2-patched` crate. If decompression fails, it falls
back to parsing the data directly as TTF.

### Initialization

For `nerd-fonts-rs`, call `init()` once at application startup:

```rust
// Use default base directory (relative to executable)
nerd_fonts_rs::init(None);

// Or specify a custom base directory
nerd_fonts_rs::init(Some("/usr/share/fonts/nerd-fonts"));
```
