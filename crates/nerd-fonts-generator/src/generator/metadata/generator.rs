//! Generator for icon metadata phf::Map tables (keywords, categories, aliases).
//!
//! For icons with upstream metadata (Font Awesome, Material Design,
//! Devicon, Octicons), keywords and categories are sourced from the
//! respective upstream data. For other icon sets, keywords are derived
//! from icon name segments and categories from the icon set prefix.
//!
//! Implements [`MetadataGenerator`] from `fonts-rs-generator`, delegating
//! data-source lookups to [`IconMetadataRegistry`].

use super::registry::IconMetadataRegistry;
use fonts_rs_generator::MetadataGenerator;
use nerd_fonts_model::GlyphEntry;
use nerd_fonts_model::IconName;

/// Generates phf::Map constants for icon keywords, categories, and aliases.
///
/// Keywords come from upstream metadata sources when available, and from
/// name segments for other icon sets. Categories come from upstream metadata
/// when available, and from the icon set prefix for others. Aliases come
/// from upstream metadata sources.
pub struct IconsMetadataGenerator<'a> {
    registry: &'a IconMetadataRegistry,
}

impl<'a> IconsMetadataGenerator<'a> {
    /// Creates a new metadata generator backed by the given registry.
    pub fn new(registry: &'a IconMetadataRegistry) -> Self {
        Self { registry }
    }
}

impl<'a> MetadataGenerator for IconsMetadataGenerator<'a> {
    type Name = IconName;

    fn keywords_for(&self, entry: &GlyphEntry<IconName>) -> Vec<String> {
        resolve_keywords(&entry.name, self.registry)
    }

    fn categories_for(&self, entry: &GlyphEntry<IconName>) -> Vec<String> {
        resolve_categories(&entry.name, self.registry)
    }

    fn aliases(&self) -> Vec<(String, String)> {
        self.registry
            .all_aliases()
            .into_iter()
            .map(|(alias, canonical)| (alias.as_ref().to_string(), canonical.as_ref().to_string()))
            .collect()
    }

    fn deduplicate_statics() -> bool {
        true
    }
}

/// Resolves keywords for an icon, using upstream data when available.
///
/// For icons with upstream metadata, returns the upstream keywords merged
/// with name-derived segments. For other icons, returns only name-derived
/// segments.
fn resolve_keywords(icon_name: &IconName, registry: &IconMetadataRegistry) -> Vec<String> {
    let mut keywords = Vec::new();

    // Add upstream keywords first if available
    if let Some(terms) = registry.keywords_for(icon_name) {
        for term in terms {
            let kw = term.as_str().to_string();
            if !keywords.contains(&kw) {
                keywords.push(kw);
            }
        }
    }

    // Add name-derived segments as fallback/supplement
    let derived = derive_keywords(icon_name.as_ref());
    for kw in derived {
        if !keywords.contains(&kw) {
            keywords.push(kw);
        }
    }

    keywords
}

/// Resolves categories for an icon, using upstream data when available.
///
/// For icons with upstream metadata, returns the upstream category labels.
/// For other icons, returns the prefix-based category.
fn resolve_categories(icon_name: &IconName, registry: &IconMetadataRegistry) -> Vec<String> {
    if let Some(cats) = registry.categories_for(icon_name) {
        return cats.iter().map(|c| c.as_str().to_string()).collect();
    }

    // Fallback: prefix-based category
    let cat = derive_category(icon_name.as_ref());
    vec![cat.to_string()]
}

/// Derives search keywords from an icon name.
///
/// Strips the `nf-` prefix and `-symbolic` suffix, removes the set prefix
/// (e.g. `fa`, `md`), and returns each remaining segment as a keyword.
/// Also includes the full name without prefix as a keyword.
///
/// Examples:
/// - `"nf-fa-gamepad-symbolic"` -> `["gamepad"]`
/// - `"nf-md-alert-circle-outline-symbolic"` -> `["alert", "circle", "outline", "alert-circle-outline"]`
fn derive_keywords(icon_name: &str) -> Vec<String> {
    let name = icon_name.strip_prefix("nf-").unwrap_or(icon_name);
    let name = name.strip_suffix("-symbolic").unwrap_or(name);

    let segments: Vec<&str> = name.split('-').collect();
    if segments.len() < 2 {
        return Vec::new();
    }

    // Skip the set prefix (first segment, e.g. "fa", "md")
    let name_segments = &segments[1..];
    let mut keywords: Vec<String> = name_segments.iter().map(|s| (*s).to_string()).collect();

    // Add the full name (minus prefix) as a keyword
    let full_name = name_segments.join("-");
    if !full_name.is_empty() && !keywords.contains(&full_name) {
        keywords.push(full_name);
    }

    keywords
}

