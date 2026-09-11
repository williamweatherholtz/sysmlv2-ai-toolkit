//! A control's PROOF state (D0360): has any test ever constructed this guard's defect and asserted
//! that the guard reports it?
//!
//! `keel show controls` already computes two things about a control - DECLARED (an edge says it
//! covers a hazard, D0195) and ARMED (a probe says it can physically fire, D0217). Neither answers
//! the question that separates a control from a control-shaped thing, and a clean pass on a clean
//! tree is not evidence about what a control catches. This module computes the third state, per
//! enforced guard, from the test corpus in the tree:
//!
//! - `PROVEN`       - the guard's name occurs inside a `#[test]` function body that asserts a
//!   failure (a non-empty violation set, a `FAIL` verdict, or a refusal).
//! - `UNPROVEN`     - the name occurs in a test body, but no such body asserts a failure: the guard
//!   was shown passing, never shown catching.
//! - `UNDETERMINED` - no test body names the guard at all. The heuristic is textual, so a guard a
//!   fixture exercises without naming it lands here; "we cannot tell" is a different answer from
//!   "it was not", and the surface says which (dcControlProofStateIsComputed).
//!
//! It is an INDICATOR, not a gate (D0088): a threshold on the proven share would be answered by
//! writing whichever test moves the number. The unproven and undetermined LISTS are the actionable
//! output; the share is context.
//!
//! The heuristic is the one `.engine/tools/guard_proof_census.py` runs, kept in step on purpose:
//! the script is the independent reference the binary is held against, and the two must agree on
//! one tree. Its first version searched a text window around every occurrence of a guard's name,
//! matched the `GUARD_NAMES` registry itself, and reported every guard proven - the lesson is why
//! only TEST FUNCTION BODIES are read. STATED LIMITATION, shared with the script: `name in body` is
//! a substring test, so a one-word guard (`issues`, `actors`, `charter`) can be credited by prose.

use std::path::{Path, PathBuf};

use crate::json::Json;

/// The three computed proof states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofState {
    /// Named in a test body that asserts a failure.
    Proven,
    /// Named in a test body; none of them asserts a failure.
    Unproven,
    /// Named in no test body - the textual heuristic cannot tell.
    Undetermined,
}

impl ProofState {
    /// The word the surface prints.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Proven => "PROVEN",
            Self::Unproven => "UNPROVEN",
            Self::Undetermined => "UNDETERMINED",
        }
    }
}

/// One guard's computed proof state and the tests that carry it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardProof {
    pub guard: String,
    pub state: ProofState,
    /// `file::test_fn` for every test body naming the guard.
    pub named_in: Vec<String>,
    /// The subset of `named_in` whose body asserts a failure.
    pub proven_by: Vec<String>,
}

/// The census over one tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofCensus {
    /// Whether `keel-cli/tests` or `keel-cli/src` existed under the root. Without a corpus every
    /// guard is UNDETERMINED and the surface says why rather than listing seventy gaps.
    pub corpus_present: bool,
    pub test_functions: usize,
    pub guards: Vec<GuardProof>,
}

impl ProofCensus {
    fn count(&self, state: ProofState) -> usize {
        self.guards.iter().filter(|g| g.state == state).count()
    }
    fn names(&self, state: ProofState) -> Vec<Json> {
        self.guards.iter().filter(|g| g.state == state).map(|g| Json::s(g.guard.clone())).collect()
    }
    /// Proven count.
    #[must_use]
    pub fn proven(&self) -> usize {
        self.count(ProofState::Proven)
    }
    /// Unproven count.
    #[must_use]
    pub fn unproven(&self) -> usize {
        self.count(ProofState::Unproven)
    }
    /// Undetermined count.
    #[must_use]
    pub fn undetermined(&self) -> usize {
        self.count(ProofState::Undetermined)
    }

