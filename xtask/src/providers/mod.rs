mod copy;
mod heroicons_keywords;
mod iconify_aliases;
mod line_awesome;
mod lucide_metadata;
mod provider;

pub use provider::Provider;
pub use provider::ProviderContext;
pub use provider::ProviderOutput;
pub use provider::required_string;
pub use provider::required_string_list;

use miette::miette;

/// Returns the provider registered under `name`.
pub fn resolve_provider(name: &str) -> miette::Result<Box<dyn Provider>> {
    match name {
        "copy" => Ok(Box::new(copy::Copy)),
        "heroicons_keywords" => Ok(Box::new(heroicons_keywords::HeroiconsKeywords)),
        "iconify_aliases" => Ok(Box::new(iconify_aliases::IconifyAliases)),
        "line_awesome_aliases" => Ok(Box::new(line_awesome::LineAwesomeAliases)),
        "line_awesome_codepoints" => Ok(Box::new(line_awesome::LineAwesomeCodepoints)),
        "lucide_metadata" => Ok(Box::new(lucide_metadata::LucideMetadata)),
        other => Err(miette!("Unknown metadata provider '{other}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every documented Phase-1 provider resolves; unknown names fail.
    #[test]
    fn provider_registry() {
        for name in [
            "copy",
            "heroicons_keywords",
            "iconify_aliases",
            "line_awesome_aliases",
            "line_awesome_codepoints",
            "lucide_metadata",
        ] {
            assert!(resolve_provider(name).is_ok(), "provider '{name}' must resolve");
        }
        assert!(resolve_provider("does_not_exist").is_err());
    }
}
