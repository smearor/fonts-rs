# CSS Generation

The `css` module provides CSS string constants for GTK and web usage.

## GTK Font-Face CSS

The `font_face_css()` function generates a version-adapted `@font-face` CSS
string at runtime. It detects the GTK4 library version and adjusts the `src`
descriptor syntax for compatibility:

- **GTK >= 4.10**: emits `src: url("...") format("truetype")` for better spec
  compliance with the new GtkCssParser.
- **GTK < 4.10**: emits `src: url("...")` without a `format()` hint to avoid
  parse errors with the legacy parser.

It is loaded automatically by `init()` when the `gtk` feature is enabled.

```rust
use nerd_fonts_rs::css::font_face_css;

let css = font_face_css();
// The CSS references GResource URLs:
// resource:///io/smearor/nerd_fonts/SymbolsNerdFont-Regular.ttf
// resource:///io/smearor/nerd_fonts/SymbolsNerdFontMono-Regular.ttf
```

The CSS defines:

- `@font-face` for `NerdFontsSymbolsOnly` (proportional)
- `@font-face` for `NerdFontsSymbolsOnlyMono` (monospace)
- `.nerd-icon` class for proportional icons
- `.nerd-icon-mono` class for monospace icons

The static `FONT_FACE_CSS` constant retains the baseline CSS (without
`format()` hints) for backward compatibility and testing.

## Runtime Version Detection

The `GtkVersion` struct wraps the runtime GTK4 version and provides
comparison helpers:

```rust
use nerd_fonts_rs::css::GtkVersion;

#[cfg(feature = "gtk")]
let version = GtkVersion::runtime();

let legacy = GtkVersion::new(4, 8, 0);
assert!(!legacy.at_least(4, 10));

let modern = GtkVersion::new(4, 14, 2);
assert!(modern.at_least(4, 10));
```

## GResource Prefix

The `GRESOURCE_PREFIX` constant defines where font files are registered in the
GResource system:

```rust
use nerd_fonts_rs::css::GRESOURCE_PREFIX;

assert_eq!(GRESOURCE_PREFIX, "/io/smearor/nerd_fonts");
```

## Manual CSS Loading

If you need to load the CSS manually (e.g., in a custom initialization flow):

```rust
use gtk4::CssProvider;

let provider = gtk4::CssProvider::new();
let css = nerd_fonts_rs::css::font_face_css();

#[cfg(feature = "v4_12")]
provider.load_from_string(&css);
#[cfg(not(feature = "v4_12"))]
{
    #[allow(deprecated)]
    provider.load_from_data(&css);
}

if let Some(display) = gtk4::gdk::Display::default() {
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
```
