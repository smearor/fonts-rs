//! Facade crate for the fonts-rs workspace.
//!
//! One dependency, feature-selected font families. Every enabled
//! family is reachable under its own module alias
//! (`fonts_rs::lucide::…`), and its marker type plus name alias are
//! additionally re-exported at the crate root (`fonts_rs::Lucide`,
//! `fonts_rs::LucideIconName`) - the types most call sites name.
//! `fonts_rs::model` is always available.
//!
//! ```toml
//! [dependencies]
//! fonts-rs = { version = "0.5", features = ["lucide", "doto"] }
//! ```
//!
//! # Feature flags
//!
//! **Family selectors** - enable the family crate:
//!
//! | Feature | Crate | Family |
//! |---|---|---|
//! | `lucide` | `fonts-rs-lucide` | Lucide Icons |
//! | `tabler` | `fonts-rs-tabler` | Tabler Icons |
//! | `heroicons` | `fonts-rs-heroicons` | Heroicons |
//! | `eva` | `fonts-rs-eva` | Eva Icons |
//! | `fluent` | `fonts-rs-fluent` | Fluent UI System Icons |
//! | `line-awesome` | `fonts-rs-line-awesome` | Line Awesome |
//! | `doto` | `fonts-rs-doto` | Doto pixel font |
//! | `seven-segment` | `fonts-rs-seven-segment` | DSEG7 |
//! | `fourteen-segment` | `fonts-rs-fourteen-segment` | DSEG14 |
//! | `barcode-code39` | `fonts-rs-barcode-code39` | Code 39 |
//! | `barcode-code128` | `fonts-rs-barcode-code128` | Code 128 |
//! | `barcode-ean13` | `fonts-rs-barcode-ean13` | EAN-13 |
//! | `bravura` | `fonts-rs-bravura` | Bravura SMuFL |
//! | `redacted` | `fonts-rs-redacted` | Redacted |
//! | `dicefont` | `fonts-rs-dicefont` | Dicefont |
//! | `cuernavaca` | `fonts-rs-cuernavaca` | Cuernavaca chess |
//! | `nerd-fonts` | `nerd-fonts-rs` | Nerd Fonts |
//!
//! **Emoji** - variant features plus aliases for the default
//! (`color-flags`, COLRv1):
//!
//! | Feature | Effect |
//! |---|---|
//! | `noto-emoji` | Noto Emoji, `color-flags` variant |
//! | `noto-emoji-mono` | monochrome CBDT variant |
//! | `noto-emoji-color-flags` / `-color-noflags` | COLRv1 |
//! | `noto-emoji-bitmap-flags` / `-bitmap-noflags` | CBDT |
//! | `blobmoji` | Blobmoji, `color-flags` variant |
//! | `blobmoji-color-flags` / `-color-noflags` | COLRv1 |
//! | `blobmoji-bitmap-flags` / `-bitmap-noflags` | CBDT |
//!
//! **Capabilities** - forwarded to every enabled family that has them:
//!
//! | Feature | Semantics |
//! |---|---|
//! | `gtk` (default) | `GtkFont`/`IconThemeFont`, GTK rendering |
//! | `render` | `ab_glyph` software rasterization |
//! | `metadata` | keyword/category/alias metadata |
//! | `embed-fonts` | TTF bytes embedded via GResource |
//!
//! **Groups** - `icons`, `emoji`, `segment`, `barcode`, `all`.
//!
//! **Binary** - `gui` builds the `fonts-rs` multi-family cheat sheet
//! (`cargo install fonts-rs --features gui`); `gui-core` plus selected
//! `sheet-*` features installs a subset.
//!
//! `default-features = false` produces a genuinely GTK-free build:
//! family deps are declared without default features.

/// Shared model types - `CodePoint`, `GlyphName<F>`, the capability
/// traits and [`model::prelude`].
pub use fonts_rs_model as model;

#[cfg(feature = "barcode-code39")]
pub use fonts_rs_barcode_code39 as barcode_code39;
#[cfg(feature = "barcode-code39")]
pub use fonts_rs_barcode_code39::{Code39, Code39Name};

