//! A tiny ordered-JSON value + pretty printer that reproduces Python's
//! `json.dumps(obj, indent=2)` byte-for-byte (the format `query.py` emits).
//!
//! Ported algorithmic views (D0074/D0075 M2.2) must be byte-identical to their
//! `query.py` originals; rather than hand-format each nested structure, they build
//! a [`Json`] tree and call [`Json::dump`]. Object key order is the insertion order
//! (a `Vec` of pairs), matching Python dict ordering. Scope is what these views
//! need (ASCII strings, ints, bools, arrays, objects) — non-ASCII is emitted
//! verbatim, which is fine because the tracking data is ASCII.

use std::fmt::Write as _;

/// A JSON value with insertion-ordered object members.
pub enum Json {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Arr(Vec<Self>),
    /// Object members in insertion order (matches Python dict / `json.dumps`).
    Obj(Vec<(String, Self)>),
}

impl Json {
    /// Convenience constructor for a string value.
    pub fn s(value: impl Into<String>) -> Self {
        Self::Str(value.into())
    }

    /// Serialize to a string matching Python `json.dumps(value, indent=2)`.
    #[must_use]
    pub fn dump(&self) -> String {
        let mut out = String::new();
        self.write(&mut out, 0);
        out
    }

    fn write(&self, out: &mut String, indent: usize) {
        match self {
            Self::Null => out.push_str("null"),
            Self::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Self::Int(n) => out.push_str(&n.to_string()),
            Self::Str(s) => write_str(out, s),
            Self::Arr(items) => {
                if items.is_empty() {
                    out.push_str("[]");
                    return;
                }
                out.push_str("[\n");
                let last = items.len() - 1;
                for (i, item) in items.iter().enumerate() {
                    pad(out, indent + 1);
                    item.write(out, indent + 1);
                    if i != last {
                        out.push(',');
                    }
                    out.push('\n');
                }
                pad(out, indent);
                out.push(']');
            }
            Self::Obj(pairs) => {
                if pairs.is_empty() {
                    out.push_str("{}");
                    return;
                }
                out.push_str("{\n");
                let last = pairs.len() - 1;
                for (i, (key, value)) in pairs.iter().enumerate() {
                    pad(out, indent + 1);
                    write_str(out, key);
                    out.push_str(": ");
                    value.write(out, indent + 1);
                    if i != last {
                        out.push(',');
                    }
                    out.push('\n');
                }
                pad(out, indent);
                out.push('}');
            }
        }
    }
}

fn pad(out: &mut String, indent: usize) {
    for _ in 0..indent * 2 {
        out.push(' ');
    }
}

// Reproduce CPython's `py_encode_basestring_ascii` (json.dumps default ensure_ascii=True):
// printable ASCII verbatim (except " and \), the named short escapes, and EVERYTHING else
// (control chars + all non-ASCII) as lowercase `\uXXXX` (astral chars as a surrogate pair).
fn write_str(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if ('\u{20}'..='\u{7e}').contains(&c) => out.push(c),
            c => {
                let cp = c as u32;
                if cp > 0xFFFF {
                    let v = cp - 0x1_0000;
                    let hi = 0xD800 + (v >> 10);
                    let lo = 0xDC00 + (v & 0x3FF);
                    let _ = write!(out, "\\u{hi:04x}\\u{lo:04x}");
                } else {
                    let _ = write!(out, "\\u{cp:04x}");
                }
            }
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_containers_are_compact() {
        assert_eq!(Json::Arr(vec![]).dump(), "[]");
        assert_eq!(Json::Obj(vec![]).dump(), "{}");
    }

    #[test]
    fn nested_matches_python_indent_2() {
        let v = Json::Obj(vec![
            ("n".to_string(), Json::Int(3)),
            ("xs".to_string(), Json::Arr(vec![Json::s("a"), Json::s("b")])),
            ("empty".to_string(), Json::Arr(vec![])),
            ("ok".to_string(), Json::Bool(true)),
        ]);
        let expected = "{\n  \"n\": 3,\n  \"xs\": [\n    \"a\",\n    \"b\"\n  ],\n  \"empty\": [],\n  \"ok\": true\n}";
        assert_eq!(v.dump(), expected);
    }

    #[test]
    fn array_of_objects_indents() {
        let v = Json::Arr(vec![Json::Obj(vec![("k".to_string(), Json::s("v"))])]);
        assert_eq!(v.dump(), "[\n  {\n    \"k\": \"v\"\n  }\n]");
    }

    #[test]
    fn non_ascii_escaped_like_python_ensure_ascii() {
        // Python json.dumps("a — b") == '"a \\u2014 b"' (em dash escaped, lowercase hex).
        assert_eq!(Json::s("a — b").dump(), "\"a \\u2014 b\"");
    }
}
