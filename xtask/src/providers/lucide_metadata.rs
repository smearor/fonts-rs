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
        let dir = context.checkout()?.join(&source);

        let mut paths: Vec<std::path::PathBuf> = Vec::new();
        for entry in std::fs::read_dir(&dir).into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json") {
                paths.push(path);
            }
        }
        // The vendored key order follows filename sort ('-' < '.'), so
        // `alarm-clock-check.json` precedes `alarm-clock.json`.
        paths.sort();

        let mut merged = Vec::with_capacity(paths.len());
        for path in paths {
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
            merged.push((stem, OrderedFields(projected)));
        }

        // The vendored metadata is compact JSON without trailing
        // newline and with non-ASCII characters \u-escaped (the
        // upstream merge script used Python's json.dumps).
        let content = serde_json::to_string(&MergedObjects(merged))
            .into_diagnostic()
            .map_err(|e| e.wrap_err("lucide_metadata: failed to serialize"))?;
        Ok(ProviderOutput::file(escape_non_ascii(&content).into_bytes()))
    }
}

/// A JSON object serialized with its declared field order - the
/// vendored files keep e.g. `tags` before `categories` while
/// `serde_json::Map` would sort keys alphabetically.
struct OrderedFields(Vec<(String, Value)>);

/// The merged top-level object serialized in insertion order - the
/// vendored files order keys by upstream filename, not by stem.
struct MergedObjects(Vec<(String, OrderedFields)>);

impl Serialize for OrderedFields {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_ordered_map(serializer, self.0.iter())
    }
}

impl Serialize for MergedObjects {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_ordered_map(serializer, self.0.iter())
    }
}

/// Replaces non-ASCII characters in `json` with `\uXXXX` escapes
/// (surrogate pairs beyond the BMP), matching Python's
/// `json.dumps(ensure_ascii=True)`. Safe to run on serialized JSON
/// because non-ASCII can only occur inside string literals.
fn escape_non_ascii(json: &str) -> String {
    let mut escaped = String::with_capacity(json.len());
    let mut buffer = [0u16; 2];
    for ch in json.chars() {
        if ch.is_ascii() {
            escaped.push(ch);
        } else {
            for unit in ch.encode_utf16(&mut buffer) {
                escaped.push_str(&format!("\\u{unit:04x}"));
            }
        }
    }
    escaped
}

/// Serializes `entries` as a JSON object in iteration order.
fn serialize_ordered_map<'a, S, V, I>(serializer: S, entries: I) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    V: Serialize + 'a,
    I: Iterator<Item = &'a (String, V)> + ExactSizeIterator,
{
    let mut map = serializer.serialize_map(Some(entries.len()))?;
    for (key, value) in entries {
        map.serialize_entry(key, value)?;
    }
    map.end()
}
