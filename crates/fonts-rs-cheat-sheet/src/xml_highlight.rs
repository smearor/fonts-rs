//! Lightweight XML syntax highlighting for the SVG code block.
//!
//! A full editor widget (`gtksourceview`) would pull in a heavy
//! dependency chain for a read-only preview. Instead a small byte
//! scanner marks tag names, attributes, delimiters and comments with
//! [`gtk4::TextTag`]s - enough to make raw SVG XML readable.

use gtk4::TextBuffer;
use gtk4::TextTag;
use gtk4::pango::Style;
use gtk4::prelude::*;

/// What a byte range in the XML source represents.
enum Token {
    /// `<`, `>`, `/`, `=` and the `/>` and `</` delimiters.
    Punct,
    /// Element name after `<` or `</`.
    TagName,
    /// Attribute name inside a tag.
    AttrName,
    /// Quoted attribute value including its quotes.
    AttrValue,
    /// `<!-- ... -->` block.
    Comment,
}

/// Dim foreground for delimiters and comments - a mid gray that
/// stays readable on both light and dark themes.
const DIM: &str = "#9a9996";

/// Applies XML syntax highlighting to `buffer`, which must already
/// contain `xml`. `dark` selects the palette variant (Adwaita blue 4
/// vs blue 2 etc.).
pub(super) fn apply(buffer: &TextBuffer, xml: &str, dark: bool) {
    let (tag_color, attr_color, value_color) = if dark {
        ("#62a0ea", "#dc8add", "#8ff0a4")
    } else {
        ("#1c71d8", "#9141ac", "#26a269")
    };
    let punct = TextTag::new(None);
    punct.set_foreground(Some(DIM));
    let tag_name = TextTag::new(None);
    tag_name.set_foreground(Some(tag_color));
    tag_name.set_weight(700);
    let attr_name = TextTag::new(None);
    attr_name.set_foreground(Some(attr_color));
    let attr_value = TextTag::new(None);
    attr_value.set_foreground(Some(value_color));
    let comment = TextTag::new(None);
    comment.set_foreground(Some(DIM));
    comment.set_style(Style::Italic);
    let tag_table = buffer.tag_table();
    tag_table.add(&punct);
    tag_table.add(&tag_name);
    tag_table.add(&attr_name);
    tag_table.add(&attr_value);
    tag_table.add(&comment);
    for (start, end, token) in tokenize(xml) {
        let tag = match token {
            Token::Punct => &punct,
            Token::TagName => &tag_name,
            Token::AttrName => &attr_name,
            Token::AttrValue => &attr_value,
            Token::Comment => &comment,
        };
        buffer.apply_tag(tag, &buffer.iter_at_offset(start as i32), &buffer.iter_at_offset(end as i32));
    }
}

/// Scans `xml` and yields `(start, end, token)` byte ranges for every
/// markup element. Text content between tags is skipped - SVG output
/// never contains a raw `<` outside markup, so plain `<` always
/// opens a tag, comment or prolog.
fn tokenize(xml: &str) -> Vec<(usize, usize, Token)> {
    let bytes = xml.as_bytes();
    let len = bytes.len();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < len {
        if bytes[i] != b'<' {
            i += 1;
            continue;
        }
        if bytes[i..].starts_with(b"<!--") {
            let start = i;
            i += 4;
            while i < len && !bytes[i..].starts_with(b"-->") {
                i += 1;
            }
            i = (i + 3).min(len);
            tokens.push((start, i, Token::Comment));
            continue;
        }
        if bytes[i..].starts_with(b"<?") || bytes[i..].starts_with(b"<!") {
            // Prolog, doctype or similar - dim the whole region.
            let start = i;
            while i < len && bytes[i] != b'>' {
                i += 1;
            }
            i = (i + 1).min(len);
            tokens.push((start, i, Token::Punct));
            continue;
        }
        // Element open or close tag.
        tokens.push((i, i + 1, Token::Punct));
        i += 1;
        if i < len && bytes[i] == b'/' {
            tokens.push((i, i + 1, Token::Punct));
            i += 1;
        }
        let name_start = i;
        while i < len && is_name_byte(bytes[i]) {
            i += 1;
        }
        if name_start < i {
            tokens.push((name_start, i, Token::TagName));
        }
        while i < len {
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i >= len {
                break;
            }
            if bytes[i] == b'>' {
                tokens.push((i, i + 1, Token::Punct));
                i += 1;
                break;
            }
            if bytes[i] == b'/' && i + 1 < len && bytes[i + 1] == b'>' {
                tokens.push((i, i + 2, Token::Punct));
                i += 2;
                break;
            }
            // Attribute name.
            let start = i;
            while i < len && is_name_byte(bytes[i]) {
                i += 1;
            }
            if i == start {
                i += 1;
                continue;
            }
            tokens.push((start, i, Token::AttrName));
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i < len && bytes[i] == b'=' {
                tokens.push((i, i + 1, Token::Punct));
                i += 1;
            }
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i < len && (bytes[i] == b'"' || bytes[i] == b'\'') {
                let quote = bytes[i];
                let start = i;
                i += 1;
                while i < len && bytes[i] != quote {
                    i += 1;
                }
                i = (i + 1).min(len);
                tokens.push((start, i, Token::AttrValue));
            }
        }
    }
    tokens
}

/// XML name characters relevant for SVG: element names and
/// `xmlns:xlink`-style attribute names.
fn is_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':' | b'.')
}
