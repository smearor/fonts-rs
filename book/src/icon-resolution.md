# Icon Resolution

The `icons` module maps human-readable Nerd Font icon names to their Unicode
codepoints.

## How It Works

Icon names follow the pattern `nf-{prefix}-{name}`, for example:

- `nf-fa-gamepad` - Font Awesome gamepad icon
- `nf-md-cube` - Material Design cube icon
- `nf-linux-tux` - Linux Tux icon

The `resolve_icon_codepoint` function:

1. Normalizes the input to kebab-case, lower-case
2. Appends `-symbolic` suffix if not already present (GTK symbolic icon convention)
3. Looks up the normalized name in the `nerd_gtk_icons` codepoint map
4. Returns the Unicode character if found

## Usage

```rust
use nerd_fonts_gtk::resolve_icon_codepoint;

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

## GTK Icon Name Resolution

The `gtk` module provides `resolve_gtk_nerd_icon` which converts CSS class names
(like `nf-fa-gamepad` or `fa-gamepad`) into GTK icon names that
`gtk4::Image::from_icon_name` understands.

The `nerd_gtk_icons` crate registers SVG icons as GResource under the path
`/io/nerd_fonts/icons/`. Each icon follows the naming pattern
`nf-{prefix}-{name}-symbolic` (kebab-case, lower-case).

```rust
use nerd_fonts_gtk::gtk::resolve_gtk_nerd_icon;

let icon_name = resolve_gtk_nerd_icon("nf-fa-gamepad");
// Returns "nf-fa-gamepad-symbolic"
```
