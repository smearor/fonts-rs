# Platform Notes

## Linux

Linux is the primary target for the `gtk` feature.

### System Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install -y pkg-config libgtk-4-dev libglib2.0-dev

# Fedora
sudo dnf install -y pkg-config gtk4-devel glib2-devel

# Arch Linux
sudo pacman -S pkgconf gtk4 glib2
```

### GResource Compilation

The `build.rs` script uses `glib_build_tools::compile_resources` to compile
the GResource XML into a binary. This requires `glib-build-tools` as a
build dependency and the GTK 4 development headers to be installed.

## macOS

The `gtk` feature can be used on macOS with GTK 4 installed via Homebrew:

```bash
brew install gtk4
```

No additional configuration is needed beyond the standard Rust build process.

## Windows

The `gtk` feature can be used on Windows with GTK 4 installed via MSYS2 or
gvsbuild. See the [GTK4 Windows installation
guide](https://gtk.org/docs/installations/windows/) for details.

## Font Loading Mode

The `render` feature enables font loading (TTF/WOFF2) via `ab_glyph`. It has no
platform-specific dependencies and works on any platform supported by Rust.

```toml
[dependencies]
nerd-fonts-rs = { version = "0.1", default-features = false, features = ["render"] }
```

## Web-Only Mode

The `web` feature has no platform-specific dependencies. It simply provides a
CSS string constant.

```toml
[dependencies]
nerd-fonts-rs = { version = "0.1", default-features = false, features = ["web"] }
```

## Embedded Fonts

The `embed-fonts` feature compiles font files directly into the binary. This
increases binary size by approximately 200-500 KB but eliminates the need for
runtime file access. This is useful for:

- Standalone binaries
- Environments without a filesystem
- Simplified deployment
