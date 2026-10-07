use miette::IntoDiagnostic;
use serde::Serialize;
use serde_json::Serializer;
use serde_json::ser::PrettyFormatter;

/// Serializes `value` as pretty JSON with a single-space indent and
/// no trailing newline - matching the established vendored metadata
/// format (`JSON.stringify(value, null, 1)`).
pub fn to_json<T: Serialize>(value: &T) -> miette::Result<String> {
    let mut buffer = Vec::new();
    let formatter = PrettyFormatter::with_indent(b" ");
    value.serialize(&mut Serializer::with_formatter(&mut buffer, formatter)).into_diagnostic()?;
    String::from_utf8(buffer).into_diagnostic()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    /// Emits 1-space indented JSON without trailing newline.
    #[test]
    fn emits_single_space_indent() {
        let mut map = BTreeMap::new();
        map.insert("b".to_string(), vec!["x".to_string(), "y".to_string()]);
        map.insert("a".to_string(), Vec::<String>::new());
        let json = to_json(&map).unwrap();
        assert_eq!(json, "{\n \"a\": [],\n \"b\": [\n  \"x\",\n  \"y\"\n ]\n}");
    }
}
