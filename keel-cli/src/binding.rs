//! A passing result binds to the tree that CARRIES the work it judges (dcResultBindsToItsLandingCommit).
//!
//! WHY. The ordinary shape of a `DoD` pass is `append-result --sha HEAD` while the work sits uncommitted,
//! so `judgedAgainst` names the commit BEFORE the one that lands the work: a tree that does not contain
//! what was verified. Every reader that dereferenced `judgedAgainst` - deliverable-drift suspicion,
//! criterion drift, reverify - then measured from the wrong baseline (stpa-self run 1, UCA-A2).
//!
//! WHAT. The same fallback D0308 gave acceptances: when `judgedAgainst` is an ancestor of the commit
//! that INTRODUCED the result, the introducing commit is the binding. A result recorded on a clean tree
//! is unchanged in substance - its introducing commit differs from `judgedAgainst` only by the result
//! line itself. A result not yet committed has no introducing commit and binds where it says.
//!
//! HOW, CHEAPLY. The commit that introduced a result id is an immutable fact (D0129: never rebase), so
//! it is learned once per machine by ONE walk of `git log -p -- .tracking` that reads every added
//! `:>> id = "..."` line, remembered in the content-addressed cache ([`crate::gitfacts`]), and the next
//! walk covers only `walked..HEAD`. Ancestry is answered in-process from one `git rev-list --parents`.

use std::collections::{HashMap, HashSet};
use std::path::Path;

/// What a result is asked to bind: its id, and the commit its author judged against.
pub struct Ask<'a> {
    pub id: &'a str,
    pub judged_against: &'a str,
}

/// For each ask, the binding commit, keyed by result id.
///
/// The introducing commit when `judged_against` is its ancestor (or itself), else `judged_against`
/// unchanged. Ids with no introducing commit yet (uncommitted) are absent from the map - the caller
/// keeps `judged_against`.
#[must_use]
pub fn bind(repo: &Path, asks: &[Ask<'_>]) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let ids: Vec<&str> = asks.iter().map(|a| a.id).filter(|i| !i.is_empty()).collect();
    if ids.is_empty() {
        return out;
    }
    let intro = introducing_commits(repo, &ids);
    if intro.is_empty() {
        return out;
    }
    let parents = parents_graph(repo);
    for a in asks {
        let Some(ic) = intro.get(a.id) else { continue };
        let Some(ja) = crate::gitfacts::full_id(repo, a.judged_against) else { continue };
        if ja == *ic || (!parents.is_empty() && is_ancestor(&parents, &ja, ic)) {
            out.insert(a.id.to_string(), ic.clone());
        }
    }
    out
}

/// `id -> introducing full sha` for every id the cache knows or the (incremental) walk finds.
fn introducing_commits(repo: &Path, ids: &[&str]) -> HashMap<String, String> {
    let mut known: HashMap<String, String> = HashMap::new();
    let mut missing = false;
    for id in ids {
        match crate::gitfacts::introduced(repo, id) {
            Some(sha) => {
                known.insert((*id).to_string(), sha);
            }
            None => missing = true,
        }
    }
    let Some(head) = crate::gitfacts::head_sha(repo) else { return known };
    let walked = crate::gitfacts::introduced_walked_to(repo);
    // Nothing to learn: every id is known, or the walk already reached this HEAD (the misses are
    // uncommitted results, which no walk can place).
    if !missing || walked.as_deref() == Some(head.as_str()) {
        return known;
    }
    let found = walk(repo, walked.as_deref(), &head);
    crate::gitfacts::remember_introduced(repo, &found, &head);
    for (id, sha) in found {
        if ids.contains(&id.as_str()) {
            known.entry(id).or_insert(sha);
        }
    }
    known
}

/// One `git log -p` over `.tracking` (all of history, or `from..head`), returning `(id, commit)` for
/// every `:>> id = "..."` on an ADDED line, OLDEST commit last - callers insert first-wins after
/// reversing, or use [`crate::gitfacts::remember_introduced`], which keeps the oldest.
fn walk(repo: &Path, from: Option<&str>, head: &str) -> Vec<(String, String)> {
    let range = from.map_or_else(|| head.to_string(), |f| format!("{f}..{head}"));
    let out = crate::gitx::git()
        .arg("-C")
        .arg(repo)
        .args(["log", "--format=COMMIT %H", "-p", "--no-color", "--no-renames", "--first-parent"])
        .arg(&range)
        .args(["--", ".tracking"])
        .output();
    let Ok(out) = out else { return Vec::new() };
    if !out.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&out.stdout);
    // git log lists newest first; collect in that order and REVERSE so the oldest commit that added
    // an id wins when the same id was later moved.
    let mut newest_first: Vec<(String, String)> = Vec::new();
    let mut commit = String::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("COMMIT ") {
            commit = rest.trim().to_string();
            continue;
        }
        if !line.starts_with('+') || line.starts_with("+++") || commit.is_empty() {
            continue;
        }
        for id in ids_on(line) {
            newest_first.push((id, commit.clone()));
        }
    }
    newest_first.reverse();
    let mut seen = HashSet::new();
    newest_first.retain(|(id, _)| seen.insert(id.clone()));
    newest_first
}

