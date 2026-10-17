//! The fonts-rs prelude - one `use` for the common surface.
//!
//! Forwards [`fonts_rs_model::prelude`] (core traits like
//! `FontFamily`, `FontFamilyConfig`, `CodePoint`, `GlyphName` plus the
//! cfg-gated capability traits) and every enabled family's marker type
//! and name alias - mirroring `gtk4::prelude::*`.

pub use fonts_rs_model::prelude::*;

#[cfg(feature = "nerd-fonts")]
pub use crate::NerdFonts;
#[cfg(feature = "emoji-blobmoji")]
pub use crate::{Blobmoji, BlobmojiName};
#[cfg(feature = "bravura")]
pub use crate::{Bravura, BravuraName};
#[cfg(feature = "barcode-code39")]
pub use crate::{Code39, Code39Name};
#[cfg(feature = "barcode-code128")]
pub use crate::{Code128, Code128Name};
#[cfg(feature = "cuernavaca")]
pub use crate::{Cuernavaca, CuernavacaName};
#[cfg(feature = "dicefont")]
pub use crate::{Dicefont, DicefontName};
#[cfg(feature = "doto")]
pub use crate::{Doto, DotoName};
#[cfg(feature = "barcode-ean13")]
pub use crate::{Ean13, Ean13Name};
#[cfg(feature = "eva")]
pub use crate::{Eva, EvaIconName};
#[cfg(feature = "fluent")]
pub use crate::{Fluent, FluentIconName};
#[cfg(feature = "fourteen-segment")]
pub use crate::{FourteenSegment, FourteenSegmentName};
#[cfg(feature = "heroicons")]
pub use crate::{Heroicons, HeroiconsIconName};
#[cfg(feature = "line-awesome")]
pub use crate::{LineAwesome, LineAwesomeIconName};
#[cfg(feature = "lucide")]
pub use crate::{Lucide, LucideIconName};
#[cfg(feature = "emoji-noto")]
pub use crate::{NotoEmoji, NotoEmojiName};
#[cfg(feature = "redacted")]
pub use crate::{Redacted, RedactedName};
#[cfg(feature = "seven-segment")]
pub use crate::{SevenSegment, SevenSegmentName};
#[cfg(feature = "tabler")]
pub use crate::{Tabler, TablerIconName};
