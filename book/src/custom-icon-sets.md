# Custom Icon Sets

Font family crates ship *complete* icon sets — `fonts-rs-lucide`
embeds all ~1,850 icons (~3 MB). An application typically uses a small
subset and should not pay for the rest. Custom icon sets let an
application declare a minimal, application-specific set in a root
`icons.toml` and build it with a one-line `build.rs` — no manual SVG
vendoring, keeping the same type-safe runtime API (`GlyphName`,
`IconThemeFont`, `Image::from_icon_name`).

## The Three-Line Integration

```toml
# Cargo.toml (application)
[build-dependencies]
fonts-rs-generator = "0.6"
miette = "7.6"

[dependencies]
fonts-rs-generator = "0.6"  # for impl_font_family!/impl_gtk_font! in generated code
fonts-rs-model = "0.6"
gio = "0.22"
gtk4 = "0.11"
phf = "0.14"
# Source crates must be *normal* dependencies: links metadata (DEP_*
# env vars) does not propagate via [build-dependencies]. They are
# never used at runtime — their gresource is unreachable and stripped
# by the linker.
fonts-rs-lucide = "0.6"
fonts-rs-tabler = "0.6"

[features]
# The generated module's GTK impls are #[cfg(feature = "gtk")]-gated
# in the consuming crate — the app must declare it for install() and
# IconThemeFont to exist.
default = ["gtk"]
gtk = []
```

```rust
// build.rs
fn main() -> miette::Result<()> {
    fonts_rs_generator::IconSetBuild::from_file("icons.toml")?.run()?;
    Ok(())
}
```

```rust
// src/icons.rs — the entire module
include!(concat!(env!("OUT_DIR"), "/iconset.rs"));
```

## `icons.toml`

```toml
[iconset]

# PascalCase marker enum name (generated as `pub enum MyAppIcons {}`).
name = "MyAppIcons"

# Human-readable name for build logs and generated docs.
display_name = "My Application Icons"

# Icon name prefix: `check` becomes `myapp-check-symbolic`.
icon_prefix = "myapp"

# GResource prefix under which the SVGs register.
gresource_prefix = "/com/example/myapp/icons"

# Freedesktop icon context. Optional, default "actions".
icons_context = "actions"

[[iconset.sources]]
crate = "fonts-rs-lucide"          # packaged resources/svg via DEP_LUCIDE_SVG_DIR
post_processors = ["symbolic-classes"]
icons = [
    "check",
    { name = "download", rename = "install" },   # rename → myapp-install-symbolic
    "trash",
]

[[iconset.sources]]
crate = "fonts-rs-tabler"
subdirectory = "filled"            # subdirectory inside resources/svg
post_processors = ["current-color-fill"]
icons = [
    { name = "check", rename = "check-filled" }, # disambiguates Lucide's check
    "brand-github",
]

# `dir = "…"` (relative to CARGO_MANIFEST_DIR) is the escape hatch for
# hand-drawn or non-crate sources.
```

All `[[iconset.sources]]` tables merge into *one* set: one marker
type, one prefix, one GResource bundle, one codepoint space. The
whitelist `name` is the upstream SVG file stem; `rename` is the
exported base name (`{icon_prefix}-{rename}-symbolic`).

Source crates advertise their `resources/svg` directory via `links` +
`cargo:svg_dir` build-script metadata (the pattern the emoji resource
crates established in v0.5.0). `fonts-rs-{lucide, tabler, heroicons,
eva, fluent, line-awesome}` all emit it; `cargo metadata` is the
fallback for sources without `links`.

Available `post_processors` (kebab-case, per source):

- `symbolic-classes` — GTK symbolic-recoloring classes (required for
  stroke-based sets like Lucide, Heroicons outline)
- `current-color-fill` / `current-color-fill-inject` / `current-color-stroke`
- `optimize` — strip comments and whitespace
- `strip-entity-namespaces`

## Using the Set

```rust
fn main() {
    icons::MyAppIcons::install().expect("icon set registration failed");

    let image = gtk4::Image::from_icon_name(icons::icon_names::MYAPP_CHECK_SYMBOLIC);
}
```

`install()` registers `icons.gresource` and the icon theme; the
`icon_names` module exposes one `&str` constant per exported icon.
Only the whitelisted SVGs land in the binary — the example above is
~15–20 KB versus the ~3 MB full Lucide bundle.

## Errors

Build failures surface as miette diagnostics: `UnknownIcon` reports
the whitelisted name plus the three closest file stems the source
actually contains, `IconNameCollision` fires when two entries (within
or across sources) resolve to the same exported name, and
`SourceNotFound` lists which `DEP_*` variables and resolution paths
were tried.

## Fixture

`crates/fonts-rs-iconset-test` (`publish = false`) is the end-to-end
fixture — a merged Lucide + Tabler + `dir` set exercising renames and
per-source post-processors with headless assertions.

See `concepts/planned/CUSTOM_ICON_SET.md` for the design rationale.
