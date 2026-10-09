use crate::providers::Provider;
use crate::providers::ProviderContext;
use crate::providers::ProviderOutput;
use crate::providers::required_string;
use miette::IntoDiagnostic;

/// `copy` provider: vendors a single file from the upstream checkout
/// verbatim (`with.source` = checkout-relative path).
pub struct Copy;

impl Provider for Copy {
    fn name(&self) -> &'static str {
        "copy"
    }

    fn generate(&self, context: &ProviderContext) -> miette::Result<ProviderOutput> {
        let source = required_string(context.with, "source", self.name())?;
        let path = context.checkout()?.join(&source);
        let content = std::fs::read(&path)
            .into_diagnostic()
            .map_err(|e| e.wrap_err(format!("copy provider: failed to read {source}")))?;
        Ok(ProviderOutput::file(content))
    }
}
