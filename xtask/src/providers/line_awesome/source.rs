use miette::IntoDiagnostic;
use miette::miette;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

/// SCSS variables file with the `$la-{name}: \fXXX;` declarations.
pub const VARIABLES_SCSS: &str = "dist/line-awesome/scss/_variables.scss";
/// Compiled stylesheet with the FA4-compat `.la.la-{name}` classes.
pub const LINE_AWESOME_CSS: &str = "dist/line-awesome/css/line-awesome.css";
/// Compiled FA5-compat stylesheet with the `.fa-{name}` classes.
pub const FONT_AWESOME_CSS: &str = "dist/font-awesome-line-awesome/css/all.css";
/// Flat directory of `{name}.svg` / `{name}-solid.svg` icon files.
pub const SVG_DIR: &str = "svg";

/// Parses `$la-{name}: \fXXX;` declarations into a name -> codepoint
/// map. Non-icon variables (paths, colors, sizes) carry no `\f` escape
/// and are ignored.
pub fn parse_variables(scss: &str) -> miette::Result<BTreeMap<String, u32>> {
    let mut variables = BTreeMap::new();
    for (line_number, line) in scss.lines().enumerate() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("$la-") else {
            continue;
        };
        let Some((name, value)) = rest.split_once(':') else {
            continue;
        };
        let Some(codepoint) = parse_css_escape(value) else {
            continue;
        };
        let name = name.trim().to_string();
        if variables.insert(name.clone(), codepoint).is_some() {
            return Err(miette!("line-awesome: duplicate variable '$la-{name}' at line {}", line_number + 1));
        }
    }
    Ok(variables)
}

/// Extracts `.{class}... { content: "\fXXX"; }` rules and returns the
/// `(class, codepoint)` pairs whose class carries `prefix` (`la-` /
/// `fa-`). For compound selectors the icon class is the last prefixed
/// token (`.la.la-address-book-o` -> `la-address-book-o`).
pub fn icon_classes(css: &str, prefix: &str) -> Vec<(String, u32)> {
    let mut classes = Vec::new();
    for rule in css.split('}') {
        let Some((selector, body)) = rule.split_once('{') else {
            continue;
        };
        let Some(class) = selector
            .split(['.', ',', ' ', '\t', '\n'])
            .filter_map(|token| token.split(':').next())
            .filter(|token| token.starts_with(prefix))
            .next_back()
        else {
            continue;
        };
        let Some(codepoint) = parse_css_escape(body) else {
            continue;
        };
        classes.push((class.to_string(), codepoint));
    }
    classes
}

/// Reads the `content: "\fXXX"` escape out of a declaration block or
/// scss value.
fn parse_css_escape(text: &str) -> Option<u32> {
    let start = text.find('\\')? + 1;
    let digits: String = text[start..].chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    if digits.is_empty() {
        return None;
    }
    u32::from_str_radix(&digits, 16).ok()
}

/// Lists the sorted file stems of every `*.svg` directly below `dir`.
pub fn svg_stems(dir: &Path) -> miette::Result<BTreeSet<String>> {
    let mut stems = BTreeSet::new();
    for entry in std::fs::read_dir(dir).into_diagnostic()? {
        let entry = entry.into_diagnostic()?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "svg") {
            if let Some(stem) = path.file_stem() {
                stems.insert(stem.to_string_lossy().into_owned());
            }
        }
    }
    Ok(stems)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses `$la-{name}: \fXXX;` and ignores non-icon variables.
    #[test]
    fn parses_icon_variables() {
        let scss = "$la-font-path: \"../fonts\" !default;\n$la-500px: \\f26e;\n$la-address-book: \\f2b9;\n$la-version: 1.3.0 !default;\n";
        let vars = parse_variables(scss).unwrap();
        assert_eq!(vars.len(), 2);
        assert_eq!(vars["500px"], 0xf26e);
        assert_eq!(vars["address-book"], 0xf2b9);
    }

    /// Takes the last prefixed class of compound selectors.
    #[test]
    fn extracts_icon_classes() {
        let css = ".la.la-address-book-o:before {\n  content: \"\\f2b9\"; }\n\n.la-fw {\n  width: 1em; }\n.fa-arrows-alt:before {\n  content: \"\\f0b2\"; }";
        assert_eq!(icon_classes(css, "la-"), vec![("la-address-book-o".to_string(), 0xf2b9)]);
        assert_eq!(icon_classes(css, "fa-"), vec![("fa-arrows-alt".to_string(), 0xf0b2)]);
    }
}
