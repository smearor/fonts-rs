//! Prelude re-exporting the runtime font API.
//!
//! One `use` line brings the core trait and all enabled capability
//! traits into scope — same pattern as `gtk4::prelude::*`:
//!
//! ```ignore
//! use fonts_rs_model::prelude::*;
//! ```

pub use crate::CodePoint;
#[cfg(feature = "gtk")]
pub use crate::FontError;
pub use crate::FontFamily;
pub use crate::FontFamilyMarker;
#[cfg(feature = "gtk")]
pub use crate::GtkFont;
#[cfg(feature = "gtk")]
pub use crate::HtmlBuilder;
#[cfg(feature = "gtk")]
pub use crate::HtmlExport;
#[cfg(feature = "gtk")]
pub use crate::HtmlMode;
#[cfg(feature = "gtk")]
pub use crate::IconThemeFont;
#[cfg(feature = "gtk")]
pub use crate::PangoFont;
#[cfg(feature = "render")]
pub use crate::SoftwareRender;
