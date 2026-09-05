# GTK Integration

The `gtk` module (enabled with the `gtk` feature) provides GTK4-specific
functionality for icon resolution and color application.

## Initialization

Call `init()` once at application startup:

```rust
use nerd_fonts_gtk::init;

fn main() {
    init(None);
    // ...
}
```

`init()` performs the following:

1. **Font initialization** (with `render` feature) - Sets the font search base
   directory
2. **GResource registration** - Registers the compiled GResource binary
3. **Icon registration** - Registers `nerd_gtk_icons` SVG icons
4. **CSS loading** - Loads `FONT_FACE_CSS` into a `CssProvider` and adds it to
   the default display

## Icon Color Application

### Image Icons

```rust
use nerd_fonts_gtk::gtk::apply_icon_color;
use nerd_fonts_gtk::Color;

let icon = gtk4::Image::new();
apply_icon_color(&icon, Color::new(1.0, 0.0, 0.0)); // Red
```

This adds a unique CSS class to the icon widget and loads a display-scoped
`CssProvider` targeting only that class. This follows the GTK 4.10 recommendation
to avoid widget-scoped `StyleContext::add_provider` (deprecated since 4.10).

### Label Text Color

```rust
use nerd_fonts_gtk::gtk::apply_text_color;
use nerd_fonts_gtk::Color;

let label = gtk4::Label::new(Some("Status"));

// Apply a color
apply_text_color(&label, Some(Color::new(1.0, 0.5, 0.0)));

// Reset to default (remove custom color)
apply_text_color::<Color>(&label, None);
```

On each call, all previously applied `text-color-*` CSS classes are removed
before the new class is added. This prevents CSS class accumulation across
repeated `update_ui()` calls.

## GTK Icon Name Resolution

```rust
use nerd_fonts_gtk::gtk::resolve_gtk_nerd_icon;

let icon_name = resolve_gtk_nerd_icon("nf-fa-gamepad");
// Returns "nf-fa-gamepad-symbolic"

let image = gtk4::Image::from_icon_name(&icon_name.unwrap());
```

The function normalizes CSS class names (like `nf-fa-gamepad` or `fa-gamepad`)
into GTK icon names that `gtk4::Image::from_icon_name` understands.
