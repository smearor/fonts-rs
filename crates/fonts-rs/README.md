# fonts-rs

[![crates.io](https://img.shields.io/crates/v/fonts-rs.svg)](https://crates.io/crates/fonts-rs)
[![Rust Edition 2024](https://img.shields.io/badge/Rust_Edition-2024-orange?logo=rust&logoColor=white)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Facade crate for the **fonts-rs** workspace — one dependency,
feature-selected font families.

```toml
[dependencies]
fonts-rs = { version = "0.5", features = ["lucide", "doto"] }
```

```rust,no_run
fn main() -> miette::Result<()> {
    fonts_rs::lucide::Lucide::install()?;
    let icon = gtk4::Image::from_icon_name("lucide-check-symbolic");
    Ok(())
}
```

Every enabled family is reachable under its own module alias
(`fonts_rs::lucide::…`), and its marker type plus name alias are
additionally re-exported at the crate root (`fonts_rs::Lucide`,
`fonts_rs::LucideIconName`). `fonts_rs::model` is always available;
`fonts_rs::prelude` is the one-import surface.

## Feature matrix

**Family selectors**

| Feature | Crate |
|---|---|
| `lucide` | `fonts-rs-lucide` |
| `tabler` | `fonts-rs-tabler` |
| `heroicons` | `fonts-rs-heroicons` |
| `eva` | `fonts-rs-eva` |
| `fluent` | `fonts-rs-fluent` |
| `line-awesome` | `fonts-rs-line-awesome` |
| `doto` | `fonts-rs-doto` |
| `seven-segment` | `fonts-rs-seven-segment` |
| `fourteen-segment` | `fonts-rs-fourteen-segment` |
| `barcode-code39` | `fonts-rs-barcode-code39` |
| `barcode-code128` | `fonts-rs-barcode-code128` |
| `barcode-ean13` | `fonts-rs-barcode-ean13` |
| `bravura` | `fonts-rs-bravura` |
| `redacted` | `fonts-rs-redacted` |
| `dicefont` | `fonts-rs-dicefont` |
| `cuernavaca` | `fonts-rs-cuernavaca` |
| `nerd-fonts` | `nerd-fonts-rs` |

**Emoji** — variant features (`noto-emoji-{mono,color-flags,color-noflags,bitmap-flags,bitmap-noflags}`,
`blobmoji-{color-flags,color-noflags,bitmap-flags,bitmap-noflags}`)
plus aliases `noto-emoji`/`blobmoji` for the default `color-flags`
variant. The umbrellas guard exactly one variant — select one, not
several.

**Capabilities** — forwarded to every enabled family that has them:
`gtk` (default), `render`, `metadata`, `embed-fonts`.

**Groups** — `icons`, `emoji`, `segment`, `barcode`, `all`.

`default-features = false` produces a genuinely GTK-free build —
family deps are declared without default features:

```toml
fonts-rs = { version = "0.5", default-features = false, features = ["doto", "render"] }
```

## The `fonts-rs` binary

`cargo install fonts-rs --features gui` installs a multi-family cheat
sheet: one window, every family, a header dropdown switches between
them in place (GResource/font init is lazy per family).

For a subset, pick `gui-core` plus the wanted `sheet-*` bindings:

```sh
cargo install fonts-rs --features "gui-core sheet-lucide sheet-tabler"
```

## License

MIT. Bundled fonts and icon sets ship under their own licenses —
see the individual family crates.
