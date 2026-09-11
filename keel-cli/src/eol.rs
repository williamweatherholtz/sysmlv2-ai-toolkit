//! THE WORKING TREE'S LINE ENDINGS AGAINST THE REPOSITORY'S OWN ATTRIBUTE (issue478).
//!
//! `.gitattributes` says `*.sysml text eol=lf` (and the same for every source, config and doc type),
//! so git NORMALISES those files at the commit: a CRLF working copy lands as LF and `git status` reads
//! it as clean. Nothing that tests the working tree sees the normalised bytes - `keel suite --touched`
//! and `keel land` compile and run the files AS THEY ARE ON DISK - so on 2026-09-11 (sprint 668) five
//! tests that read an exact newline-brace-newline anchor found 0 where they expected 2, and the
//! verifier's receipt reported five failures nothing in the change could reach. Every file the sprint's
//! ad-hoc `pathlib.write_text` scripts and the harness's Write tool had produced was CRLF, and
//! `git ls-files --eol` read each as `i/lf w/crlf attr/text eol=lf`. A census after the fix found 140 more unchanged
//! CRLF paths in the same clone (core.autocrlf true) that no test happened to read with an anchor.
//!
//! THE SOURCE IS `git ls-files --eol -z`, one call per run: for every tracked path, the index blob's
//! ending (`i/`), the working copy's (`w/`) and the attribute in force (`attr/`). A path is a MISMATCH
//! when its attribute declares an ending (`eol=lf` or `eol=crlf`) and the working copy holds another
//! (`crlf`, `lf` or `mixed`). A path whose attribute declares NO ending (`text=auto`, bare `text`,
//! `-text`, nothing) is outside this check on purpose: its working-copy ending is whatever
//! `core.autocrlf` chose on this machine, which is exactly what the repository did not pin. A binary
//! (`w/-text`) or absent (`w/none`) working copy has no ending to judge.
//!
//! Three callers read the same census: the forward guard `working-tree-eol` (every path is a violation),
//! `keel suite --touched` (the receipt writes `outcome = "eol-mismatch"` and cargo never starts), and
//! `keel land` (refuses the push with the same line). The touched run and land name the CHANGED paths
//! first - the ones this push wrote - then count the rest, because the changed ones are the ones a
//! writer in this session produced and the rest were there before it.

use std::fmt::Write as _;
use std::path::Path;

/// One line of `git ls-files --eol`: the index ending, the working-copy ending, the attribute text
/// (`text eol=lf`, `text=auto`, `-text`, or empty) and the repo-relative path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub index: String,
    pub worktree: String,
    pub attr: String,
    pub path: String,
}

/// A tracked path whose working copy holds an ending its attribute does not declare.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch {
    pub path: String,
    /// `lf` or `crlf` - what `.gitattributes` declares.
    pub declared: String,
    /// `lf`, `crlf` or `mixed` - what the bytes on disk hold.
    pub worktree: String,
}

/// The census of one tree: how many tracked paths declare an ending, and which of them break it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Census {
    /// Tracked paths whose attribute declares an ending and whose working copy has one to judge.
    pub scanned: usize,
    pub mismatches: Vec<Mismatch>,
    /// The one git call's wall clock, so the cost is a recorded number and not a claim.
    pub millis: u64,
}

/// Parse `git ls-files --eol -z` output (pure). Each record is `i/<x> w/<y> attr/<text>\t<path>`,
/// NUL-terminated; the attribute text may hold spaces (`text eol=lf`) and may be empty (`attr/`).
#[must_use]
pub fn parse(out: &[u8]) -> Vec<Entry> {
    let mut entries = Vec::new();
    for rec in out.split(|b| *b == 0).filter(|r| !r.is_empty()) {
        let rec = String::from_utf8_lossy(rec);
        let Some((left, path)) = rec.split_once('\t') else { continue };
        let mut words = left.split_whitespace();
        let Some(index) = words.next().and_then(|w| w.strip_prefix("i/")) else { continue };
        let Some(worktree) = words.next().and_then(|w| w.strip_prefix("w/")) else { continue };
        let attr = words.collect::<Vec<_>>().join(" ");
        let attr = attr.strip_prefix("attr/").unwrap_or(&attr).to_string();
        entries.push(Entry { index: index.to_string(), worktree: worktree.to_string(), attr, path: path.to_string() });
    }
    entries
}

