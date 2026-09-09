//! tests_bind_to_properties — the control for issue388 (D0401).
//!
//! A test that reads program source (`include_str!` / `read_to_string` of a `.rs` file) and asserts that
//! a CODE-SHAPED literal is in it is bound to how the code is spelled, not to what it does: it passes
//! through a regression that keeps the tokens and fails through a refactor that keeps the behaviour, and
//! it teaches the next editor to update the needle rather than to ask what the test was for. main.rs
//! carried one whose own doc comment said it replaced a test bound to a mechanism rather than a property.
//!
//! This file walks every test region in keel-cli and REFUSES that shape. Per offender the report names
//! the file and line, the test, the literal, the property to bind to instead (the first sentence of the
//! test's own doc comment - every offender found so far had already stated it there) and any behavioural
//! equivalent the doc names that exists in the surface.
//!
//! Outside the rule by construction - the legitimate source-reading tests on 2026-09-09: a NEGATED
//! containment (an offender scan asserting a set is empty), a needle built by format! from a catalogue,
//! a needle that is a variable, and a prose needle carrying none of the code tokens.
//!
//! D0388: the discriminator runs against known-positive and known-negative fixtures (`probe_*` below)
//! before the real tree is read. The fixtures ARE the offending shape, so this file excludes itself.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// A literal carrying any of these is a statement, a branch, a binding, a comparison or a call - the
/// spelling of a mechanism, never a property.
const CODE_TOKENS: &[&str] = &[";", "{", "}", "==", "!=", "return ", "let ", "if ", "fn ", "=>", "()"];

fn code_shaped(lit: &str) -> bool {
    CODE_TOKENS.iter().any(|t| lit.contains(t))
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Finding {
    file: String,
    line: usize,
    test: String,
    literal: String,
    property: String,
    equivalents: Vec<String>,
}

impl Finding {
    fn render(&self) -> String {
        let eq = if self.equivalents.is_empty() { "none found".to_string() } else { self.equivalents.join(", ") };
        format!(
            "{}:{} {} - asserts the source contains `{}`; bind to: {}; behavioural equivalent: {}",
            self.file, self.line, self.test, self.literal, self.property, eq
        )
    }
}

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("keel-cli sits in the repo").to_path_buf()
}

fn is_ident_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

/// Whole-word occurrence of `ident` in `text`.
fn mentions(text: &str, ident: &str) -> bool {
    let b = text.as_bytes();
    let mut from = 0;
    while let Some(i) = text[from..].find(ident) {
        let s = from + i;
        let e = s + ident.len();
        let before = s == 0 || !is_ident_char(b[s - 1]);
        let after = e == b.len() || !is_ident_char(b[e]);
        if before && after {
            return true;
        }
        from = s + 1;
    }
    false
}

/// The string literal starting at `text[0]` (`"…"`, `r"…"`, `r#"…"#`): its raw contents and the index
/// just past it. None when `text` does not start a literal.
fn string_literal_at(text: &str) -> Option<(String, usize)> {
    let b = text.as_bytes();
    if b.first() == Some(&b'"') {
        let mut i = 1;
        while i < b.len() {
            match b[i] {
                b'\\' => i += 2,
                b'"' => return Some((text[1..i].to_string(), i + 1)),
                _ => i += 1,
            }
        }
        return None;
    }
    if b.first() == Some(&b'r') {
        let mut hashes = 0;
        while b.get(1 + hashes) == Some(&b'#') {
            hashes += 1;
        }
        if b.get(1 + hashes) != Some(&b'"') {
            return None;
        }
        let open = 2 + hashes;
        let close = format!("\"{}", "#".repeat(hashes));
        let end = text[open..].find(&close)? + open;
        return Some((text[open..end].to_string(), end + close.len()));
    }
    None
}

