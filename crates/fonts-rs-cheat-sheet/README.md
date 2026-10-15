# fonts-rs-cheat-sheet

[![crates.io](https://img.shields.io/crates/v/fonts-rs-cheat-sheet.svg)](https://crates.io/crates/fonts-rs-cheat-sheet)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Generic GTK 4 cheat sheet application shared by all `fonts-rs` font
families - icon grid, incremental search, navigation history and a
detail sidebar, driven by one trait implementation per family.

## Overview

Family crates supply their data through [`CheatSheetFamily`]; `run()`
builds the complete UI:

```rust
use fonts_rs_cheat_sheet::CheatSheetFamily;

struct MyFontSheet;

impl CheatSheetFamily for MyFontSheet {
    const TITLE: &'static str = "My Font - Cheat Sheet";
    const NOUN: &'static str = "icons";
    const APP_ID: &'static str = "io.example.my_font_cheat_sheet";
    const SEARCH_PLACEHOLDER: &'static str = "Search...";

    type Name = String;
    // ...init, all_glyphs, codepoint, parse_name, display_name,
    // subtitle, search, keywords, categories, glyph_svg
}

fn main() -> miette::Result<gtk4::glib::ExitCode> {
    fonts_rs_cheat_sheet::run::<MyFontSheet>()
}
```

Used by `nerd-fonts-cheat-sheet`, `fonts-rs-blobmoji-cheat-sheet`,
`fonts-rs-noto-emoji-cheat-sheet` and the other family binaries.

## The `ordering()` hook

Families with an upstream-curated glyph order (e.g. emoji picker
ordering from `googlefonts/emoji-metadata`) return their
[`GlyphOrdering`](fonts_rs_model::GlyphOrdering) implementation from
`CheatSheetFamily::ordering()`. `None` (the default) keeps the plain
alphabetical grid; icon families pay nothing.

Opting in enables:

- **Picker-order grid sort** via `order_index` - search results and the
  browse grid follow the curated order.
- **Group jump-list dropdown** plus a **group label** in the detail
  sidebar via `group_labels`/`group`.
- **Variant collapsing** via `collapse_variants`/`base_of` - families
  opt in to hide variants behind their base in the browse grid (emoji
  skin tones do; style families do not - a whole style is first-class
  content). Pairs stay linked through the sidebar's **"Variants"
  section** / **"Variant of"** back-link via `alternates`/`base_of`.
- **Default-tone picking** via `tone_group` - a variant detail offers
  "Set as default tone"; the chosen alternate index is stored per tone
  group (`TonePicker`) and `render_grid` swaps every base cell in the
  group to that tone. Session-scoped; GSettings persistence is a
  follow-up.

Glyph names in `GlyphOrdering` tables must match the names produced by
`all_glyphs()` - the generator joins sequences onto the exported name
set at build time, so this holds automatically for `fonts-rs-generator`
families.

## License

MIT.
