use crate::json::to_json;
use crate::providers::Provider;
use crate::providers::ProviderContext;
use crate::providers::ProviderOutput;
use crate::providers::line_awesome::source;
use miette::IntoDiagnostic;
use miette::miette;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

/// `line_awesome_aliases` provider: emits `alias -> canonical stem`
/// pairs for the FA compatibility classes. `line-awesome.css` carries
/// the FA4 layer (`.la.la-{name}`: alias key without the `la-`
/// prefix), `font-awesome-line-awesome/css/all.css` the FA5 layer
/// (`.fa-{name}`: alias key verbatim). Each class codepoint resolves
/// to its canonical `$la-` variable name and then to the svg stem —
/// `{name}` first, `{name}-solid` when only the solid variant exists
/// (e.g. `fa-hotel` -> `bed-solid`). Classes resolving to their own
/// canonical name are dropped.
pub struct LineAwesomeAliases;

impl Provider for LineAwesomeAliases {
    fn name(&self) -> &'static str {
        "line_awesome_aliases"
    }

    fn generate(&self, context: &ProviderContext) -> miette::Result<ProviderOutput> {
        let checkout = context.checkout()?;
        let scss = std::fs::read_to_string(checkout.join(source::VARIABLES_SCSS))
            .into_diagnostic()
            .map_err(|e| e.wrap_err("line_awesome_aliases: failed to read _variables.scss"))?;
        let line_awesome_css = std::fs::read_to_string(checkout.join(source::LINE_AWESOME_CSS))
            .into_diagnostic()
            .map_err(|e| e.wrap_err("line_awesome_aliases: failed to read line-awesome.css"))?;
        let font_awesome_css = std::fs::read_to_string(checkout.join(source::FONT_AWESOME_CSS))
            .into_diagnostic()
            .map_err(|e| e.wrap_err("line_awesome_aliases: failed to read all.css"))?;

        let variables = source::parse_variables(&scss)?;
        let mut canonical_names: BTreeMap<u32, String> = BTreeMap::new();
        for (name, codepoint) in &variables {
            canonical_names.insert(*codepoint, name.clone());
        }
        let stems = source::svg_stems(&checkout.join(source::SVG_DIR))?;

        let mut aliases = BTreeMap::new();
        let classes = source::icon_classes(&line_awesome_css, "la-")
            .into_iter()
            .chain(source::icon_classes(&font_awesome_css, "fa-"));
        for (class, codepoint) in classes {
            let alias = class.strip_prefix("la-").unwrap_or(&class).to_string();
            let Some(canonical) = canonical_names.get(&codepoint) else {
                continue;
            };
            if alias == *canonical {
                continue;
            }
            let Some(stem) = resolve_stem(canonical, &stems) else {
                continue;
            };
            match aliases.get(&alias) {
                Some(existing) if existing != &stem => {
                    return Err(miette!("line_awesome_aliases: '{alias}' maps to both '{existing}' and '{stem}'"));
                }
                Some(_) => {}
                None => {
                    aliases.insert(alias, stem);
                }
            }
        }

        let content = to_json(&aliases).map_err(|e| e.wrap_err("line_awesome_aliases: failed to serialize"))?;
        // The vendored line-awesome metadata files end with a newline.
        Ok(ProviderOutput::file(format!("{content}\n").into_bytes()))
    }
}

/// Resolves a canonical variable name to its svg file stem: `{name}`
/// wins, `{name}-solid` is the fallback for solid-only icons.
fn resolve_stem(name: &str, stems: &BTreeSet<String>) -> Option<String> {
    if stems.contains(name) {
        return Some(name.to_string());
    }
    let solid = format!("{name}-solid");
    stems.contains(&solid).then_some(solid)
}