/// Index of the `)` matching the `(` at `open`, skipping string literals; None when unbalanced.
fn matching_paren(text: &str, open: usize) -> Option<usize> {
    let b = text.as_bytes();
    let mut depth = 0i32;
    let mut i = open;
    while i < b.len() {
        match b[i] {
            b'"' | b'r' if string_literal_at(&text[i..]).is_some() && (b[i] == b'"' || i == 0 || !is_ident_char(b[i - 1])) => {
                let (_, len) = string_literal_at(&text[i..]).unwrap_or_default();
                i += len.max(1);
                continue;
            }
            b'\'' if i + 2 < b.len() && (b[i + 2] == b'\'' || (b[i + 1] == b'\\' && b.get(i + 3) == Some(&b'\''))) => {
                i += if b[i + 1] == b'\\' { 4 } else { 3 };
                continue;
            }
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// Split `text` at depth-0 occurrences of any of `seps`, outside string literals.
fn split_top<'a>(text: &'a str, seps: &[&str]) -> Vec<&'a str> {
    split_on(text, seps, false)
}

/// Split `text` at occurrences of any of `seps` outside string literals - at depth 0 only, or at any
/// depth (`any_depth`: a statement inside a loop or closure is still a statement).
fn split_on<'a>(text: &'a str, seps: &[&str], any_depth: bool) -> Vec<&'a str> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    let mut i = 0;
    while i < b.len() {
        if (b[i] == b'"' || (b[i] == b'r' && (i == 0 || !is_ident_char(b[i - 1])))) && string_literal_at(&text[i..]).is_some() {
            let (_, len) = string_literal_at(&text[i..]).unwrap_or_default();
            i += len.max(1);
            continue;
        }
        match b[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            _ => {}
        }
        if (any_depth || depth == 0) && text.is_char_boundary(i) {
            if let Some(sep) = seps.iter().find(|s| text[i..].starts_with(**s)) {
                out.push(&text[start..i]);
                i += sep.len();
                start = i;
                continue;
            }
        }
        i += 1;
    }
    out.push(&text[start..]);
    out
}

fn ident_after<'a>(text: &'a str, keyword: &str) -> Option<&'a str> {
    let mut rest = text.find(keyword)? + keyword.len();
    let t = text[rest..].trim_start_matches("mut ").trim_start();
    rest = text.len() - t.len();
    let end = text[rest..].bytes().take_while(|c| is_ident_char(*c)).count();
    (end > 0).then(|| &text[rest..rest + end])
}

/// Every identifier bound to program source in `text` - a `let`, `const` or `static` whose initialiser
/// reads a `.rs` file - with the position of the read.
fn source_bindings(text: &str) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    for marker in ["include_str!(", "read_to_string("] {
        let mut from = 0;
        while let Some(i) = text[from..].find(marker) {
            let at = from + i;
            from = at + marker.len();
            let open = at + marker.len() - 1;
            let Some(close) = matching_paren(text, open) else { continue };
            if !text[open..close].contains(".rs\"") {
                continue;
            }
            let stmt_start = text[..at].rfind([';', '{', '}']).map_or(0, |p| p + 1);
            let stmt = &text[stmt_start..at];
            for kw in ["let ", "const ", "static "] {
                if let Some(id) = ident_after(stmt, kw) {
                    out.push((id.to_string(), at));
                    break;
                }
            }
        }
    }
    out
}

struct TestFn {
    name: String,
    doc: String,
    start: usize,
    end: usize,
}

fn line_of(text: &str, at: usize) -> usize {
    text[..at].matches('\n').count() + 1
}