    /// The `controlProof` member of `keel show controls`.
    #[must_use]
    pub fn to_json(&self) -> Json {
        let n = |c: usize| Json::Int(i64::try_from(c).unwrap_or(i64::MAX));
        let states: Vec<Json> = self
            .guards
            .iter()
            .map(|g| {
                Json::Obj(vec![
                    ("guard".to_string(), Json::s(g.guard.clone())),
                    ("proof".to_string(), Json::s(g.state.label())),
                    ("provenBy".to_string(), Json::Arr(g.proven_by.iter().map(|s| Json::s(s.clone())).collect())),
                    ("namedIn".to_string(), Json::Arr(g.named_in.iter().map(|s| Json::s(s.clone())).collect())),
                ])
            })
            .collect();
        let total = self.guards.len();
        let share = if self.corpus_present {
            format!("{} of {total} enforced guards have a demonstrated catch; {} shown only passing; {} named in no test body", self.proven(), self.unproven(), self.undetermined())
        } else {
            format!("no test corpus under this root (keel-cli/tests, keel-cli/src) - all {total} guards UNDETERMINED here; run at the engine's own root")
        };
        Json::Obj(vec![
            ("note".to_string(), Json::s("PROOF is the third state beside DECLARED and ARMED (D0360): PROVEN = a test body names the guard and asserts a failure (it was shown catching); UNPROVEN = named only in tests that assert no failure (shown passing, never catching); UNDETERMINED = named in no test body, so the textual heuristic cannot tell. An indicator, not a gate (D0088): the lists are the actionable output, the share is context. Same heuristic as .engine/tools/guard_proof_census.py; the two must agree on one tree.")),
            ("corpusPresent".to_string(), Json::Bool(self.corpus_present)),
            ("testFunctionsScanned".to_string(), n(self.test_functions)),
            ("guards".to_string(), n(total)),
            ("proven".to_string(), n(self.proven())),
            ("unproven".to_string(), n(self.unproven())),
            ("undetermined".to_string(), n(self.undetermined())),
            ("share".to_string(), Json::s(share)),
            ("unprovenList".to_string(), Json::Arr(self.names(ProofState::Unproven))),
            ("undeterminedList".to_string(), Json::Arr(self.names(ProofState::Undetermined))),
            ("states".to_string(), Json::Arr(states)),
        ])
    }
}

/// The proof census of every enforced guard (`GUARD_NAMES`) over the test corpus under `root`.
#[must_use]
pub fn census(root: &Path) -> ProofCensus {
    census_over(root, &crate::guards::GUARD_NAMES)
}

/// The census for an explicit guard list - the fixture entry point, and what [`census`] calls.
#[must_use]
pub fn census_over(root: &Path, guard_names: &[&str]) -> ProofCensus {
    let (bodies, corpus_present) = corpus_bodies(root);
    census_of_bodies(&bodies, guard_names, corpus_present)
}

/// The census over already-extracted `(file, test_fn, body)` triples.
#[must_use]
pub fn census_of_bodies(bodies: &[(String, String, String)], guard_names: &[&str], corpus_present: bool) -> ProofCensus {
    let guards = guard_names
        .iter()
        .map(|g| {
            let mut named_in = Vec::new();
            let mut proven_by = Vec::new();
            for (file, func, body) in bodies {
                if !body.contains(*g) {
                    continue;
                }
                let at = format!("{file}::{func}");
                if asserts_a_failure(body) {
                    proven_by.push(at.clone());
                }
                named_in.push(at);
            }
            let state = if !proven_by.is_empty() {
                ProofState::Proven
            } else if named_in.is_empty() {
                ProofState::Undetermined
            } else {
                ProofState::Unproven
            };
            GuardProof { guard: (*g).to_string(), state, named_in, proven_by }
        })
        .collect();
    ProofCensus { corpus_present, test_functions: bodies.len(), guards }
}