#[cfg(feature = "barcode-code128")]
pub use fonts_rs_barcode_code128 as barcode_code128;
#[cfg(feature = "barcode-code128")]
pub use fonts_rs_barcode_code128::{Code128, Code128Name};

#[cfg(feature = "barcode-ean13")]
pub use fonts_rs_barcode_ean13 as barcode_ean13;
#[cfg(feature = "barcode-ean13")]
pub use fonts_rs_barcode_ean13::{Ean13, Ean13Name};

#[cfg(feature = "bravura")]
pub use fonts_rs_bravura as bravura;
#[cfg(feature = "bravura")]
pub use fonts_rs_bravura::{Bravura, BravuraName};

#[cfg(feature = "cuernavaca")]
pub use fonts_rs_cuernavaca as cuernavaca;
#[cfg(feature = "cuernavaca")]
pub use fonts_rs_cuernavaca::{Cuernavaca, CuernavacaName};

#[cfg(feature = "dicefont")]
pub use fonts_rs_dicefont as dicefont;
#[cfg(feature = "dicefont")]
pub use fonts_rs_dicefont::{Dicefont, DicefontName};

#[cfg(feature = "doto")]
pub use fonts_rs_doto as doto;
#[cfg(feature = "doto")]
pub use fonts_rs_doto::{Doto, DotoName};

#[cfg(feature = "eva")]
pub use fonts_rs_eva as eva;
#[cfg(feature = "eva")]
pub use fonts_rs_eva::{Eva, EvaIconName};

#[cfg(feature = "fluent")]
pub use fonts_rs_fluent as fluent;
#[cfg(feature = "fluent")]
pub use fonts_rs_fluent::{Fluent, FluentIconName};

#[cfg(feature = "fourteen-segment")]
pub use fonts_rs_fourteen_segment as fourteen_segment;
#[cfg(feature = "fourteen-segment")]
pub use fonts_rs_fourteen_segment::{FourteenSegment, FourteenSegmentName};

#[cfg(feature = "heroicons")]
pub use fonts_rs_heroicons as heroicons;
#[cfg(feature = "heroicons")]
pub use fonts_rs_heroicons::{Heroicons, HeroiconsIconName};

#[cfg(feature = "line-awesome")]
pub use fonts_rs_line_awesome as line_awesome;
#[cfg(feature = "line-awesome")]
pub use fonts_rs_line_awesome::{LineAwesome, LineAwesomeIconName};

#[cfg(feature = "lucide")]
pub use fonts_rs_lucide as lucide;
#[cfg(feature = "lucide")]
pub use fonts_rs_lucide::{Lucide, LucideIconName};

#[cfg(feature = "redacted")]
pub use fonts_rs_redacted as redacted;
#[cfg(feature = "redacted")]
pub use fonts_rs_redacted::{Redacted, RedactedName};

#[cfg(feature = "seven-segment")]
pub use fonts_rs_seven_segment as seven_segment;
#[cfg(feature = "seven-segment")]
pub use fonts_rs_seven_segment::{SevenSegment, SevenSegmentName};

#[cfg(feature = "tabler")]
pub use fonts_rs_tabler as tabler;
#[cfg(feature = "tabler")]
pub use fonts_rs_tabler::{Tabler, TablerIconName};

#[cfg(feature = "nerd-fonts")]
pub use nerd_fonts_rs as nerd_fonts;
#[cfg(feature = "nerd-fonts")]
pub use nerd_fonts_rs::NerdFonts;

#[cfg(feature = "emoji-noto")]
pub use fonts_rs_noto_emoji as noto_emoji;
#[cfg(feature = "emoji-noto")]
pub use fonts_rs_noto_emoji::{NotoEmoji, NotoEmojiName};

#[cfg(feature = "emoji-blobmoji")]
pub use fonts_rs_blobmoji as blobmoji;
#[cfg(feature = "emoji-blobmoji")]
pub use fonts_rs_blobmoji::{Blobmoji, BlobmojiName};

pub mod prelude;