/// Derives a category name from an icon name prefix.
///
/// Maps the Nerd Font set prefix to a human-readable category name.
fn derive_category(icon_name: &str) -> &'static str {
    match icon_name {
        n if n.starts_with("nf-md-") => "Material Design",
        n if n.starts_with("nf-fa-") => "Font Awesome",
        n if n.starts_with("nf-dev-") => "Devicons",
        n if n.starts_with("nf-cod-") => "Codicons",
        n if n.starts_with("nf-oct-") => "Octicons",
        n if n.starts_with("nf-weather-") => "Weather",
        n if n.starts_with("nf-fae-") => "Font Awesome Extended",
        n if n.starts_with("nf-seti-") => "Seti",
        n if n.starts_with("nf-linux-") => "Linux",
        n if n.starts_with("nf-ple-") => "Powerline Extra",
        n if n.starts_with("nf-pl-") => "Powerline",
        n if n.starts_with("nf-custom-") => "Custom",
        n if n.starts_with("nf-extra-") => "Extra",
        n if n.starts_with("nf-pom-") => "Pomicons",
        n if n.starts_with("nf-alpha-") => "Alpha",
        n if n.starts_with("nf-iec-") => "IEC",
        n if n.starts_with("nf-checkbox-") => "Checkbox",
        n if n.starts_with("nf-file-") => "File",
        n if n.starts_with("nf-heart-") => "Heart",
        n if n.starts_with("nf-image-") => "Image",
        n if n.starts_with("nf-indentation-") => "Indentation",
        n if n.starts_with("nf-music-") => "Music",
        n if n.starts_with("nf-near-") => "Near",
        n if n.starts_with("nf-nonmarkingreturn-") => "Non-Marking Return",
        n if n.starts_with("nf-exit-") => "Exit",
        _ => "Other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_keywords_single_segment() {
        let kws = derive_keywords("nf-fa-gamepad-symbolic");
        assert_eq!(kws, vec!["gamepad"]);
    }

    #[test]
    fn derive_keywords_multi_segment() {
        let kws = derive_keywords("nf-md-alert-circle-outline-symbolic");
        assert!(kws.contains(&"alert".to_string()));
        assert!(kws.contains(&"circle".to_string()));
        assert!(kws.contains(&"outline".to_string()));
        assert!(kws.contains(&"alert-circle-outline".to_string()));
    }

    #[test]
    fn derive_keywords_strips_symbolic_suffix() {
        let kws = derive_keywords("nf-fa-star-symbolic");
        assert_eq!(kws, vec!["star"]);
    }

    #[test]
    fn derive_keywords_no_symbolic_suffix() {
        let kws = derive_keywords("nf-fa-star");
        assert_eq!(kws, vec!["star"]);
    }

    #[test]
    fn derive_keywords_empty_for_no_segments() {
        let kws = derive_keywords("nf-");
        assert!(kws.is_empty());
    }

    #[test]
    fn derive_keywords_empty_for_single_segment() {
        let kws = derive_keywords("nf-fa");
        assert!(kws.is_empty());
    }

    #[test]
    fn derive_category_font_awesome() {
        assert_eq!(derive_category("nf-fa-gamepad-symbolic"), "Font Awesome");
    }

    #[test]
    fn derive_category_material_design() {
        assert_eq!(derive_category("nf-md-home-symbolic"), "Material Design");
    }

    #[test]
    fn derive_category_octicons() {
        assert_eq!(derive_category("nf-oct-mark-github-symbolic"), "Octicons");
    }

    #[test]
    fn derive_category_unknown() {
        assert_eq!(derive_category("nf-unknown-xyz"), "Other");
    }
}
