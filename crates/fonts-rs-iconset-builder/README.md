# fonts-rs-iconset-builder

[![crates.io](https://img.shields.io/crates/v/fonts-rs-iconset-builder.svg)](https://crates.io/crates/fonts-rs-iconset-builder)
[![Rust Edition 2024](https://img.shields.io/badge/Rust_Edition-2024-orange?logo=rust&logoColor=white)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Cross-family glyph picker that emits a spec-conformant `icons.toml`
for `IconSetBuild` — browse all icon-capable fonts-rs families in one
combined grid, pin what you need, rename for your app's vocabulary,
save the spec.

```sh
cargo install fonts-rs-iconset-builder
fonts-rs-iconset-builder
```

- **Combined browser** — one grid over Lucide, Tabler, Heroicons, Eva,
  Fluent and Line Awesome; family filter chips + shared search.
- **Selection workflow** — toggle cells to pick, sidebar groups picks
  by source crate, per-entry rename editing, live exported-name
  collision detection (the generator's `IconNameCollision` rule at
  authoring time).
- **Save** — writes `icons.toml`; **Copy scaffold** puts the matching
  `Cargo.toml`/`build.rs`/module stub on the clipboard.
- **Preview** — runs `IconSetBuild::preview` in-process over a temp dir
  (the real pipeline: filters, renames, hashed PUA, post-processors)
  and shows the exported names, codepoints and SVGs. Crate sources
  resolve via the `cargo metadata` fallback — run inside a workspace
  that can see the source crates.

The builder writes configuration, never vendored assets — the target
app's `build.rs` consumes `icons.toml` on its next build and the icons
land embedded in *its* binary.

## License

MIT.