/// The test functions in `text` from `region_start`, each spanning from its attribute to the next one.
fn test_fns(text: &str, region_start: usize) -> Vec<TestFn> {
    let mut attrs = Vec::new();
    for marker in ["#[test]", "#[tokio::test"] {
        let mut from = region_start;
        while let Some(i) = text[from..].find(marker) {
            attrs.push(from + i);
            from = from + i + marker.len();
        }
    }
    attrs.sort_unstable();
    let mut out = Vec::new();
    for (k, &attr) in attrs.iter().enumerate() {
        let end = attrs.get(k + 1).copied().unwrap_or(text.len());
        let Some(fn_at) = text[attr..end].find("fn ").map(|p| attr + p) else { continue };
        let Some(name) = ident_after(&text[fn_at..end], "fn ") else { continue };
        // The doc comment: the `///` lines immediately above the attribute, through other attributes.
        let mut doc = Vec::new();
        let line_start = text[..attr].rfind('\n').map_or(0, |p| p + 1);
        let mut above = &text[..line_start];
        while let Some(prev) = above.trim_end_matches('\n').rfind('\n').map(|p| p + 1).or(Some(0)) {
            let l = above[prev..].trim();
            if let Some(d) = l.strip_prefix("///") {
                doc.push(d.trim().to_string());
            } else if !l.starts_with("#[") {
                break;
            }
            if prev == 0 {
                break;
            }
            above = &above[..prev];
        }
        doc.reverse();
        out.push(TestFn { name: name.to_string(), doc: doc.join(" "), start: attr, end });
    }
    out
}

