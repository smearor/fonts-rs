# GTK Integration

## Font Family Crates

Each font family marker type provides an `install()` method (enabled with
the `gtk` feature) that registers the compiled GResource bundle and loads
the font:

```rust
use fonts_rs_doto::Doto;

fn main() {
    // Register GResource + load font (call once at startup)
    Doto::install().unwrap();

    // Now GTK can resolve icon names from the GResource bundle
    let image = gtk4::Image::from_icon_name("doto-a");
}
```

`install()` registers the compiled GResource binary containing all SVG
glyph files and loads the TTF into the fontconfig/pango font map.
Registration is lazy and idempotent - `ensure_registered()` is called
automatically by glyph accessors, so `install()` is only needed for
explicit fail-fast setup at startup.

## Nerd Fonts (nerd-fonts-rs)

The `nerd-fonts-rs` crate provides additional GTK4 functionality:

### Initialization

Call `init()` once at application startup:

```rust
fn main() {
    nerd_fonts_rs::init().unwrap();
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

### GTK Icon Name Resolution

```rust
use nerd_fonts_rs::gtk::resolve_gtk_nerd_icon;

let icon_name = resolve_gtk_nerd_icon("nf-fa-gamepad");
// Returns "nf-fa-gamepad-symbolic"

let image = gtk4::Image::from_icon_name(&icon_name.unwrap());
```

The function normalizes CSS class names (like `nf-fa-gamepad` or `fa-gamepad`)
into GTK icon names that `gtk4::Image::from_icon_name` understands.
