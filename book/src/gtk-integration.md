# GTK Integration

The `gtk` module (enabled with the `gtk` feature) provides GTK4-specific
functionality for icon name resolution.

## Initialization

Call `init()` once at application startup:

```rust
use nerd_fonts_rs::init;

fn main() {
    init(None);
    // ...
}
```

`init()` performs the following:

1. **Font initialization** (with `render` feature) - Sets the font search base
   directory
2. **GResource registration** - Registers the compiled GResource binary
3. **Icon registration** - Registers vendored Nerd Font SVG icons
4. **CSS loading** - Loads version-adapted `@font-face` CSS via
   `font_face_css()` into a `CssProvider` and adds it to the default display

## GTK Icon Name Resolution

```rust
use nerd_fonts_rs::gtk::resolve_gtk_nerd_icon;

let icon_name = resolve_gtk_nerd_icon("nf-fa-gamepad");
// Returns "nf-fa-gamepad-symbolic"

let image = gtk4::Image::from_icon_name(&icon_name.unwrap());
```

The function normalizes CSS class names (like `nf-fa-gamepad` or `fa-gamepad`)
into GTK icon names that `gtk4::Image::from_icon_name` understands.
