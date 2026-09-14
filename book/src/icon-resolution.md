# Glyph Resolution

Each font family crate provides glyph name resolution - mapping human-readable
glyph names to Unicode codepoints and vice versa.

## Font Family Crates

Font family crates use `GlyphName<F>` (phantom-typed) for type-safe glyph
name resolution. Each crate defines a `GlyphNameExt` trait with a
`codepoint()` method:

```rust
use fonts_rs_doto::GlyphNameExt;
use fonts_rs_doto::DotoName;

// Look up a glyph by name
let name = DotoName::new("doto-a".to_string());
if let Some(codepoint) = name.codepoint() {
    println!("Glyph codepoint: U+{:04X}", codepoint as u32);
}
```

The `codepoint()` method looks up the glyph name in the generated
`phf::Map` (`REVERSE_GLYPHS`), trying both the bare name and the
prefixed name (`{GLYPH_PREFIX}-{name}`).

### Iterating All Glyphs

```rust
use fonts_rs_doto::all_glyphs;

for (codepoint, name) in all_glyphs() {
    println!("U+{:04X} → {}", codepoint as u32, name);
}
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

The `resolve_icon_codepoint` function:

1. Normalizes the input to kebab-case, lower-case
2. Appends `-symbolic` suffix if not already present (GTK symbolic icon convention)
3. Looks up the normalized name in the vendored codepoint map
4. Returns the Unicode character if found

### Usage

```rust
use nerd_fonts_rs::resolve_icon_codepoint;

// Basic resolution
let codepoint = resolve_icon_codepoint("nf-fa-gamepad");
assert_eq!(codepoint, Some('\u{F11B}'));

// Underscores are normalized to hyphens
let codepoint = resolve_icon_codepoint("nf_linux_tux");
assert_eq!(codepoint, Some('\u{F31A}'));

// Case-insensitive
let codepoint = resolve_icon_codepoint("NF-FA-GAMEPAD");
assert_eq!(codepoint, Some('\u{F11B}'));

// Already has -symbolic suffix
let codepoint = resolve_icon_codepoint("nf-fa-gamepad-symbolic");
assert_eq!(codepoint, Some('\u{F11B}'));

// Unknown icon returns None
let codepoint = resolve_icon_codepoint("nf-nonexistent-icon-xyz");
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
