# Software Rendering

The `drawing` module (enabled with the `render` feature) provides functions for
rendering Nerd Font icons and text onto raw RGBA pixel buffers using `ab_glyph`.

## Pixel Buffer Format

All drawing functions operate on a flat `&mut [u8]` buffer in RGBA format (4 bytes
per pixel, row-major order). The buffer dimensions are specified as `width` and
`height` parameters.

## Drawing Functions

### Background

```rust
use nerd_fonts_gtk::drawing::fill_background;

let mut pixels = vec![0u8; 64 * 64 * 4];
fill_background(&mut pixels, 64, 64, [30, 30, 30, 255]);
```

### Nerd Font Icons

```rust
use nerd_fonts_gtk::drawing::draw_nerd_font_icon;

draw_nerd_font_icon(
    &mut pixels, 64, 64,
    "nf-fa-gamepad",
    true,  // is_active (affects default color)
    nerd_fonts_gtk::resolve_icon_codepoint,
    Some([100, 180, 255, 255]),  // custom color
);
```

If the font or icon is unavailable, a circular placeholder is drawn instead.

### Text Labels

```rust
use nerd_fonts_gtk::drawing::draw_label_text;

draw_label_text(
    &mut pixels, 64, 64,
    "Settings",
    true,
    None,  // use default text color
);
```

Text is automatically truncated to fit within 90% of the image width.

### Centered Text

```rust
use nerd_fonts_gtk::drawing::draw_text_centered;

draw_text_centered(
    &mut pixels, 128, 64,
    "Hello World",
    32.0,  // baseline y position
    16.0,  // font size
    [240, 240, 240, 255],
);
```

### Progress Bar

```rust
use nerd_fonts_gtk::drawing::draw_progress_bar;

draw_progress_bar(&mut pixels, 64, 64, 0.7, [100, 180, 255, 255]);
```

Draws a 4px high bar at the bottom of the image. The filled portion uses the
specified color; the unfilled portion uses a darkened version.

### Icon Grid

```rust
use nerd_fonts_gtk::drawing::draw_icon_grid;

let icons = ["nf-fa-gamepad", "nf-md-cube", "nf-linux-tux", "nf-fa-home"];
draw_icon_grid(
    &mut pixels, 128, 64,
    &icons,
    2,  // grid columns
    true,
    nerd_fonts_gtk::resolve_icon_codepoint,
);
```

### Custom Position

```rust
use nerd_fonts_gtk::drawing::draw_nerd_font_codepoint;

draw_nerd_font_codepoint(
    &mut pixels, 128, 64,
    '\u{F11B}',  // gamepad codepoint
    32.0,  // center x
    32.0,  // center y
    24.0,  // icon size
    [100, 180, 255, 255],
);
```

## Alpha Blending

All glyph drawing uses alpha blending. The `blend_pixel` function performs
straight-alpha over compositing, correctly blending the source color over the
existing destination pixel.

## Fallbacks

When no font is available, the `draw_text_bitmap` function provides a simple
bitmap text renderer. This is a last-resort fallback that draws filled rectangles
for each character.
