# CSS Generation

The `css` module provides CSS string constants for GTK and web usage.

## GTK Font-Face CSS

The `FONT_FACE_CSS` constant contains `@font-face` rules for Nerd Font symbol
fonts and `.nerd-icon` helper classes. It is loaded automatically by `init()`
when the `gtk` feature is enabled.

```rust
use nerd_fonts_gtk::css::FONT_FACE_CSS;

// The CSS references GResource URLs:
// resource:///io/smearor/nerd_fonts/SymbolsNerdFont-Regular.ttf
// resource:///io/smearor/nerd_fonts/SymbolsNerdFontMono-Regular.ttf
```

The CSS defines:

- `@font-face` for `NerdFontsSymbolsOnly` (proportional)
- `@font-face` for `NerdFontsSymbolsOnlyMono` (monospace)
- `.nerd-icon` class for proportional icons
- `.nerd-icon-mono` class for monospace icons

## GResource Prefix

The `GRESOURCE_PREFIX` constant defines where font files are registered in the
GResource system:

```rust
use nerd_fonts_gtk::css::GRESOURCE_PREFIX;

assert_eq!(GRESOURCE_PREFIX, "/io/smearor/nerd_fonts");
```

## Manual CSS Loading

If you need to load the CSS manually (e.g., in a custom initialization flow):

```rust
use gtk4::CssProvider;

let provider = gtk4::CssProvider::new();
provider.load_from_data(nerd_fonts_gtk::css::FONT_FACE_CSS);

if let Some(display) = gtk4::gdk::Display::default() {
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
```
