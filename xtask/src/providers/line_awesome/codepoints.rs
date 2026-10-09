use crate::json::to_json;
use crate::providers::Provider;
use crate::providers::ProviderContext;
use crate::providers::ProviderOutput;
use crate::providers::line_awesome::source;
use miette::IntoDiagnostic;
use miette::miette;
use std::collections::BTreeMap;

/// `line_awesome_codepoints` provider: joins every `svg/` file stem
/// with its codepoint from `$la-{name}: \fXXX;` variables. Solid
/// variants share the base name's codepoint (`{stem}-solid` looks up
/// `$la-{stem}`), matching `CodepointStrategy::FromNameMap`.
pub struct LineAwesomeCodepoints;

impl Provider for LineAwesomeCodepoints {
    fn name(&self) -> &'static str {
        "line_awesome_codepoints"
    }

    fn generate(&self, context: &ProviderContext) -> miette::Result<ProviderOutput> {
        let checkout = context.checkout()?;
        let scss = std::fs::read_to_string(checkout.join(source::VARIABLES_SCSS))
            .into_diagnostic()
            .map_err(|e| e.wrap_err("line_awesome_codepoints: failed to read _variables.scss"))?;
        let variables = source::parse_variables(&scss)?;
        let stems = source::svg_stems(&checkout.join(source::SVG_DIR))?;

        let mut codepoints = BTreeMap::new();
        for stem in stems {
            let base = stem.strip_suffix("-solid").unwrap_or(&stem);
            let codepoint = variables
                .get(base)
                .ok_or_else(|| miette!("line_awesome_codepoints: no $la-{base} variable for stem '{stem}'"))?;
            codepoints.insert(stem, *codepoint);
        }

        let content = to_json(&codepoints).map_err(|e| e.wrap_err("line_awesome_codepoints: failed to serialize"))?;
        // The vendored line-awesome metadata files end with a newline.
        Ok(ProviderOutput::file(format!("{content}\n").into_bytes()))
    }
}
