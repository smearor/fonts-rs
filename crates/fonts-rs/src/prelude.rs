//! The fonts-rs prelude - one `use` for the common surface.
//!
//! Forwards [`fonts_rs_model::prelude`] (core traits like
//! `FontFamily`, `FontFamilyConfig`, `CodePoint`, `GlyphName` plus the
//! cfg-gated capability traits) and every enabled family's marker type
//! and name alias - mirroring `gtk4::prelude::*`.

pub use fonts_rs_model::prelude::*;

#[cfg(feature = "emoji-blobmoji")]
pub use crate::Blobmoji;
#[cfg(feature = "emoji-blobmoji")]
pub use crate::BlobmojiName;
#[cfg(feature = "bravura")]
pub use crate::Bravura;
#[cfg(feature = "bravura")]
pub use crate::BravuraName;
#[cfg(feature = "barcode-code39")]
pub use crate::Code39;
#[cfg(feature = "barcode-code39")]
pub use crate::Code39Name;
#[cfg(feature = "barcode-code128")]
pub use crate::Code128;
#[cfg(feature = "barcode-code128")]
pub use crate::Code128Name;
#[cfg(feature = "cuernavaca")]
pub use crate::Cuernavaca;
#[cfg(feature = "cuernavaca")]
pub use crate::CuernavacaName;
#[cfg(feature = "dicefont")]
pub use crate::Dicefont;
#[cfg(feature = "dicefont")]
pub use crate::DicefontName;
#[cfg(feature = "doto")]
pub use crate::Doto;
#[cfg(feature = "doto")]
pub use crate::DotoName;
#[cfg(feature = "barcode-ean13")]
pub use crate::Ean13;
#[cfg(feature = "barcode-ean13")]
pub use crate::Ean13Name;
#[cfg(feature = "eva")]
pub use crate::Eva;
#[cfg(feature = "eva")]
pub use crate::EvaIconName;
#[cfg(feature = "fluent")]
pub use crate::Fluent;
#[cfg(feature = "fluent")]
pub use crate::FluentIconName;
#[cfg(feature = "fourteen-segment")]
pub use crate::FourteenSegment;
#[cfg(feature = "fourteen-segment")]
pub use crate::FourteenSegmentName;
#[cfg(feature = "heroicons")]
pub use crate::Heroicons;
#[cfg(feature = "heroicons")]
pub use crate::HeroiconsIconName;
#[cfg(feature = "line-awesome")]
pub use crate::LineAwesome;
#[cfg(feature = "line-awesome")]
pub use crate::LineAwesomeIconName;
#[cfg(feature = "lucide")]
pub use crate::Lucide;
#[cfg(feature = "lucide")]
pub use crate::LucideIconName;
#[cfg(feature = "nerd-fonts")]
pub use crate::NerdFonts;
#[cfg(feature = "emoji-noto")]
pub use crate::NotoEmoji;
#[cfg(feature = "emoji-noto")]
pub use crate::NotoEmojiName;
#[cfg(feature = "redacted")]
pub use crate::Redacted;
#[cfg(feature = "redacted")]
pub use crate::RedactedName;
#[cfg(feature = "seven-segment")]
pub use crate::SevenSegment;
#[cfg(feature = "seven-segment")]
pub use crate::SevenSegmentName;
#[cfg(feature = "tabler")]
pub use crate::Tabler;
#[cfg(feature = "tabler")]
pub use crate::TablerIconName;
