# Glyph Resolution

Each font family crate provides glyph name resolution - mapping human-readable
glyph names to Unicode codepoints and vice versa.

## Font Family Crates

Each font family marker type implements the `FontFamily` trait, which
provides glyph name resolution as associated functions - no trait import
needed thanks to inherent forwarders:

```rust
use fonts_rs_doto::Doto;

// Look up a glyph by name
if let Some(codepoint) = Doto::codepoint("doto-a") {
    println!("Glyph codepoint: U+{:04X}", codepoint.as_char() as u32);
}

// Reverse lookup: codepoint → glyph name
if let Some(name) = Doto::glyph_name('a') {
    println!("Glyph name: {name}");
}
```

`codepoint()` looks up the glyph name in the generated `phf::Map`
(`REVERSE_GLYPHS`), trying both the bare name and the prefixed name
(`{GLYPH_PREFIX}-{name}`).

### Iterating All Glyphs

```rust
use fonts_rs_doto::Doto;

for (codepoint, name) in Doto::all_glyphs() {
    println!("U+{:04X} → {}", codepoint.as_char() as u32, name.as_ref());
}

// O(1) glyph count
let total = Doto::glyph_count();
```

### GResource Path Resolution

Each glyph has a GResource path that can be used with GTK:

```rust
// GResource path format: {GRESOURCE_PREFIX}/scalable/{ICONS_CONTEXT}/{glyph_name}.svg
// e.g. /io/smearor/fonts/doto/scalable/glyphs/doto-a.svg
```

## Nerd Fonts (nerd-fonts-rs)

Nerd Fonts use a different naming convention. Icon names follow the pattern
`nf-{prefix}-{name}`, for example:

- `nf-fa-gamepad` - Font Awesome gamepad icon
- `nf-md-cube` - Material Design cube icon
- `nf-linux-tux` - Linux Tux icon

`NerdFonts::codepoint` accepts all `IconName` forms - it normalizes the
input to kebab-case, lower-case, strips the `nf-` prefix and `-symbolic`
suffix, then looks up the name in the vendored codepoint map.

### Usage

```rust
use nerd_fonts_rs::CodePoint;
use nerd_fonts_rs::NerdFonts;

// Basic resolution
let codepoint = NerdFonts::codepoint("nf-fa-gamepad");
assert_eq!(codepoint, Some(CodePoint::from_char('\u{F11B}')));

// Underscores are normalized to hyphens
let codepoint = NerdFonts::codepoint("nf_linux_tux");
assert_eq!(codepoint, Some(CodePoint::from_char('\u{F31A}')));

// Case-insensitive
let codepoint = NerdFonts::codepoint("NF-FA-GAMEPAD");
assert_eq!(codepoint, Some(CodePoint::from_char('\u{F11B}')));

// Already has -symbolic suffix
let codepoint = NerdFonts::codepoint("nf-fa-gamepad-symbolic");
assert_eq!(codepoint, Some(CodePoint::from_char('\u{F11B}')));

// Unknown icon returns None
let codepoint = NerdFonts::codepoint("nf-nonexistent-icon-xyz");
assert_eq!(codepoint, None);
```

### GTK Icon Name Resolution

The `gtk` module provides `resolve_gtk_nerd_icon` which converts CSS class names
(like `nf-fa-gamepad` or `fa-gamepad`) into GTK icon names that
`gtk4::Image::from_icon_name` understands.

The vendored icon GResource registers SVG icons under the path
`/io/smearor/nerd_fonts/icons/`. Each icon follows the naming pattern
`nf-{prefix}-{name}-symbolic` (kebab-case, lower-case).

```rust
use nerd_fonts_rs::gtk::resolve_gtk_nerd_icon;

let icon_name = resolve_gtk_nerd_icon("nf-fa-gamepad");
// Returns "nf-fa-gamepad-symbolic"
```