/// Every test body in the corpus: `keel-cli/tests/*.rs` (one level) and `keel-cli/src/**/*.rs`.
/// The bool says whether either directory existed.
fn corpus_bodies(root: &Path) -> (Vec<(String, String, String)>, bool) {
    let tests = root.join("keel-cli").join("tests");
    let src = root.join("keel-cli").join("src");
    let mut files: Vec<PathBuf> = Vec::new();
    let mut present = false;
    if let Ok(rd) = std::fs::read_dir(&tests) {
        present = true;
        files.extend(rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "rs")));
    }
    if src.is_dir() {
        present = true;
        walk_rs(&src, &mut files);
    }
    files.sort();
    let mut out = Vec::new();
    for path in files {
        let Ok(bytes) = std::fs::read(&path) else { continue };
        let text = String::from_utf8_lossy(&bytes);
        let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
        for (func, body) in test_bodies(&text) {
            out.push((fname.clone(), func, body));
        }
    }
    (out, present)
}

fn walk_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for entry in rd.flatten() {
        let p = entry.path();
        if p.is_dir() {
            walk_rs(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// Each `#[test]` function's `(name, body)`, the body by brace matching from the signature's `{`
/// (inclusive) to its closing `}` (exclusive) - or to the end of the text if it never closes.
///
/// The signature must follow the attribute within 200 characters and read
/// `fn <name>(<params>) {` with a lower-case identifier and no return type, so a `-> Result` test
/// is not scanned. That is the census script's regex, kept identical so the two agree.
#[must_use]
pub fn test_bodies(text: &str) -> Vec<(String, String)> {
    let chars: Vec<char> = text.chars().collect();
    let attr: Vec<char> = "#[test]".chars().collect();
    let mut out = Vec::new();
    let mut search = 0usize;
    while let Some(start) = find_from(&chars, &attr, search) {
        let after = start + attr.len();
        let mut matched: Option<(String, usize)> = None;
        // Lazy gap: the first `fn` within 200 chars whose signature completes.
        for gap in 0..=200 {
            let p = after + gap;
            if p > chars.len() {
                break;
            }
            if let Some(hit) = signature_at(&chars, p) {
                matched = Some(hit);
                break;
            }
        }
        let Some((name, open)) = matched else {
            search = start + 1;
            continue;
        };
        let close = matching_brace(&chars, open);
        let body: String = chars.get(open..close).unwrap_or_default().iter().collect();
        out.push((name, body));
        search = open;
    }
    out
}

/// `fn\s+([a-z0-9_]+)\s*\([^)]*\)\s*\{` anchored at `p`; returns the name and the `{` index.
fn signature_at(chars: &[char], p: usize) -> Option<(String, usize)> {
    if !(chars.get(p) == Some(&'f') && chars.get(p + 1) == Some(&'n')) {
        return None;
    }
    let mut i = p + 2;
    let ws_start = i;
    while chars.get(i).is_some_and(|c| c.is_whitespace()) {
        i += 1;
    }
    if i == ws_start {
        return None;
    }
    let name_start = i;
    while chars.get(i).is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '_') {
        i += 1;
    }
    if i == name_start {
        return None;
    }
    let name: String = chars.get(name_start..i)?.iter().collect();
    while chars.get(i).is_some_and(|c| c.is_whitespace()) {
        i += 1;
    }
    if chars.get(i) != Some(&'(') {
        return None;
    }
    i += 1;
    while chars.get(i).is_some_and(|c| *c != ')') {
        i += 1;
    }
    if chars.get(i) != Some(&')') {
        return None;
    }
    i += 1;
    while chars.get(i).is_some_and(|c| c.is_whitespace()) {
        i += 1;
    }
    (chars.get(i) == Some(&'{')).then_some((name, i))
}

/// Naive brace matching from `open` (a `{`): the index of its closing `}`, or `chars.len()`.
fn matching_brace(chars: &[char], open: usize) -> usize {
    let mut depth = 0i64;
    let mut j = open;
    while let Some(c) = chars.get(j) {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return j;
                }
            }
            _ => {}
        }
        j += 1;
    }
    chars.len()
}

