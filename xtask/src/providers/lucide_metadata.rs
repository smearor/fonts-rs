use crate::providers::Provider;
use crate::providers::ProviderContext;
use crate::providers::ProviderOutput;
use crate::providers::required_string;
use crate::providers::required_string_list;
use miette::IntoDiagnostic;
use miette::miette;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeMap;
use serde_json::Value;
use std::collections::BTreeMap;

/// `lucide_metadata` provider: merges the per-icon `{dir}/*.json`
/// files of the upstream checkout into one `{stem: {field: value}}`
/// map, projecting each entry onto the declared `with.fields` in
/// order. Used for lucide's `icons.json` (`with.fields =
/// ["tags", "categories"]`, dropping `$schema`/`contributors`/
/// `use-cases`) and `categories.json` (`["title", "icon"]`).
pub struct LucideMetadata;

impl Provider for LucideMetadata {
    fn name(&self) -> &'static str {
        "lucide_metadata"
    }

    fn generate(&self, context: &ProviderContext) -> miette::Result<ProviderOutput> {
        let source = required_string(context.with, "source", self.name())?;
        let fields = required_string_list(context.with, "fields", self.name())?;
        let dir = context.checkout.join(&source);

        let mut merged = BTreeMap::new();
        for entry in std::fs::read_dir(&dir).into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            let path = entry.path();
            if !path.extension().is_some_and(|ext| ext == "json") {
                continue;
            }
            let stem = path
                .file_stem()
                .ok_or_else(|| miette!("lucide_metadata: '{}' has no file stem", path.display()))?
                .to_string_lossy()
                .into_owned();
            let document: Value = serde_json::from_slice(&std::fs::read(&path).into_diagnostic()?)
                .into_diagnostic()
                .map_err(|e| e.wrap_err(format!("lucide_metadata: failed to parse {}", path.display())))?;
            let object = document
                .as_object()
                .ok_or_else(|| miette!("lucide_metadata: {} is not a JSON object", path.display()))?;
            let mut projected = Vec::with_capacity(fields.len());
            for field in &fields {
                let value = object
                    .get(field)
                    .ok_or_else(|| miette!("lucide_metadata: {source}/{stem}.json has no '{field}' field"))?;
                projected.push((field.clone(), value.clone()));
            }
            merged.insert(stem, OrderedFields(projected));
        }

        // The vendored metadata is compact JSON without trailing newline.
        let content = serde_json::to_string(&merged)
            .into_diagnostic()
            .map_err(|e| e.wrap_err("lucide_metadata: failed to serialize"))?;
        Ok(ProviderOutput::file(content.into_bytes()))
    }
}

/// A JSON object serialized with its declared field order — the
/// vendored files keep e.g. `tags` before `categories` while
/// `serde_json::Map` would sort keys alphabetically.
struct OrderedFields(Vec<(String, Value)>);

impl Serialize for OrderedFields {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (key, value) in &self.0 {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