/// Every `:>> id = "<uuid>"` value on one line.
fn ids_on(line: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = line;
    while let Some(pos) = rest.find("id = \"") {
        let after = &rest[pos + 6..];
        if let Some(end) = after.find('"') {
            let v = &after[..end];
            if v.len() == 36 && v.bytes().all(|b| b.is_ascii_hexdigit() || b == b'-') {
                found.push(v.to_string());
            }
            rest = &after[end..];
        } else {
            break;
        }
    }
    found
}

/// `commit -> parents` for every commit reachable from HEAD, from one `git rev-list --parents`.
/// Empty when git cannot answer, in which case no re-binding happens (fail closed to `judgedAgainst`).
fn parents_graph(repo: &Path) -> HashMap<String, Vec<String>> {
    let out = crate::gitx::git()
        .arg("-C")
        .arg(repo)
        .args(["rev-list", "--parents", "HEAD"])
        .output();
    let Ok(out) = out else { return HashMap::new() };
    if !out.status.success() {
        return HashMap::new();
    }
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|l| {
            let mut it = l.split_whitespace();
            let c = it.next()?;
            Some((c.to_string(), it.map(str::to_string).collect()))
        })
        .collect()
}

/// Is `anc` reachable from `from` by walking parents (or equal)?
fn is_ancestor(parents: &HashMap<String, Vec<String>>, anc: &str, from: &str) -> bool {
    let mut stack = vec![from.to_string()];
    let mut seen = HashSet::new();
    while let Some(c) = stack.pop() {
        if c == anc {
            return true;
        }
        if !seen.insert(c.clone()) {
            continue;
        }
        if let Some(ps) = parents.get(&c) {
            stack.extend(ps.iter().cloned());
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_on_reads_every_uuid_on_an_added_line() {
        let line = "+        part xDoDR1 : TestResult { :>> id = \"9393c050-ba83-482d-ac3d-51ce0630c4b6\"; :>> outcome = VerdictKind::pass; }";
        assert_eq!(ids_on(line), vec!["9393c050-ba83-482d-ac3d-51ce0630c4b6".to_string()]);
        assert!(ids_on("+  :>> id = \"short\";").is_empty());
        assert!(ids_on("+  no id here").is_empty());
    }

    #[test]
    fn ancestry_walks_parents_and_treats_self_as_ancestor() {
        let mut g = HashMap::new();
        g.insert("c".to_string(), vec!["b".to_string()]);
        g.insert("b".to_string(), vec!["a".to_string()]);
        g.insert("a".to_string(), vec![]);
        assert!(is_ancestor(&g, "a", "c"));
        assert!(is_ancestor(&g, "c", "c"));
        assert!(!is_ancestor(&g, "c", "a"));
        assert!(!is_ancestor(&g, "zz", "c"));
    }

    /// The fixture the `DoD` names: a pass recorded at HEAD, then the work committed - the pass reads as
    /// bound to the landing commit; a pass whose id is not yet in history keeps `judgedAgainst`.
    #[test]
    fn a_pass_recorded_before_its_landing_commit_binds_to_the_landing_commit() {
        let dir = std::env::temp_dir().join(format!("keel-binding-{}", crate::write::gen_uuid()));
        std::fs::create_dir_all(dir.join(".tracking")).unwrap();
        let git = |args: &[&str]| {
            let o = crate::gitx::git().arg("-C").arg(&dir).args(args).output().unwrap();
            assert!(o.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&o.stderr));
            String::from_utf8_lossy(&o.stdout).trim().to_string()
        };
        git(&["init", "-q"]);
        git(&["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q", "--allow-empty", "-m", "base"]);
        let base = git(&["rev-parse", "HEAD"]);
        // Work + a pass judged against `base` (HEAD at the time), committed together: the landing commit.
        let id = "11111111-2222-4333-8444-555555555555";
        std::fs::write(
            dir.join(".tracking").join("t.sysml"),
            format!("package T {{\n    part tDoDR1 : TestResult {{ :>> id = \"{id}\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"{base}\"; }}\n}}\n"),
        )
        .unwrap();
        git(&["add", "-A"]);
        git(&["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q", "-m", "land"]);
        let land = git(&["rev-parse", "HEAD"]);
        let bound = bind(&dir, &[Ask { id, judged_against: &base }]);
        assert_eq!(bound.get(id), Some(&land), "the pass binds to the commit that carries the work");
        // An uncommitted result has no introducing commit: absent, so the caller keeps judgedAgainst.
        let bound = bind(&dir, &[Ask { id: "99999999-2222-4333-8444-555555555555", judged_against: &base }]);
        assert!(bound.is_empty());
        // A judgedAgainst that is NOT an ancestor of the introducing commit is left alone.
        let bound = bind(&dir, &[Ask { id, judged_against: "0000000000000000000000000000000000000000" }]);
        assert!(bound.is_empty());
        crate::gitfacts::flush(&dir);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