fn find_from(chars: &[char], needle: &[char], from: usize) -> Option<usize> {
    if needle.is_empty() || from > chars.len() {
        return None;
    }
    (from..=chars.len().checked_sub(needle.len())?).find(|&i| chars.get(i..i + needle.len()) == Some(needle))
}

fn starts_with_at(chars: &[char], at: usize, lit: &str) -> bool {
    (at..).zip(lit.chars()).all(|(i, c)| chars.get(i) == Some(&c))
}

/// `<anchor>[^;]{0,max_gap}<alt>`: after some occurrence of `anchor`, one of `alts` begins within
/// `max_gap` characters with no `;` in between.
fn anchor_then(chars: &[char], anchor: &str, max_gap: usize, alt: &dyn Fn(&[char], usize) -> bool) -> bool {
    let anchor_chars: Vec<char> = anchor.chars().collect();
    let mut from = 0usize;
    while let Some(start) = find_from(chars, &anchor_chars, from) {
        let end = start + anchor_chars.len();
        for gap in 0..=max_gap {
            let at = end + gap;
            if gap > 0 && chars.get(at - 1) == Some(&';') {
                break;
            }
            if at > chars.len() {
                break;
            }
            if alt(chars, at) {
                return true;
            }
        }
        from = start + 1;
    }
    false
}

