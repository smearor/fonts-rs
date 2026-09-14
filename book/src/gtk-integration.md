# GTK Integration

## Font Family Crates

Each font family crate provides a `register_glyphs()` function (enabled with
the `gtk` feature) that registers the compiled GResource bundle:

```rust
use fonts_rs_doto::register_glyphs;

fn main() {
    // Register GResource (call once at startup)
    register_glyphs().unwrap();

    // Now GTK can resolve icon names from the GResource bundle
    let image = gtk4::Image::from_icon_name("doto-a");
}
```

`register_glyphs()` calls `gio::resources_register_include!("icons.gresource")`
to register the compiled GResource binary containing all SVG glyph files.

## Nerd Fonts (nerd-fonts-rs)

The `nerd-fonts-rs` crate provides additional GTK4 functionality:

### Initialization

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

### GTK Icon Name Resolution

```rust
use nerd_fonts_rs::gtk::resolve_gtk_nerd_icon;

let icon_name = resolve_gtk_nerd_icon("nf-fa-gamepad");
// Returns "nf-fa-gamepad-symbolic"

let image = gtk4::Image::from_icon_name(&icon_name.unwrap());
```

The function normalizes CSS class names (like `nf-fa-gamepad` or `fa-gamepad`)
into GTK icon names that `gtk4::Image::from_icon_name` understands.
