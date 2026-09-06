# Architecture

## Module Structure

The crate is organized into modules gated by feature flags:

| Module    | Feature  | Description                                            |
|-----------|----------|--------------------------------------------------------|
| `icons`   | always   | Icon name to Unicode codepoint resolution              |
| `color`   | always   | RGBA color type for rendering and GTK                  |
| `css`     | always   | CSS string constants (GResource prefix, font-face CSS) |
| `gtk`     | `gtk`    | GTK4 icon name resolution and color application        |
| `drawing` | `render` | Software rendering onto pixel buffers                  |
| `fonts`   | `render` | Font loading (TTF and WOFF2) with caching              |
| `web`     | `web`    | Web CSS constant for web instances                     |

## Initialization Flow

```mermaid
sequenceDiagram
    participant App as Application
    participant Init as nerd_fonts_gtk::init()
    participant Fonts as fonts::init()
    participant GResource as GResource
    participant Icons as icons::register_icons()
    participant CSS as CssProvider

    App->>Init: init(base_dir)
    alt render feature
        Init->>Fonts: fonts::init(base_dir)
        Fonts->>Fonts: Set font search directory
    end
    alt gtk feature
        Init->>GResource: resources_register_include!("compiled.gresource")
        Init->>Icons: icons::register_icons()
        Init->>CSS: Load FONT_FACE_CSS into CssProvider
        Init->>CSS: Add provider to display
    end
    Init-->>App: Ready
```

## Font Loading Strategy

Fonts can be loaded in two ways:

1. **Embedded** (`embed-fonts` feature) - Font files are compiled into the binary
   via `include_bytes!`. No runtime file access needed.
2. **From disk** (default) - Font files are read from a base directory at runtime.
   The base directory is set via `init(base_dir: Option<&str>)`.

The Nerd Font symbol font (`SymbolsNerdFont-Regular.ttf`) is used for icon
rendering. The label font (`JetBrainsMonoNLNerdFont-Regular.woff2`) is used for
text labels in rendered images.

## GResource Registration

The `build.rs` script compiles two GResource bundles:

1. **`compiled.gresource`** from `resources/nerd-fonts.gresource.xml` — contains
   the Nerd Font TTF files
2. **`icons.gresource`** from `resources/icons.gresource.xml` — contains 10.403
   SVG icon files

At runtime, `init()` registers both bundles via
`gio::resources_register_include!`.

The font GResource contains:

- `SymbolsNerdFont-Regular.ttf` - Nerd Font symbol font
- `SymbolsNerdFontMono-Regular.ttf` - Monospace Nerd Font symbol font

These are referenced by the `@font-face` CSS rules in `FONT_FACE_CSS`.

## Build Pipeline

The build pipeline is fully automated in `build.rs`. Icons are generated from
the bundled font file (~2 seconds) when missing or when the font file has
changed, then phf maps and GResource bundles are compiled:

```mermaid
flowchart TD
    subgraph Export["Icon export (if metadata.json missing or font hash changed)"]
        A["SymbolsNerdFont-Regular.ttf"]
    end
    A -->|ttf-parser outline extraction| B["resources/icons/*.svg\n(10.403 SVG files)"]
    A -->|FNV-1a hash| J["resources/.font-hash"]
    subgraph Codegen["Code generation + GResource compilation"]
        C -->|phf::Map generation| E["$OUT_DIR/codemap.rs\n(phf::Map char→name + name→char)"]
        C -->|const generation| F["$OUT_DIR/icons.rs\n(icon name constants)"]
        D -->|glib-build-tools| G["icons.gresource\n(compiled GResource binary)"]
        H["resources/nerd-fonts.gresource.xml"] -->|glib-build-tools| I["compiled.gresource\n(font GResource binary)"]
    end
    A -->|ttf-parser outline extraction| C["resources/metadata.json\n(name, codepoint, file path)"]
    A -->|ttf-parser outline extraction| D["resources/icons.gresource.xml\n(GResource manifest)"]
```

The generated icon files are in `.gitignore` — they are not checked in.
`build.rs` uses `cargo:rerun-if-changed` to detect when to re-run, plus an
FNV-1a font hash comparison to decide whether the export needs to run. No
manual cleanup is needed when updating the font file. See
[Icon Export](./icon-export.md) for details.