/// Does a test body assert a FAILURE - a non-empty violation set, a `FAIL` verdict, or a refusal?
///
/// The census script's three tests, verbatim in meaning:
/// `violations[^;]{0,80}(is_empty\(\)|len\(\)|!\s*=|>)`, `"FAIL" in body`, and
/// `assert!\([^;]{0,120}(violation|refus|denied)`.
#[must_use]
pub fn asserts_a_failure(body: &str) -> bool {
    if body.contains("FAIL") {
        return true;
    }
    let chars: Vec<char> = body.chars().collect();
    let violation_alt = |c: &[char], at: usize| -> bool {
        if starts_with_at(c, at, "is_empty()") || starts_with_at(c, at, "len()") || starts_with_at(c, at, ">") {
            return true;
        }
        if c.get(at) != Some(&'!') {
            return false;
        }
        let mut i = at + 1;
        while c.get(i).is_some_and(|x| x.is_whitespace()) {
            i += 1;
        }
        c.get(i) == Some(&'=')
    };
    if anchor_then(&chars, "violations", 80, &violation_alt) {
        return true;
    }
    let refusal_alt = |c: &[char], at: usize| -> bool { starts_with_at(c, at, "violation") || starts_with_at(c, at, "refus") || starts_with_at(c, at, "denied") };
    anchor_then(&chars, "assert!(", 120, &refusal_alt)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// D0388: the known-positive and known-negative are fixed before any real tree is read.
    const POSITIVE: &str = "#[test]\nfn the_guard_catches_it() {\n    let violations = run(\"doc-sync\", &root);\n    assert!(!violations.is_empty(), \"must catch\");\n}\n";
    const NEGATIVE: &str = "#[test]\nfn the_guard_passes_a_clean_tree() {\n    let ok = run(\"doc-sync\", &root);\n    assert!(ok);\n}\n";

    #[test]
    fn a_body_that_asserts_a_non_empty_violation_set_is_a_constructed_failure() {
        let bodies = test_bodies(POSITIVE);
        assert_eq!(bodies.len(), 1);
        assert_eq!(bodies[0].0, "the_guard_catches_it");
        assert!(asserts_a_failure(&bodies[0].1));
    }

    #[test]
    fn a_body_that_only_asserts_a_pass_is_not() {
        let bodies = test_bodies(NEGATIVE);
        assert_eq!(bodies.len(), 1);
        assert!(!asserts_a_failure(&bodies[0].1));
    }

    #[test]
    fn the_three_states_are_distinguishable_and_named_apart() {
        let bodies = vec![
            ("a.rs".to_string(), "p".to_string(), test_bodies(POSITIVE)[0].1.clone()),
            ("b.rs".to_string(), "n".to_string(), test_bodies(NEGATIVE).swap_remove(0).1.replace("doc-sync", "issues")),
        ];
        let c = census_of_bodies(&bodies, &["doc-sync", "issues", "never-named"], true);
        let state = |g: &str| c.guards.iter().find(|x| x.guard == g).map(|x| x.state).unwrap();
        assert_eq!(state("doc-sync"), ProofState::Proven);
        assert_eq!(state("issues"), ProofState::Unproven);
        assert_eq!(state("never-named"), ProofState::Undetermined);
        assert_eq!((c.proven(), c.unproven(), c.undetermined()), (1, 1, 1));
        let dump: String = c.to_json().dump().split_whitespace().collect();
        assert!(dump.contains("\"unprovenList\":[\"issues\"]"), "{dump}");
        assert!(dump.contains("\"undeterminedList\":[\"never-named\"]"), "{dump}");
        assert!(!dump.contains('%'), "the share is context, never a bare percentage: {dump}");
    }

    #[test]
    fn the_failure_words_match_the_census_regexes() {
        assert!(asserts_a_failure("{ assert!(violations.len() > 0); }"));
        assert!(asserts_a_failure("{ assert_ne!(violations.len(), 0) }"));
        assert!(asserts_a_failure("{ assert!(violations != vec![]) }"));
        assert!(asserts_a_failure("{ assert!(out.contains(\"FAIL\")) }"));
        assert!(asserts_a_failure("{ assert!(stderr.contains(\"refused\")) }"));
        assert!(asserts_a_failure("{ assert!(code == 1, \"the write is denied\") }"));
        // A `;` inside the window ends the search: `violations` on one statement, `len()` on the next.
        assert!(!asserts_a_failure("{ let violations = x; let n = y.len(); }"));
        // Beyond the 120-char window after `assert!(` the refusal word does not count.
        let far = format!("{{ assert!({}refused) }}", "a".repeat(121));
        assert!(!asserts_a_failure(&far));
        let near = format!("{{ assert!({}refused) }}", "a".repeat(120));
        assert!(asserts_a_failure(&near));
    }

    #[test]
    fn a_test_returning_result_or_with_an_uppercase_name_is_not_scanned_like_the_census() {
        let text = "#[test]\nfn returns_result() -> Result<(), E> {\n    violations.len()\n}\n#[test]\n#[ignore]\nfn attribute_between() {\n    x\n}\n";
        let bodies = test_bodies(text);
        assert_eq!(bodies.iter().map(|b| b.0.as_str()).collect::<Vec<_>>(), vec!["attribute_between"]);
    }

    #[test]
    fn a_body_is_bounded_by_its_own_braces_and_an_unclosed_one_runs_to_the_end() {
        let text = "#[test]\nfn a() {\n    if x { y }\n    \"doc-sync\"\n}\nfn helper() { \"actors\" }\n#[test]\nfn b() {\n    open";
        let bodies = test_bodies(text);
        assert_eq!(bodies.len(), 2);
        assert!(bodies[0].1.contains("doc-sync") && !bodies[0].1.contains("actors"));
        assert_eq!(bodies[1].1, "{\n    open");
    }

    #[test]
    fn no_corpus_means_every_guard_undetermined_and_the_surface_says_why() {
        let dir = std::env::temp_dir().join(format!("keel-proof-nocorpus-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let c = census_over(&dir, &["doc-sync", "issues"]);
        assert!(!c.corpus_present);
        assert_eq!(c.undetermined(), 2);
        assert!(c.to_json().dump().contains("no test corpus under this root"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