/// The ending an attribute text DECLARES: `Some("lf")` / `Some("crlf")` for an `eol=` clause, `None` for
/// `text=auto`, bare `text`, `-text` or nothing - the endings this repository left to each machine.
#[must_use]
pub fn declared(attr: &str) -> Option<&str> {
    attr.split_whitespace().find_map(|w| w.strip_prefix("eol=")).filter(|e| matches!(*e, "lf" | "crlf"))
}

/// The mismatches in a parsed census, and how many paths were judged (pure).
#[must_use]
pub fn judge(entries: &[Entry]) -> (usize, Vec<Mismatch>) {
    let mut scanned = 0usize;
    let mut out = Vec::new();
    for e in entries {
        let Some(want) = declared(&e.attr) else { continue };
        // No bytes to judge: a binary working copy, or none on disk (deleted, or a submodule gitlink).
        if !matches!(e.worktree.as_str(), "lf" | "crlf" | "mixed") {
            continue;
        }
        scanned += 1;
        if e.worktree != want {
            out.push(Mismatch { path: e.path.clone(), declared: want.to_string(), worktree: e.worktree.clone() });
        }
    }
    (scanned, out)
}

/// Read the census for `repo` - ONE `git ls-files --eol -z` over every tracked path under it.
///
/// # Errors
/// When git cannot list the tree (not a repository, or git absent). The caller decides what that means:
/// the guard reports nothing to judge; the touched run and land, which already needed git to compute
/// their set, refuse.
pub fn census(repo: &Path) -> Result<Census, String> {
    crate::perf::phase("eol:census", || {
        let t0 = std::time::Instant::now();
        let out = crate::gitx::git().arg("-C").arg(repo).args(["ls-files", "--eol", "-z"]).output().map_err(|e| format!("git ls-files --eol: {e}"))?;
        if !out.status.success() {
            return Err(format!("git ls-files --eol failed: {}", String::from_utf8_lossy(&out.stderr).trim()));
        }
        let (scanned, mismatches) = judge(&parse(&out.stdout));
        Ok(Census { scanned, mismatches, millis: u64::try_from(t0.elapsed().as_millis()).unwrap_or(u64::MAX) })
    })
}

/// The one line the touched run and `land` print and the guard's violations restate: the mismatched
/// paths that this push CHANGED first (by name), then the count of the rest, then the remedy. Pure.
#[must_use]
pub fn describe(mismatches: &[Mismatch], changed: &[String]) -> String {
    let is_changed = |p: &str| changed.iter().any(|c| c.replace('\\', "/") == p);
    let named: Vec<String> = mismatches.iter().filter(|m| is_changed(&m.path)).map(|m| format!("{} (w/{} where eol={})", m.path, m.worktree, m.declared)).collect();
    let rest = mismatches.len() - named.len();
    let mut s = format!("working-tree eol MISMATCH: {} tracked path{} hold{} a line ending .gitattributes does not declare (issue478)", mismatches.len(), if mismatches.len() == 1 { "" } else { "s" }, if mismatches.len() == 1 { "s" } else { "" });
    if !named.is_empty() {
        let _ = write!(s, " - changed by this push: [{}]", named.join(", "));
    }
    if rest > 0 {
        let unchanged: Vec<&str> = mismatches.iter().filter(|m| !is_changed(&m.path)).map(|m| m.path.as_str()).take(5).collect();
        let _ = write!(
            s,
            "{} {rest} unchanged path{} (first: {}{})",
            if named.is_empty() { " -" } else { "; and" },
            if rest == 1 { "" } else { "s" },
            unchanged.join(", "),
            if rest > 5 { ", ..." } else { "" }
        );
    }
    let _ = write!(s, ". {REMEDY}");
    s
}

/// What to do about one: written once, printed by every caller.
pub const REMEDY: &str = "Rewrite the bytes with the declared ending - a Python `open(p, 'w')` and the harness Write tool translate newlines on Windows; `newline=''` or a byte write does not - then `git ls-files --eol -- <path>` reads w/<declared>. `keel guard working-tree-eol` lists every path";