fn first_sentence(doc: &str) -> String {
    if doc.trim().is_empty() {
        return "(no doc comment - state the property first)".to_string();
    }
    let d = doc.trim();
    let end = d.find(". ").map_or(d.len(), |p| p + 1);
    d[..end].split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Bound identifiers reachable inside `body`: `seed` plus every `let` whose right-hand side mentions one,
/// to a fixpoint.
fn derived_bindings(body: &str, seed: &BTreeSet<String>) -> BTreeSet<String> {
    let mut bound = seed.clone();
    loop {
        let before = bound.len();
        for stmt in split_on(body, &[";", "{"], true) {
            if let Some(let_at) = stmt.find("let ") {
                if let (Some(id), Some(eq)) = (ident_after(&stmt[let_at..], "let "), stmt[let_at..].find('=')) {
                    let rhs = &stmt[let_at + eq + 1..];
                    if bound.iter().any(|b| mentions(rhs, b)) {
                        bound.insert(id.to_string());
                    }
                }
            }
            if let Some(for_at) = stmt.find("for ") {
                if let (Some(id), Some(in_at)) = (ident_after(&stmt[for_at..], "for "), stmt[for_at..].find(" in ")) {
                    let rhs = &stmt[for_at + in_at + 4..];
                    if bound.iter().any(|b| mentions(rhs, b)) {
                        bound.insert(id.to_string());
                    }
                }
            }
        }
        if bound.len() == before {
            return bound;
        }
    }
}

/// The findings in one file's test region. `known` is every test function name and test file stem in
/// the surface, used to name behavioural equivalents the doc comment points at.
fn scan(file: &str, text: &str, region_start: usize, known: &BTreeSet<String>) -> Vec<Finding> {
    let fns = test_fns(text, region_start);
    let bindings = source_bindings(text);
    let module_level: BTreeSet<String> = bindings
        .iter()
        .filter(|(_, at)| !fns.iter().any(|f| f.start <= *at && *at < f.end) || {
            // a const/static is visible module-wide wherever it is declared
            let stmt_start = text[..*at].rfind([';', '{', '}']).map_or(0, |p| p + 1);
            let stmt = &text[stmt_start..*at];
            stmt.contains("const ") || stmt.contains("static ")
        })
        .map(|(id, _)| id.clone())
        .collect();
    let mut out = Vec::new();
    for f in &fns {
        let body = &text[f.start..f.end];
        let mut seed = module_level.clone();
        seed.extend(bindings.iter().filter(|(_, at)| f.start <= *at && *at < f.end).map(|(id, _)| id.clone()));
        if seed.is_empty() {
            continue;
        }
        let bound = derived_bindings(body, &seed);
        for macro_name in ["assert!(", "assert_eq!(", "assert_ne!("] {
            let mut from = 0;
            while let Some(i) = body[from..].find(macro_name) {
                let at = from + i;
                from = at + macro_name.len();
                let open = at + macro_name.len() - 1;
                let Some(close) = matching_paren(body, open) else { continue };
                let args = &body[open + 1..close];
                for arg in split_top(args, &[","]) {
                    let arg = arg.trim();
                    if arg.starts_with('"') {
                        continue; // the message
                    }
                    for term in split_top(arg, &["&&", "||"]) {
                        let term = term.trim();
                        if term.starts_with('!') {
                            continue; // a negated containment is an offender scan
                        }
                        for call in [".contains(", ".matches("] {
                            let Some(c) = term.find(call) else { continue };
                            let receiver = &term[..c];
                            if !bound.iter().any(|b| mentions(receiver, b)) {
                                continue;
                            }
                            let Some((lit, _)) = string_literal_at(&term[c + call.len()..]) else { continue };
                            if !code_shaped(&lit) {
                                continue;
                            }
                            let equivalents = f
                                .doc
                                .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
                                .filter(|w| w.contains('_') && *w != f.name && known.contains(*w))
                                .map(str::to_string)
                                .collect::<BTreeSet<_>>()
                                .into_iter()
                                .collect();
                            out.push(Finding {
                                file: file.to_string(),
                                line: line_of(text, f.start + at),
                                test: f.name.clone(),
                                literal: lit,
                                property: first_sentence(&f.doc),
                                equivalents,
                            });
                        }
                    }
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn rs_files_under(dir: &Path, recurse: bool, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            if recurse {
                rs_files_under(&p, true, out);
            }
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// Every test surface in keel-cli: (display path, text, region start), this file excluded.
fn surface() -> Vec<(String, String, usize)> {
    let root = repo();
    let mut files = Vec::new();
    rs_files_under(&root.join("keel-cli/src"), true, &mut files);
    rs_files_under(&root.join("keel-cli/tests"), false, &mut files);
    files.sort();
    let mut out = Vec::new();
    for p in files {
        if p.file_name().is_some_and(|n| n == "tests_bind_to_properties.rs") {
            continue;
        }
        let text = std::fs::read_to_string(&p).expect("a source file is readable");
        let display = p.strip_prefix(&root).unwrap_or(&p).to_string_lossy().replace('\\', "/");
        let region = if display.starts_with("keel-cli/src/") {
            match text.find("#[cfg(test)]") {
                Some(r) => r,
                None => continue,
            }
        } else {
            0
        };
        out.push((display, text, region));
    }
    out
}

fn known_tests(surface: &[(String, String, usize)]) -> BTreeSet<String> {
    let mut known = BTreeSet::new();
    for (file, text, region) in surface {
        known.extend(test_fns(text, *region).into_iter().map(|f| f.name));
        if file.starts_with("keel-cli/tests/") {
            if let Some(stem) = Path::new(file).file_stem() {
                known.insert(stem.to_string_lossy().into_owned());
            }
        }
    }
    known
}

// ── PROBES (D0388): the check against its known cases, before the tree ────────────────────────────

fn known() -> BTreeSet<String> {
    ["turn_boundary_gate_allows_a_clean_tree_and_blocks_a_dishonest_one", "stop_says_nothing_about_decisions"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

/// KNOWN POSITIVE - the issue388 shape as it stood in main.rs: a module-level const of the file, a
/// slice derived from it through two lets, and a positive contains of a branch.
const POSITIVE_TURN_BOUNDARY: &str = r##"
#[cfg(test)]
mod t {
    /// The turn boundary blocks ONLY on dishonest state. `turn_boundary_gate_allows_a_clean_tree_and_blocks_a_dishonest_one`
    /// and `stop_says_nothing_about_decisions` hold that behaviourally.
    #[test]
    fn the_turn_boundary_blocks_only_on_dishonest_state() {
        const MAIN_RS: &str = include_str!("main.rs");
        let at = MAIN_RS.find("fn hook_stop(").unwrap_or(0);
        let hook = &MAIN_RS[at..];
        let block = hook.find("\"decision\": \"block\"").unwrap_or(0);
        assert!(
            hook[..block].contains("if problems.is_empty() {"),
            "the block path must sit behind the problems check"
        );
    }
}
"##;

#[test]
fn probe_positive_the_turn_boundary_shape_is_refused_with_its_property_and_equivalents() {
    let f = scan("keel-cli/src/main.rs", POSITIVE_TURN_BOUNDARY, 0, &known());
    assert_eq!(f.len(), 1, "one finding expected: {f:?}");
    assert_eq!(f[0].literal, "if problems.is_empty() {");
    assert_eq!(f[0].test, "the_turn_boundary_blocks_only_on_dishonest_state");
    assert_eq!(f[0].property, "The turn boundary blocks ONLY on dishonest state.");
    assert_eq!(
        f[0].equivalents,
        vec!["stop_says_nothing_about_decisions".to_string(), "turn_boundary_gate_allows_a_clean_tree_and_blocks_a_dishonest_one".to_string()]
    );
    assert_eq!(f[0].line, 12, "the line is the assert's, so the report is clickable at the defect");
}

/// KNOWN POSITIVE - the guards.rs shape: a fn-local let through a crate helper, a comparison literal.
const POSITIVE_RUNNER_FLAGS: &str = r##"
#[cfg(test)]
mod scan_count_tests {
    #[test]
    fn the_runner_flags_a_violation_against_an_empty_scan() {
        let src = crate::corpus::read_to_string("src/guards.rs").expect("guards.rs is readable");
        assert!(
            src.contains("self.scanned == 0 && !self.violations.is_empty()"),
            "the self-contradiction check must survive in GuardReport::print"
        );
    }
}
"##;

#[test]
fn probe_positive_the_runner_flags_shape_is_refused_and_an_absent_doc_is_said() {
    let f = scan("keel-cli/src/guards.rs", POSITIVE_RUNNER_FLAGS, 0, &known());
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].literal, "self.scanned == 0 && !self.violations.is_empty()");
    assert_eq!(f[0].property, "(no doc comment - state the property first)");
    assert!(f[0].equivalents.is_empty());
}

/// KNOWN POSITIVE - the fingerprint.rs shape: std read, two derivations, a contains AND an assert_eq
/// on a count of a call literal. Both are the same defect and both are named.
const POSITIVE_EPOCH: &str = r##"
#[cfg(test)]
mod tests {
    /// A READ must not advance the epoch, and a WRITE must (D0167). Getting it backwards is invisible.
    #[test]
    fn the_epoch_is_advanced_by_writes_and_observations_not_by_reads() {
        let src = std::fs::read_to_string("src/serve.rs").expect("serve.rs is readable");
        let mw = src.split_once("async fn log_request").expect("the request middleware exists").1;
        let body = &mw[..mw.find("zzz").unwrap_or(mw.len())];
        assert!(
            body.contains("let writes = method != axum::http::Method::GET"),
            "the middleware must distinguish reads from writes"
        );
        assert_eq!(body.matches("crate::fingerprint::new_epoch()").count(), 2, "before and after");
    }
}
"##;

#[test]
fn probe_positive_the_epoch_shape_is_refused_on_both_asserts() {
    let f = scan("keel-cli/src/fingerprint.rs", POSITIVE_EPOCH, 0, &known());
    let lits: Vec<&str> = f.iter().map(|x| x.literal.as_str()).collect();
    assert_eq!(lits, vec!["let writes = method != axum::http::Method::GET", "crate::fingerprint::new_epoch()"], "findings come in line order: {f:?}");
    assert!(f.iter().all(|x| x.property == "A READ must not advance the epoch, and a WRITE must (D0167)."));
}

/// KNOWN NEGATIVE - the four legitimate shapes: a format!-built catalogue arm, a prose needle, an
/// offender scan asserting emptiness, and a negated containment. Plus a non-source string that happens
/// to contain code, which is outside the rule because nothing binds it to a `.rs` file.
const NEGATIVES: &str = r##"
#[cfg(test)]
mod tests {
    const SERVE_RS: &str = include_str!("serve.rs");

    #[test]
    fn every_enforced_guard_dispatches() {
        const GUARDS_RS: &str = include_str!("guards.rs");
        let dispatch = GUARDS_RS.split_once("pub fn run_one(").expect("run_one must exist").1;
        for name in NAMES {
            let arm = format!("\"{name}\" =>");
            assert!(dispatch.contains(&arm), "guard `{name}` has no arm in run_one");
        }
    }

    #[test]
    fn the_checks_own_scope_is_stated_where_someone_will_read_it() {
        let src = include_str!("../src/adoption_check.rs");
        assert!(src.contains("WHAT THIS CANNOT CATCH"), "say what the check does NOT cover");
        for needle in ["issue259", "issue263"] {
            assert!(src.contains(needle), "lost `{needle}`");
        }
    }

    #[test]
    fn no_new_engine_surface_writer_is_unrepresented() {
        let src = std::fs::read_to_string("keel-cli/src/main.rs").expect("main.rs");
        let mut offenders = Vec::new();
        for line in src.lines() {
            if line.contains("fn foo() {") {
                offenders.push(line);
            }
        }
        assert!(offenders.is_empty(), "unrepresented: {offenders:?}");
    }

    #[test]
    fn an_unconditional_bump_is_the_regression() {
        let src = std::fs::read_to_string("src/serve.rs").expect("serve.rs is readable");
        assert!(!src.contains("crate::fingerprint::new_epoch();"), "an UNCONDITIONAL bump");
        assert!(SERVE_RS.contains("no-store"), "the console must be served no-store");
    }

    #[test]
    fn rendered_output_may_look_like_code() {
        let rendered = render_help();
        assert!(rendered.contains("fn main() {"), "a codegen test is about its output, not its source");
    }
}
"##;

#[test]
fn probe_negative_the_legitimate_source_reading_shapes_are_not_refused() {
    let f = scan("keel-cli/src/x.rs", NEGATIVES, 0, &known());
    assert!(f.is_empty(), "no finding expected on the legitimate shapes: {f:?}");
}

#[test]
fn probe_the_test_region_of_a_src_file_starts_at_cfg_test() {
    // The same offending shape ABOVE `#[cfg(test)]` is not a test and is not scanned.
    let text = format!("fn helper() {{\n    let src = include_str!(\"main.rs\");\n    assert!(src.contains(\"fn x() {{\"));\n}}\n{POSITIVE_RUNNER_FLAGS}");
    let region = text.find("#[cfg(test)]").expect("fixture has a test region");
    let f = scan("keel-cli/src/y.rs", &text, region, &known());
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].test, "the_runner_flags_a_violation_against_an_empty_scan");
}

// ── THE CONTROL ─────────────────────────────────────────────────────────────────────────────────────

/// D0401: no test in keel-cli asserts that program source contains a code-shaped literal. Each finding
/// names the property the test should bind to and any behavioural equivalent already present.
#[test]
fn every_source_reading_test_binds_to_a_property() {
    let surface = surface();
    assert!(surface.len() > 40, "the surface walk found {} files - the scan is mis-aimed", surface.len());
    let known = known_tests(&surface);
    let mut findings = Vec::new();
    for (file, text, region) in &surface {
        findings.extend(scan(file, text, *region, &known));
    }
    assert!(
        findings.is_empty(),
        "{} test(s) assert that program source contains a code-shaped literal (issue388, D0401). Such a test \
         passes through a regression that keeps the tokens and fails through a refactor that keeps the \
         behaviour. Bind each to the property its doc comment states, behaviourally:\n  {}",
        findings.len(),
        findings.iter().map(Finding::render).collect::<Vec<_>>().join("\n  ")
    );
}
