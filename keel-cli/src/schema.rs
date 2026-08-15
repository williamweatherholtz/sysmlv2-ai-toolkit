//! The schema, read by the engine itself — so vocabulary lives in ONE place (issue119/issue120).
//!
//! # Why this module exists
//!
//! Three constants in this binary used to restate schema vocabulary: `KNOWN_EDGES` mirrored the
//! `EdgeKind` enum, `ENGINE_MARKERS` mirrored the declared `metadata def`s, and `RISK_ORDER` mirrored
//! the `RiskClass` enum. Nothing reconciled any of them, and one had already drifted in BOTH
//! directions: the view layer rejected `derivedFrom`, `covers`, `dispositions` and `specialize` —
//! which the schema declares and which 166 edges in the model actually use — while accepting three
//! kinds the schema never declared. A user reading the schema was told its own vocabulary was unknown.
//!
//! Deriving instead of restating makes that drift UNREPRESENTABLE rather than merely detectable.
//!
//! # Why the EMBEDDED schema and never the project's
//!
//! `include_dir!` bakes the engine's own `schema/` into the binary at build time. Reading the
//! DOWNSTREAM PROJECT's on-disk `.engine/schema/` instead is exactly what caused issue090: a newer
//! binary met an older on-disk schema and produced 566 violations, blocking every commit, with the
//! only remedy being a frozen-core edit. So the engine's vocabulary travels WITH the engine. A
//! project's own additions are read from its files separately, where they are additive and cannot
//! take the engine's vocabulary away.
//!
//! # Parsing, and why it is not the real parser
//!
//! A deliberately small scan: schema files declare their vocabulary in a handful of fixed forms, and
//! this needs the names, not the semantics. It strips `//` comments and `/* */` doc blocks FIRST,
//! because this corpus is full of prose ABOUT the schema and a text scan that reads prose as
//! structure is a mistake this project has made repeatedly (issue099).

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use include_dir::{include_dir, Dir};

/// The engine's own schema, baked in at build time. See the module doc for why this is never the
/// downstream project's on-disk copy.
static SCHEMA_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../.engine/schema");

/// Everything derivable from the schema text.
pub struct Vocabulary {
    /// Enum name -> its members IN DECLARATION ORDER. Order is load-bearing: `RiskClass` is declared
    /// worst-first and `arch criticality` ranks on exactly that sequence.
    pub enums: HashMap<String, Vec<String>>,
    /// Every `metadata def` — the marker vocabulary.
    pub markers: HashSet<String>,
    /// Type name -> the attribute/ref names it declares directly (not including inherited).
    pub attrs: HashMap<String, HashSet<String>>,
    /// Type name -> its supertype, from `X :> Y`, so inherited attributes can be resolved.
    pub supertype: HashMap<String, String>,
}

/// Strip `/* */` blocks and `//` tails.
///
/// FIRST, before any structural match. `.engine/schema` is unusually comment-dense — the files
/// explain their own design at length — so a scan that skips this reads sentences about
/// `metadata def` as declarations of one.
fn strip_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(i) = rest.find("/*") {
        out.push_str(&rest[..i]);
        let Some(j) = rest[i..].find("*/") else {
            rest = "";
            break;
        };
        rest = &rest[i + j + 2..];
    }
    out.push_str(rest);
    out.lines().map(|l| l.split("//").next().unwrap_or("")).collect::<Vec<_>>().join("\n")
}

/// `<kind> def <Name>` on a line, returning `(kind, name)`.
///
/// Kind is matched as "any lowercase word(s) before `def`" rather than an allow-list, which is the
/// bug the throwaway audit script hit twice: an allow-list of five kinds silently missed
/// `occurrence def TestResult` and `verification def Test`, and reported the engine's most-used
/// types as undefined.
fn def_on_line(line: &str) -> Option<(String, String)> {
    let t = line.trim().strip_prefix("abstract ").unwrap_or_else(|| line.trim());
    let i = t.find(" def ")?;
    let kind = t[..i].trim();
    if kind.is_empty() || !kind.chars().all(|c| c.is_ascii_lowercase() || c == ' ') {
        return None;
    }
    let name: String = t[i + 5..].trim().chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
    (!name.is_empty()).then(|| (kind.to_string(), name))
}

