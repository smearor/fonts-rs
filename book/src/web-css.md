# Web CSS

The `web` module (enabled with the `web` feature) provides a CSS constant for
web-based widget rendering.

## WEB_NERDFONT_CSS

The `WEB_NERDFONT_CSS` constant contains a complete CSS stylesheet with:

- `@font-face` rules for loading Nerd Font symbol fonts
- Per-icon `content: "\XXXX"` mappings for use in web widgets

```rust
use nerd_fonts_rs::web::WEB_NERDFONT_CSS;

// Include in your web HTML
let html = format!(
    "<style>{}</style><span class='nf-fa-gamepad'></span>",
    WEB_NERDFONT_CSS
);
```

## Usage with Web Instances

The web CSS is designed for web-based widget rendering systems that display
Nerd Font icons in HTML pages. Each icon class maps to its Unicode codepoint
via the CSS `content` property.

### Example HTML

```html
<link rel="stylesheet" href="nerdfont.css">
<span class="nerd-icon nf-fa-gamepad"></span>
```

The CSS is generated from the `SymbolsNerdFont-Regular.ttf` glyph table and
includes mappings for all Nerd Font symbols.