#[cfg(test)]
mod tests {
    use super::{declared, describe, judge, parse, Mismatch};

    const SAMPLE: &[u8] = b"i/lf    w/crlf  attr/text eol=lf      \tkeel-cli/src/a.rs\0i/lf    w/lf    attr/text eol=lf      \tCLAUDE.md\0i/lf    w/crlf  attr/text=auto        \tLICENSE\0i/-text w/-text attr/text=auto        \timg.png\0i/none  w/none  attr/                  \tsub\0i/crlf  w/crlf  attr/text eol=crlf    \twin.bat\0i/lf    w/mixed attr/text eol=lf      \tmixed.toml\0i/lf    w/none  attr/text eol=lf      \tgone.md\0";

    /// D0388 known-positive, named before any tree is read: a `text eol=lf` path whose working copy is
    /// CRLF is a mismatch, and so is a mixed one; the census names them and counts every judged path.
    #[test]
    fn a_crlf_working_copy_under_eol_lf_is_a_mismatch() {
        let entries = parse(SAMPLE);
        assert_eq!(entries.len(), 8, "{entries:?}");
        assert_eq!(entries[0].attr, "text eol=lf");
        assert_eq!(entries[4].attr, "", "an empty attribute parses as empty, not as a missing record");
        let (scanned, m) = judge(&entries);
        assert_eq!(scanned, 4, "a.rs, CLAUDE.md, win.bat and mixed.toml declare an ending and have bytes to judge");
        assert_eq!(
            m,
            vec![
                Mismatch { path: "keel-cli/src/a.rs".into(), declared: "lf".into(), worktree: "crlf".into() },
                Mismatch { path: "mixed.toml".into(), declared: "lf".into(), worktree: "mixed".into() },
            ]
        );
    }

    /// D0388 known-negative: an LF copy under `eol=lf` passes; a CRLF copy under `eol=crlf` passes; a
    /// CRLF copy whose attribute declares NO ending (`text=auto`, this clone's LICENSE under
    /// core.autocrlf) is not named; a binary or absent working copy is not judged at all.
    #[test]
    fn a_conforming_copy_an_undeclared_ending_and_a_binary_are_not_named() {
        let (_, m) = judge(&parse(SAMPLE));
        for quiet in ["CLAUDE.md", "LICENSE", "img.png", "sub", "win.bat", "gone.md"] {
            assert!(!m.iter().any(|x| x.path == quiet), "{quiet} must not be named: {m:?}");
        }
        assert_eq!(declared("text eol=lf"), Some("lf"));
        assert_eq!(declared("text eol=crlf"), Some("crlf"));
        assert_eq!(declared("text=auto"), None);
        assert_eq!(declared("text"), None);
        assert_eq!(declared("-text"), None);
        assert_eq!(declared(""), None);
        assert_eq!(declared("eol=native"), None, "an ending git leaves to the platform is not a declaration");
    }

    /// The line names the changed paths first and counts the rest (the definition of done's shape).
    #[test]
    fn the_line_names_the_changed_paths_and_counts_the_rest() {
        let m = vec![
            Mismatch { path: "a.rs".into(), declared: "lf".into(), worktree: "crlf".into() },
            Mismatch { path: "b.sysml".into(), declared: "lf".into(), worktree: "crlf".into() },
            Mismatch { path: "c.md".into(), declared: "lf".into(), worktree: "mixed".into() },
        ];
        let s = describe(&m, &["b.sysml".to_string(), "other.rs".to_string()]);
        assert!(s.starts_with("working-tree eol MISMATCH: 3 tracked paths hold"), "{s}");
        assert!(s.contains("changed by this push: [b.sysml (w/crlf where eol=lf)]"), "{s}");
        assert!(s.contains("and 2 unchanged paths (first: a.rs, c.md)"), "{s}");
        let none_changed = describe(&m[..1], &[]);
        assert!(none_changed.contains("1 tracked path holds") && none_changed.contains("- 1 unchanged path (first: a.rs)"), "{none_changed}");
        assert!(!none_changed.contains("changed by this push"), "{none_changed}");
    }
}