fn parse(src: &str, v: &mut Vocabulary) {
    let body = strip_comments(src);

    // Enums, body and all — `enum def X { a; b; }` may span lines.
    let mut rest = body.as_str();
    while let Some(i) = rest.find("enum def ") {
        let after = &rest[i + 9..];
        let name: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        let Some(open) = after.find('{') else { break };
        let Some(close) = after[open..].find('}') else { break };
        let members: Vec<String> = after[open + 1..open + close]
            .split(';')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if !name.is_empty() {
            v.enums.insert(name, members);
        }
        rest = &after[open + close..];
    }

    // Types, their supertype, and their attributes — tracked by brace depth so an attribute is
    // attributed to the def that encloses it rather than to whichever def was seen last.
    let mut stack: Vec<(String, i32)> = Vec::new();
    let mut depth: i32 = 0;
    for line in body.lines() {
        if let Some((kind, name)) = def_on_line(line) {
            if kind == "enum" {
                // handled above
            } else if kind == "metadata" {
                v.markers.insert(name);
            } else {
                if let Some(sup) = line.split(":>").nth(1) {
                    let s: String = sup.trim().chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                    if !s.is_empty() {
                        v.supertype.insert(name.clone(), s);
                    }
                }
                v.attrs.entry(name.clone()).or_default();
                if line.contains('{') {
                    stack.push((name, depth));
                }
            }
        } else if let Some((owner, _)) = stack.last() {
            let t = line.trim();
            for kw in ["attribute ", "ref ", "port "] {
                if let Some(r) = t.strip_prefix(kw) {
                    if let Some(n) = r.split(':').next() {
                        let n = n.trim().trim_end_matches("[0..1]").trim();
                        if !n.is_empty() && n.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            v.attrs.entry(owner.clone()).or_default().insert(n.to_string());
                        }
                    }
                }
            }
        }
        depth += i32::try_from(line.matches('{').count()).unwrap_or(0)
            - i32::try_from(line.matches('}').count()).unwrap_or(0);
        while stack.last().is_some_and(|(_, d)| depth <= *d) {
            stack.pop();
        }
    }
}

fn walk(dir: &Dir<'_>, v: &mut Vocabulary) {
    for f in dir.files() {
        if f.path().extension().is_some_and(|e| e == "sysml") {
            if let Some(s) = f.contents_utf8() {
                parse(s, v);
            }
        }
    }
    for d in dir.dirs() {
        walk(d, v);
    }
}

/// The engine's schema vocabulary, parsed once.
pub static VOCAB: LazyLock<Vocabulary> = LazyLock::new(|| {
    let mut v = Vocabulary {
        enums: HashMap::new(),
        markers: HashSet::new(),
        attrs: HashMap::new(),
        supertype: HashMap::new(),
    };
    walk(&SCHEMA_DIR, &mut v);
    v
});

/// Edge kinds the model can carry, lowercased for matching.
///
/// DERIVED from two sources deliberately. The `EdgeKind` enum is the declared algebra, and every
/// `metadata def` is an edge marker the parser turns into an edge kind — the engine's real edges are
/// authored as markers (`#DerivedFrom`), so a list built from the enum alone would omit them. The
/// trailing `Edge` on enum members (`satisfyEdge`) is a naming convention in the enum, not part of
/// the kind the parser produces, so it is trimmed.
#[must_use]
pub fn edge_kinds() -> HashSet<String> {
    let mut out: HashSet<String> = VOCAB
        .enums
        .get("EdgeKind")
        .map(|vs| vs.iter().map(|v| v.trim_end_matches("Edge").to_lowercase()).collect())
        .unwrap_or_default();
    out.extend(VOCAB.markers.iter().map(|m| m.to_lowercase()));
    // Structural kinds the PARSER synthesises rather than the schema declaring them: `contains` from
    // nesting, `resultof` from the result naming convention, `succession`/`ordering` from flow, and
    // bare `dependency` for an unmarked edge. They are real kinds a view may traverse, and no schema
    // declaration will ever produce them, so they are named here with the reason attached.
    for k in ["contains", "resultof", "succession", "ordering", "dependency"] {
        out.insert(k.to_string());
    }
    out
}

