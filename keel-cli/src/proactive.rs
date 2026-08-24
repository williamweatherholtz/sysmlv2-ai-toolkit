//! Proactive post-edit guidance (D0209 clause 4, dcProactivePostEdit): NON-BLOCKING "that change
//! just broke X" prevention.
//!
//! After the blocking fast tier, the post-edit hook calls this to surface, as guidance the AI can
//! act on IN-LOOP, the specific downstream items an edit broke:
//!
//!   * a typed edge whose endpoint no longer resolves (the edit renamed or removed the target), and
//!   * a verified criterion the edit CHANGED while its passing `TestResult` still stands (drift).
//!
//! Both are advisory, never a block: the fast tier already blocks a malformed model, and D0098 keeps
//! completeness off the commit path. This only tells the author what their last edit touched
//! downstream, at the point of the edit, so the break is fixed now rather than found at commit.

use std::path::Path;

/// The HEAD blob of a repo-relative path, or empty when it is new / unreadable (a new file has no
/// prior criterion to drift, so empty is the correct "nothing to compare" answer).
fn head_blob(root: &Path, rel: &str) -> String {
    crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["show", &format!("HEAD:{rel}")])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default()
}

/// Extract `verification <name> ... { <body> }` blocks as `(name, body)`. Non-nested braces (the
/// engine's verification declarations never nest), scanned from source text so it needs no model build.
fn verification_blocks(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (idx, _) in text.match_indices("verification ") {
        let after = &text[idx + "verification ".len()..];
        let Some(name) = after.split([' ', ':', '{']).next().map(str::trim) else { continue };
        if name.is_empty() {
            continue;
        }
        let Some(open) = after.find('{') else { continue };
        let Some(close_rel) = after[open..].find('}') else { continue };
        let body = &after[open + 1..open + close_rel];
        out.push((name.to_string(), body.to_string()));
    }
    out
}

/// Does `text` carry a PASSING result for verification `name` (`part <name>R<n> : TestResult ... pass`)?
fn has_passing_result(text: &str, name: &str) -> bool {
    let needle = format!("{name}R");
    for (idx, _) in text.match_indices(&needle) {
        let stmt_end = text[idx..].find('}').map_or(text.len(), |e| idx + e);
        let line_start = text[..idx].rfind('\n').map_or(0, |n| n + 1);
        let stmt = &text[line_start..stmt_end];
        if stmt.contains(": TestResult") && stmt.contains("VerdictKind::pass") {
            return true;
        }
    }
    false
}

/// A verified criterion whose body the edit CHANGED while its passing result still stands. Pure core
/// (no git) so it is unit-testable: `old` is the HEAD blob, `new` is the working text.
fn criterion_drift_core(old: &str, new: &str) -> Vec<String> {
    if old.is_empty() {
        return Vec::new(); // new file — no prior criterion to drift from
    }
    let old_blocks: std::collections::HashMap<String, String> = verification_blocks(old).into_iter().collect();
    let mut out = Vec::new();
    for (name, new_body) in verification_blocks(new) {
        let Some(old_body) = old_blocks.get(&name) else { continue }; // newly added — not a drift
        if *old_body != new_body && has_passing_result(new, &name) {
            out.push(format!(
                "criterion drift — `{name}` changed but its verify result still says pass; re-verify or the pass is stale"
            ));
        }
    }
    out
}

/// Non-blocking advisories for a just-edited `.sysml` file. Empty = silent (a clean edit costs nothing).
///
/// # Panics
/// Never — every fallible step degrades to "no advisory" so a hook can never be wedged by this.
#[must_use]
pub fn post_edit_advisories(root: &Path, edited_rel: &str) -> Vec<String> {
    let mut out = Vec::new();
    // (a) Broken edges. The committed tree carries ZERO dangling endpoints (edge-endpoints is a hard
    // commit gate), so any dangling endpoint NOW is attributable to the working edit — "that change
    // broke X". Capped so a huge accidental breakage cannot flood the advisory.
    if let Ok(dangling) = crate::view::dangling_edge_endpoints(root) {
        let total = dangling.len();
        for d in dangling.iter().take(10) {
            out.push(format!("broken edge — {d}"));
        }
        if total > 10 {
            out.push(format!("...and {} more dangling edge(s)", total - 10));
        }
    }
    // (b) Criterion drift on the edited file, diffed against its HEAD blob.
    out.extend(criterion_drift_core(&head_blob(root, edited_rel), &std::fs::read_to_string(root.join(edited_rel)).unwrap_or_default()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verification_blocks_extracts_name_and_body() {
        let t = "verification fooDoD : Test { :>> id = \"x\"; :>> procedureText = \"do A\"; }";
        let b = verification_blocks(t);
        assert_eq!(b.len(), 1);
        assert_eq!(b[0].0, "fooDoD");
        assert!(b[0].1.contains("do A"));
    }

    #[test]
    fn has_passing_result_needs_the_result_line() {
        let pass = "part fooDoDR1 : TestResult { :>> outcome = VerdictKind::pass; }";
        assert!(has_passing_result(pass, "fooDoD"));
        let fail = "part fooDoDR1 : TestResult { :>> outcome = VerdictKind::fail; }";
        assert!(!has_passing_result(fail, "fooDoD"));
        assert!(!has_passing_result("verification fooDoD : Test { }", "fooDoD"));
    }

    #[test]
    fn changed_criterion_with_passing_result_is_drift() {
        let old = "verification fooDoD : Test { :>> procedureText = \"old A\"; }\npart fooDoDR1 : TestResult { :>> outcome = VerdictKind::pass; }";
        let new = "verification fooDoD : Test { :>> procedureText = \"NEW B\"; }\npart fooDoDR1 : TestResult { :>> outcome = VerdictKind::pass; }";
        let d = criterion_drift_core(old, new);
        assert_eq!(d.len(), 1, "{d:?}");
        assert!(d[0].contains("fooDoD"));
    }

    #[test]
    fn unchanged_criterion_is_not_drift() {
        let same = "verification fooDoD : Test { :>> procedureText = \"same\"; }\npart fooDoDR1 : TestResult { :>> outcome = VerdictKind::pass; }";
        assert!(criterion_drift_core(same, same).is_empty());
    }

    #[test]
    fn changed_criterion_without_a_passing_result_is_not_drift() {
        // No result yet -> the criterion is still being authored, not drifting away from evidence.
        let old = "verification fooDoD : Test { :>> procedureText = \"old\"; }";
        let new = "verification fooDoD : Test { :>> procedureText = \"new\"; }";
        assert!(criterion_drift_core(old, new).is_empty());
    }

    #[test]
    fn new_file_has_no_drift() {
        assert!(criterion_drift_core("", "verification x : Test { }").is_empty());
    }
}
