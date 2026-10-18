//! The [`BuilderSettings`] set-level fields of an `icons.toml`.

use fonts_rs_model::IconSetSpec;
use fonts_rs_model::IconSetSpecError;

/// The `[iconset]` table fields, editable in the settings pane.
///
/// Validation is delegated to [`IconSetSpec::validate`] at emission
/// time - the builder cannot emit a spec the generator would reject.
pub struct BuilderSettings {
    /// PascalCase marker type name (`IconSetConfig`-marker).
    pub name: String,
    /// Human-readable set name for docs.
    pub display_name: String,
    /// Kebab-case exported icon prefix (`{prefix}-{name}-symbolic`).
    pub icon_prefix: String,
    /// GResource mount prefix for the exported icons.
    pub gresource_prefix: String,
    /// GTK icon-theme context for registration.
    pub icons_context: String,
}

impl Default for BuilderSettings {
    fn default() -> Self {
        Self {
            name: "MyIcons".to_string(),
            display_name: "My Icons".to_string(),
            icon_prefix: "my".to_string(),
            gresource_prefix: "/com/example/myapp/icons".to_string(),
            icons_context: "actions".to_string(),
        }
    }
}

impl BuilderSettings {
    /// Validates the set fields by building a minimal spec around them
    /// and delegating to [`IconSetSpec::validate`]. The placeholder
    /// source keeps the check honest without a selection.
    pub fn validate(&self, sources: Vec<fonts_rs_model::IconSetSourceSpec>) -> Result<(), IconSetSpecError> {
        IconSetSpec {
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            icon_prefix: self.icon_prefix.clone(),
            gresource_prefix: self.gresource_prefix.clone(),
            icons_context: self.icons_context.clone(),
            sources,
        }
        .validate()
    }
}