/// The declared members of an enum, in declaration order. Empty if the enum is not declared.
#[must_use]
pub fn enum_members(name: &str) -> Vec<String> {
    VOCAB.enums.get(name).cloned().unwrap_or_default()
}

/// Every attribute name declared for `type_name`, following `:>` supertypes.
///
/// Returns `None` when the type is not in the engine schema at all — a PROJECT-DECLARED type, whose
/// attributes the engine cannot know and must not judge.
#[must_use]
pub fn declared_attrs(type_name: &str) -> Option<HashSet<String>> {
    let mut cur = type_name.to_string();
    let mut seen = HashSet::new();
    let mut out: HashSet<String> = HashSet::new();
    let mut found = false;
    while let Some(direct) = VOCAB.attrs.get(&cur) {
        found = true;
        out.extend(direct.iter().cloned());
        let Some(next) = VOCAB.supertype.get(&cur) else { break };
        if !seen.insert(cur.clone()) {
            break;
        }
        cur.clone_from(next);
    }
    found.then_some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_def_kind_in_the_schema_is_recognised() {
        // The allow-list bug, encoded: `occurrence def` and `verification def` are real and were
        // silently missed by an audit script that knew only five kinds.
        assert!(declared_attrs("TestResult").is_some_and(|a| a.contains("judgedBy")), "occurrence def");
        assert!(declared_attrs("Test").is_some_and(|a| a.contains("procedureText")), "verification def");
        assert!(declared_attrs("Need").is_some_and(|a| a.contains("statement")), "requirement def");
        assert!(declared_attrs("Claim").is_some_and(|a| a.contains("claimedItem")), "part def");
    }

    #[test]
    fn attributes_are_inherited_through_specialization() {
        // Claim :> Element, so `id` and `title` are legal on a Claim without being redeclared.
        let a = declared_attrs("Claim").expect("Claim is an engine type");
        assert!(a.contains("id") && a.contains("createdAt"), "inherited from Element: {a:?}");
    }

    #[test]
    fn a_project_declared_type_is_unknown_not_empty() {
        assert!(declared_attrs("SomeProjectType").is_none(), "unknown must be None, never an empty set");
    }

    #[test]
    fn enum_order_is_preserved_because_ranking_depends_on_it() {
        let r = enum_members("RiskClass");
        assert_eq!(r.first().map(String::as_str), Some("dataLoss"), "RiskClass is declared worst-first");
        assert!(r.len() > 3, "got {r:?}");
    }

    #[test]
    fn edge_kinds_cover_what_the_view_layer_used_to_reject() {
        let e = edge_kinds();
        for k in ["derivedfrom", "covers", "dispositions", "satisfy", "verify", "dependson"] {
            assert!(e.contains(k), "`{k}` must be a known edge kind (issue119)");
        }
    }

    #[test]
    fn comments_are_stripped_before_structure_is_read() {
        let src = "/* metadata def NotReal; */\n// metadata def AlsoNotReal;\nmetadata def Actual;";
        let mut v = Vocabulary { enums: HashMap::new(), markers: HashSet::new(), attrs: HashMap::new(), supertype: HashMap::new() };
        parse(src, &mut v);
        assert_eq!(v.markers.len(), 1, "prose about the schema is not schema (issue099): {:?}", v.markers);
        assert!(v.markers.contains("Actual"));
    }
}
