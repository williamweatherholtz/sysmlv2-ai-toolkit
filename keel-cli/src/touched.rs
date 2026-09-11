//! THE TOUCHED TEST SET (D0421, issue416): the integration tests that NAME a module changed since the
//! base of the push, run before the first push - and nothing else.
//!
//! WHY THIS EXISTS. D0356 withdrew the full-suite push gate on the owner's measurement: ~11 wall
//! minutes every time code moved, against roughly one catchable bad push in twenty-five. The next
//! CI red after that (issue416) was a test whose text named the very module the commit had changed -
//! the one file a text match would have picked out in seconds. This is NOT the D0356 gate returning:
//! the whole suite is still `keel suite`, still measured, still gating nothing. This runs the subset a
//! commit can be expected to have touched, and says when that subset is empty.
//!
//! HOW THE SET IS COMPUTED - by text, deliberately. A changed path under `keel-cli/src/` contributes
//! its MODULE STEM (`sync.rs` -> `sync`, `view/mod.rs` -> `view`); `main.rs` and `lib.rs` name no
//! module and are reported as unattributed rather than matched against every test that says `main`.
//! An integration test is touched when its text carries a stem as a whole word (`keel_cli::sync::`,
//! `keel sync`, `sync.rs` all count; `synced` does not), or when the test file itself changed.
//! Over-inclusion costs a test run; under-inclusion is the CI red this replaces, so the match is
//! generous and pure (`names_stem`, `touched_tests` - unit-tested below).
//!
//! THE BASE is `origin/<branch>` when it resolves (what the push will land on), else the head the
//! last suite receipt recorded, else `HEAD~1`; a tree with none of those reads every tracked module
//! as changed and says so. THE CHANGED SET is measured from the merge-base to the WORKING TREE -
//! tracked edits and untracked files alike - not to HEAD: the D0425 verifier runs this BEFORE the
//! commit, and on 2026-09-10 (sprint 659, HEAD equal to origin/main, twenty files edited under
//! `keel-cli/`) `base...HEAD` read an empty set and the verifier ran nothing (issue463). Inside the
//! post-commit land the working tree IS HEAD, so nothing changes there.
//!
//! WHERE IT IS INERT (D0337). A refusal on `land` is a change to the integration path, which is outside
//! standing consent - so `land` computes and PRINTS the set on every push, but RUNS it and refuses only
//! once D0421 carries the human's acceptance (`d0421AcceptR1` in its decision file, the D0338 pattern).
//! `keel suite --touched` runs the set on demand regardless: an explicit command is the caller's word.
//!
//! THE RECEIPT is machine-local, beside the suite's (`.keel/metrics/touched-receipt.toml`): the base,
//! the stems, the tests, what they cost, and the outcome - an empty set is a receipt too.

use std::path::{Path, PathBuf};

pub const RECEIPT: &str = ".keel/metrics/touched-receipt.toml";

/// The module stem a changed path contributes, or `None` for a path that names no module.
///
/// Pure: `keel-cli/src/sync.rs` -> `sync`; `keel-cli/src/view/mod.rs` -> `view`;
/// `keel-cli/src/main.rs`, `keel-cli/src/lib.rs`, anything outside `keel-cli/src/` -> `None`.
#[must_use]
pub fn module_stem(path: &str) -> Option<String> {
    let p = path.replace('\\', "/");
    let rel = p.strip_prefix("keel-cli/src/")?;
    let file = rel.strip_suffix(".rs")?;
    let mut parts: Vec<&str> = file.split('/').collect();
    let last = parts.pop()?;
    let stem = if last == "mod" { *parts.last()? } else { last };
    if stem == "main" || stem == "lib" || stem.is_empty() {
        return None;
    }
    Some(stem.to_string())
}

/// Is this a changed path that names no module but is still deliverable code (`main.rs`, `lib.rs`)?
#[must_use]
pub fn is_unattributed_source(path: &str) -> bool {
    let p = path.replace('\\', "/");
    p.starts_with("keel-cli/src/") && std::path::Path::new(&p).extension().is_some_and(|x| x.eq_ignore_ascii_case("rs")) && module_stem(&p).is_none()
}

/// The test name an integration-test path contributes (`keel-cli/tests/land_gate.rs` -> `land_gate`).
#[must_use]
pub fn test_name(path: &str) -> Option<String> {
    let p = path.replace('\\', "/");
    let rel = p.strip_prefix("keel-cli/tests/")?;
    if rel.contains('/') {
        return None; // a helper under tests/<dir>/ is not a test binary
    }
    rel.strip_suffix(".rs").map(str::to_string)
}

/// Does `text` carry `stem` as a whole word? A word character is `[A-Za-z0-9_]`, so `keel_cli::sync::`
/// and `keel sync` and `sync.rs` all name `sync`, and `synced` / `resync` do not.
#[must_use]
pub fn names_stem(text: &str, stem: &str) -> bool {
    if stem.is_empty() {
        return false;
    }
    let bytes = text.as_bytes();
    let is_word = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut from = 0;
    while let Some(i) = text[from..].find(stem) {
        let start = from + i;
        let end = start + stem.len();
        let before_ok = start == 0 || bytes.get(start - 1).is_none_or(|b| !is_word(*b));
        let after_ok = bytes.get(end).is_none_or(|b| !is_word(*b));
        if before_ok && after_ok {
            return true;
        }
        from = start + 1;
    }
    false
}

/// The touched set, pure: every test whose text names a changed stem, plus every test that changed
/// itself. Sorted, deduplicated. `tests` is `(name, text)`.
#[must_use]
pub fn touched_tests(tests: &[(String, String)], stems: &[String], changed_tests: &[String]) -> Vec<String> {
    let mut out: Vec<String> = tests
        .iter()
        .filter(|(name, text)| changed_tests.contains(name) || stems.iter().any(|s| names_stem(text, s)))
        .map(|(name, _)| name.clone())
        .collect();
    out.sort();
    out.dedup();
    out
}

/// The test binaries cargo reported as failed (pure, over cargo's captured output).
///
/// Read from cargo's OWN attribution on stderr - `error: test failed, to rerun pass \`--test <name>\``
/// and the `--no-fail-fast` summary `error: N targets failed:` followed by one \`--test <name>\` per
/// line. The first cut paired each `test result: FAILED` line with the `Running tests/<name>.rs`
/// header before it; that pairing holds on a terminal, where the two streams interleave, and NOT in
/// a capture, where stdout (the results) is read whole before stderr (the headers) - the first live
/// run recorded `failed = 1` and `failing = []`. Cargo's rerun hint is the one line that carries the
/// name and the verdict together.
#[must_use]
pub fn failing_binaries(cargo_output: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in cargo_output.lines() {
        // The lib's hint is `--lib`, with no name: it is reported as `lib`.
        if line.contains("`--lib`") && !out.iter().any(|n| n == "lib") {
            out.push("lib".to_string());
        }
        let mut rest = line;
        while let Some(i) = rest.find("`--test ") {
            let after = &rest[i + "`--test ".len()..];
            if let Some(end) = after.find('`') {
                let name = after[..end].trim();
                if !name.is_empty() && !out.iter().any(|n| n == name) {
                    out.push(name.to_string());
                }
                rest = &after[end + 1..];
            } else {
                break;
            }
        }
    }
    out.sort();
    out
}

/// What was computed for one push: the base compared against, the stems, and the set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Touched {
    pub base: String,
    pub stems: Vec<String>,
    pub unattributed: Vec<String>,
    pub tests: Vec<String>,
    /// The lib's own unit tests (`cargo test --lib`) are in the set whenever ANY `keel-cli/src` path
    /// changed: a module's `#[cfg(test)]` block names it by construction, and a unit test elsewhere
    /// can read the changed module's live facts - `cli_surface_declared_tests` read the suite
    /// synopsis and hardcoded which Decisions it may cite, and CI went red on cab7cac when the
    /// synopsis gained a citation the set never ran (issue438). ~15 s on this host.
    pub lib: bool,
    /// Every path the working tree changed since the base (tracked edits and untracked crate files),
    /// repo-relative - what the eol refusal names first (issue478).
    pub changed: Vec<String>,
    /// The working tree's line endings against the attribute (issue478): every tracked path whose
    /// `.gitattributes` entry declares an ending and whose working copy holds another. Read ONCE, with
    /// the set, so `land` and `suite --touched` refuse on the same census before cargo compiles the
    /// bytes; `eol_scanned` / `eol_millis` are the census's population and cost.
    pub eol: Vec<crate::eol::Mismatch>,
    pub eol_scanned: usize,
    pub eol_millis: u64,
}

impl Touched {
    /// Nothing to run: no integration test names a changed module AND no source path changed.
    #[must_use]
    pub const fn nothing_to_run(&self) -> bool {
        self.tests.is_empty() && !self.lib
    }

    /// The eol refusal line, or `None` when every declared path holds its ending: the changed paths by
    /// name first, then the count of the rest (`eol::describe`).
    #[must_use]
    pub fn eol_refusal(&self) -> Option<String> {
        if self.eol.is_empty() {
            return None;
        }
        Some(crate::eol::describe(&self.eol, &self.changed))
    }

    /// The one line that says the census ran and what it cost.
    #[must_use]
    pub fn eol_line(&self) -> String {
        format!("working-tree eol: {} declared paths hold their ending ({} ms, git ls-files --eol)", self.eol_scanned, self.eol_millis)
    }
}

fn git_out(repo: &Path, args: &[&str]) -> Option<String> {
    let o = crate::gitx::git().arg("-C").arg(repo).args(args).output().ok()?;
    o.status.success().then(|| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

/// The base the changed set is measured from: `origin/<branch>`, else the last suite receipt's head,
/// else `HEAD~1`. `None` when nothing resolves (a one-commit repository with no remote).
fn base_ref(repo: &Path) -> Option<String> {
    let branch = git_out(repo, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_else(|| "HEAD".to_string());
    let remote = format!("origin/{branch}");
    if git_out(repo, &["rev-parse", "--verify", "--quiet", &format!("{remote}^{{commit}}")]).is_some() {
        return Some(remote);
    }
    if let Some(r) = crate::suite::receipt(repo) {
        if !r.head.is_empty() && git_out(repo, &["rev-parse", "--verify", "--quiet", &format!("{}^{{commit}}", r.head)]).is_some() {
            return Some(r.head);
        }
    }
    git_out(repo, &["rev-parse", "--verify", "--quiet", "HEAD~1^{commit}"]).map(|_| "HEAD~1".to_string())
}

/// Compute the touched set for `repo`: what the working tree changed since the base. Self-build only
/// (the caller checks).
///
/// # Errors
/// When git cannot list the changed paths (`diff --name-only` against the base, or `ls-files` when
/// no base resolves).
pub fn compute(repo: &Path) -> Result<Touched, String> {
    let (base, changed): (String, Vec<String>) = if let Some(b) = base_ref(repo) {
        // The merge-base, so a remote that moved ahead does not read as our change; then the diff
        // from it to the WORKING TREE (no second revision), plus the untracked files under the crate -
        // a new module or test file is a change the diff of tracked paths cannot see.
        let from = git_out(repo, &["merge-base", &b, "HEAD"]).unwrap_or_else(|| b.clone());
        let out = git_out(repo, &["diff", "--name-only", &from]).ok_or_else(|| format!("git diff --name-only {from} failed"))?;
        let untracked = git_out(repo, &["ls-files", "--others", "--exclude-standard", "--", "keel-cli"]).unwrap_or_default();
        let mut paths: Vec<String> = out.lines().chain(untracked.lines()).filter(|l| !l.is_empty()).map(str::to_string).collect();
        paths.sort();
        paths.dedup();
        (b, paths)
    } else {
        let out = git_out(repo, &["ls-files", "keel-cli/src", "keel-cli/tests"]).ok_or_else(|| "git ls-files failed".to_string())?;
        ("(no base: every tracked module)".to_string(), out.lines().map(str::to_string).collect())
    };
    let mut stems: Vec<String> = changed.iter().filter_map(|p| module_stem(p)).collect();
    stems.sort();
    stems.dedup();
    let unattributed: Vec<String> = changed.iter().filter(|p| is_unattributed_source(p)).cloned().collect();
    let changed_tests: Vec<String> = changed.iter().filter_map(|p| test_name(p)).collect();
    let tests_dir = repo.join("keel-cli").join("tests");
    let mut tests: Vec<(String, String)> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&tests_dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("rs")) && p.is_file() {
                if let Some(name) = p.file_stem().map(|s| s.to_string_lossy().to_string()) {
                    let text = std::fs::read_to_string(&p).unwrap_or_default();
                    tests.push((name, text));
                }
            }
        }
    }
    let tests = touched_tests(&tests, &stems, &changed_tests);
    let lib = !stems.is_empty() || !unattributed.is_empty();
    // issue478: the endings are read with the set, before any decision to run - the receipt this
    // computation writes must be able to say `eol-mismatch` in place of a verdict cargo never reached.
    let census = crate::eol::census(repo)?;
    Ok(Touched { base, stems, unattributed, tests, lib, changed, eol: census.mismatches, eol_scanned: census.scanned, eol_millis: census.millis })
}

/// Is the land refusal ARMED - has the human accepted D0421? The D0338 pattern: the decision file
/// carries its first acceptance result once `keel accept` has run.
#[must_use]
pub fn gate_accepted(repo: &Path) -> bool {
    let Ok(rd) = std::fs::read_dir(repo.join(".engine").join("decisions")) else { return false };
    rd.flatten().any(|e| {
        let name = e.file_name().to_string_lossy().to_string();
        name.starts_with("0421-") && std::fs::read_to_string(e.path()).is_ok_and(|t| text_carries_acceptance(&t, "d0421"))
    })
}

/// Does a Decision file's text carry a PASSING first acceptance - the `part <dec>AcceptR1 : TestResult`
/// declaration itself, with `outcome = VerdictKind::pass` in its body (pure).
///
/// Not a substring search for the token: D0421's own decision text names `d0421AcceptR1` as the thing
/// that arms it, and the first cut matched that prose at record time - the gate armed itself on the
/// sentence describing how it would be armed, and the post-commit hook's `land` then refused a push
/// while the Decision was PROPOSED (issue435). Only the declaration is the acceptance.
#[must_use]
pub fn text_carries_acceptance(text: &str, dec: &str) -> bool {
    let needle = format!("part {dec}AcceptR1 ");
    let Some(pos) = text.find(&needle) else { return false };
    let after = &text[pos..];
    // `keel accept` writes the part on one line with no nested braces; its own `}` ends the body.
    let body_end = after.find('}').unwrap_or(after.len());
    let body = &after[..body_end];
    body.contains(": TestResult") && body.contains("outcome = VerdictKind::pass")
}

/// One run of the set.
#[derive(Debug, Clone)]
pub struct Run {
    pub passed: u64,
    pub failed: u64,
    pub failing: Vec<String>,
    pub seconds: u64,
    pub cargo_ok: bool,
    pub log: PathBuf,
}

fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// What the receipt records about the run.
///
/// Nothing to record (`NotRun` - an empty or not-run set), a run in progress (`Running` - the
/// stub written BEFORE cargo starts, D0387's sibling for this receipt), or a finished run.
#[derive(Debug, Clone, Copy)]
pub enum Phase<'a> {
    NotRun,
    Running { started: u64, log: &'a Path },
    Done(&'a Run),
    /// The working tree's line endings disagree with the attribute (issue478): cargo never started,
    /// and the receipt names the paths in place of a verdict.
    EolMismatch,
}

fn render_receipt(t: &Touched, head: &str, at: u64, phase: Phase<'_>) -> String {
    use std::fmt::Write as _;
    let list = |v: &[String]| v.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", ");
    let mut s = format!(
        "# touched receipt (D0421): the integration tests that NAME a module changed since the base, and what\n# running exactly those cost. An empty set is a receipt too. Beside the suite's receipt, never in it.\n# `eol_scanned` / `eol_ms`: the working tree's line endings against .gitattributes, read with the set\n# (issue478); `outcome = \"eol-mismatch\"` names the paths that broke it and means cargo never started.\nhead = \"{}\"\nat = {}\nbase = \"{}\"\nstems = [{}]\nunattributed = [{}]\ntests = [{}]\nlib = {}\neol_scanned = {}\neol_ms = {}\n",
        head,
        at,
        t.base,
        list(&t.stems),
        list(&t.unattributed),
        list(&t.tests),
        t.lib,
        t.eol_scanned,
        t.eol_millis
    );
    match phase {
        Phase::EolMismatch => {
            let paths: Vec<String> = t.eol.iter().map(|m| m.path.clone()).collect();
            let _ = write!(s, "outcome = \"eol-mismatch\"\npassed = 0\nfailed = 0\nfailing = []\neol_mismatch = [{}]\nseconds = 0\n", list(&paths));
        }
        // The previous receipt is REPLACED before cargo starts (issue468, the D0387/issue399 class on
        // this receipt): a reader during the run - or after a killed one - sees `running` with THIS
        // run's set and log, never the last run's pass over a different change set. The verifier of
        // sprint 661 read the prior run's 546/0 as its own while its own run was failing two lib tests.
        Phase::Running { started, log } => {
            let _ = write!(
                s,
                "outcome = \"running\"\npassed = 0\nfailed = 0\nfailing = []\nseconds = {}\nlog = \"{}\"\n",
                at.saturating_sub(started),
                log.to_string_lossy().replace('\\', "/")
            );
        }
        Phase::Done(r) => {
            let outcome = if r.cargo_ok && r.failed == 0 { "pass" } else { "fail" };
            let _ = write!(
                s,
                "outcome = \"{}\"\npassed = {}\nfailed = {}\nfailing = [{}]\nseconds = {}\nlog = \"{}\"\n",
                outcome,
                r.passed,
                r.failed,
                list(&r.failing),
                r.seconds,
                r.log.to_string_lossy().replace('\\', "/")
            );
        }
        // `empty`: nothing to run. `not-run`: a set exists and was not run (the refusal is inert, D0337).
        Phase::NotRun => {
            let _ = write!(s, "outcome = \"{}\"\npassed = 0\nfailed = 0\nseconds = 0\n", if t.nothing_to_run() { "empty" } else { "not-run" });
        }
    }
    s
}

fn write_receipt(repo: &Path, t: &Touched, phase: Phase<'_>) {
    let metrics = repo.join(".keel").join("metrics");
    let _ = std::fs::create_dir_all(&metrics);
    let head = git_out(repo, &["rev-parse", "--short", "HEAD"]).unwrap_or_default();
    if let Err(e) = crate::write::write_atomic(&repo.join(RECEIPT), render_receipt(t, &head, now_secs(), phase)) {
        eprintln!("touched: receipt could not be written: {e}");
    }
}

/// Run exactly `t.tests` as one cargo invocation (`--release`, so the binaries CI links are the ones
/// exercised; `--no-fail-fast`, so every named test reports). Writes the log and the receipt.
///
/// # Errors
/// When the metrics directory cannot be created or cargo cannot be started at all.
pub fn run(repo: &Path, t: &Touched) -> Result<Run, String> {
    if t.nothing_to_run() {
        write_receipt(repo, t, Phase::NotRun);
        return Ok(Run { passed: 0, failed: 0, failing: vec![], seconds: 0, cargo_ok: true, log: PathBuf::new() });
    }
    let metrics = repo.join(".keel").join("metrics");
    std::fs::create_dir_all(&metrics).map_err(|e| format!("cannot create {}: {e}", metrics.display()))?;
    let started = now_secs();
    let log = metrics.join(format!("touched-{started}.log"));
    write_receipt(repo, t, Phase::Running { started, log: &log });
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("test").arg("--release").arg("--manifest-path").arg(repo.join("keel-cli").join("Cargo.toml")).arg("--no-fail-fast");
    for name in &t.tests {
        cmd.arg("--test").arg(name);
    }
    if t.lib {
        cmd.arg("--lib");
    }
    let out = cmd.current_dir(repo).output().map_err(|e| format!("cargo could not be run: {e}"))?;
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    let _ = std::fs::write(&log, &text);
    let (passed, failed) = crate::suite::count_results(&text);
    // A build failure is not a verdict about the tests - but it IS a reason not to push: the
    // binaries CI will link do not link here either. Every named test is reported as not run.
    let failing = if crate::suite::never_ran(out.status.success(), passed, failed) { t.tests.clone() } else { failing_binaries(&text) };
    let r = Run { passed, failed, failing, seconds: now_secs().saturating_sub(started), cargo_ok: out.status.success(), log };
    write_receipt(repo, t, Phase::Done(&r));
    Ok(r)
}

/// One line per fact, for `land` and `suite --touched` alike.
fn describe(t: &Touched) -> String {
    use std::fmt::Write as _;
    let mut s = format!("touched tests: base {}; changed modules [{}]", t.base, t.stems.join(", "));
    if !t.unattributed.is_empty() {
        let _ = write!(s, "; unattributed (name no module): [{}]", t.unattributed.join(", "));
    }
    if t.tests.is_empty() {
        s.push_str("; set EMPTY - no integration test names a changed module");
    } else {
        let _ = write!(s, "; set [{}]", t.tests.join(", "));
    }
    if t.lib {
        s.push_str("; plus the lib unit tests (a source path changed)");
    } else if t.tests.is_empty() {
        s.push_str(", nothing to run");
    }
    s
}

/// `land`'s first call, BEFORE the tree gate: compute the set and judge the line endings (issue478).
///
/// Self-build only. `None` = a downstream tree, or one whose set could not be computed - `land` gates
/// and pushes without a touched run. `Some(Err(code))` = refuse with that exit code; `Some(Ok(t))` =
/// the set to hand to [`after_gate`] once the tree gate is green.
///
/// WHY BEFORE THE GATE: guard `working-tree-eol` reads the same census, so a mismatch would otherwise
/// surface as one of N gate problems and this line - the changed paths first, the count of the rest,
/// the receipt - would never be reached. The census is one `git ls-files --eol` per run either way.
#[must_use]
pub fn before_gate(repo: &Path) -> Option<Result<Touched, i32>> {
    if !crate::suite::is_self_build(repo) {
        return None;
    }
    let t = match compute(repo) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("keel land: touched tests could not be computed ({e}) - pushing without them");
            return None;
        }
    };
    println!("keel land: {}", describe(&t));
    // issue478: the bytes cargo would compile are judged BEFORE the run and before the empty-set
    // shortcut - a CRLF `.sysml` under `eol=lf` names no module, and it is still a tree this push
    // must not carry to a machine whose tests read it exactly. Not behind the D0421 arming: the
    // touched run's verdict depended on the tree's endings, and a verdict that depends on which tool
    // last wrote a file is not the verdict D0421 armed.
    if let Some(line) = t.eol_refusal() {
        write_receipt(repo, &t, Phase::EolMismatch);
        eprintln!("keel land: {line}");
        eprintln!("  REFUSING to push: the touched tests would read these bytes, not the ones git normalises at the commit (issue478). Receipt {RECEIPT} says eol-mismatch. Nothing was pushed.");
        return Some(Err(1));
    }
    println!("keel land: {}", t.eol_line());
    Some(Ok(t))
}

/// `land`'s call after the tree gate and before the first push, with the set [`before_gate`]
/// computed. `None` = continue to push; `Some(code)` = refuse with that exit code.
#[must_use]
pub fn after_gate(repo: &Path, t: &Touched) -> Option<i32> {
    if t.nothing_to_run() {
        write_receipt(repo, t, Phase::NotRun);
        return None;
    }
    if !gate_accepted(repo) {
        println!("keel land: not run - D0421 is proposed; the touched-test refusal is declared but INERT until the human's word (D0337); the set is in the receipt ({RECEIPT}).");
        write_receipt(repo, t, Phase::NotRun);
        return None;
    }
    if let Some(reason) = crate::suite::own_image_refusal(repo, "keel land") {
        eprintln!("{reason}");
        return Some(2);
    }
    println!("keel land: running {} touched test binar{} before the push (cargo test --release --test ...)", t.tests.len(), if t.tests.len() == 1 { "y" } else { "ies" });
    match run(repo, t) {
        Ok(r) if r.cargo_ok && r.failed == 0 => {
            println!("keel land: touched tests pass - {} passed in {}s (receipt {RECEIPT})", r.passed, r.seconds);
            None
        }
        Ok(r) => {
            eprintln!("keel land: touched tests FAIL - REFUSING to push. Failing: [{}] ({} passed, {} failed, {}s; log {})", r.failing.join(", "), r.passed, r.failed, r.seconds, r.log.display());
            eprintln!("  These tests name a module this push changes. Fix them (or the module), commit, and re-run. Nothing was pushed.");
            Some(1)
        }
        Err(e) => {
            eprintln!("keel land: touched tests could not be run ({e}) - REFUSING to push: the set is non-empty and unverified.");
            Some(2)
        }
    }
}

/// `keel suite --touched [ROOT]`: compute and run the set now, on the caller's word.
#[must_use]
pub fn cmd(repo: &Path) -> i32 {
    if !crate::suite::is_self_build(repo) {
        eprintln!("keel suite --touched: {} holds no keel-cli/Cargo.toml - there is no test set to compute here", repo.display());
        return 2;
    }
    let t = match compute(repo) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("keel suite --touched: {e}");
            return 2;
        }
    };
    println!("keel suite --touched: {}", describe(&t));
    if let Some(line) = t.eol_refusal() {
        write_receipt(repo, &t, Phase::EolMismatch);
        eprintln!("keel suite --touched: {line}");
        eprintln!("  REFUSING to run: cargo would compile and test these bytes, not the ones git normalises at the commit (issue478). Receipt {RECEIPT} says eol-mismatch; nothing was measured.");
        return 1;
    }
    println!("keel suite --touched: {}", t.eol_line());
    if t.nothing_to_run() {
        write_receipt(repo, &t, Phase::NotRun);
        println!("keel suite --touched: empty set recorded in {RECEIPT}");
        return 0;
    }
    if let Some(reason) = crate::suite::own_image_refusal(repo, "keel suite --touched") {
        eprintln!("{reason}");
        return 2;
    }
    match run(repo, &t) {
        Ok(r) => {
            for l in std::fs::read_to_string(&r.log).unwrap_or_default().lines().filter(|l| l.contains("FAILED") || l.contains("panicked at")).take(20) {
                println!("  {l}");
            }
            let outcome = if r.cargo_ok && r.failed == 0 { "pass" } else { "fail" };
            println!("keel suite --touched: {outcome} - {} passed, {} failed in {}s; failing [{}]; receipt {RECEIPT}", r.passed, r.failed, r.seconds, r.failing.join(", "));
            if outcome == "pass" { 0 } else { 101 }
        }
        Err(e) => {
            eprintln!("keel suite --touched: {e}");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    fn fixture() -> Touched {
        Touched {
            base: "origin/main".into(),
            stems: vec!["scaffold".into()],
            unattributed: vec![],
            tests: vec!["orient_bdd".into()],
            lib: true,
            changed: vec!["keel-cli/src/scaffold.rs".into()],
            eol: vec![],
            eol_scanned: 3,
            eol_millis: 7,
        }
    }

    /// issue478 known-positive: a set whose tree holds a CRLF `eol=lf` path renders `eol-mismatch` with
    /// the paths, no verdict and no run; the refusal line names the changed path first and counts the
    /// rest. Known-negative: a clean census renders no refusal and the header carries its population.
    #[test]
    fn an_eol_mismatch_is_a_receipt_in_place_of_a_verdict() {
        let mut t = fixture();
        assert!(t.eol_refusal().is_none(), "a clean census refuses nothing");
        let clean = render_receipt(&t, "1234567", 100, Phase::NotRun);
        assert!(clean.contains("eol_scanned = 3\n") && clean.contains("eol_ms = 7\n"), "{clean}");
        t.eol = vec![
            crate::eol::Mismatch { path: "keel-cli/src/scaffold.rs".into(), declared: "lf".into(), worktree: "crlf".into() },
            crate::eol::Mismatch { path: ".tracking/backlog.sysml".into(), declared: "lf".into(), worktree: "crlf".into() },
        ];
        let line = t.eol_refusal().expect("a mismatch refuses");
        assert!(line.contains("changed by this push: [keel-cli/src/scaffold.rs (w/crlf where eol=lf)]"), "{line}");
        assert!(line.contains("and 1 unchanged path (first: .tracking/backlog.sysml)"), "{line}");
        let text = render_receipt(&t, "1234567", 100, Phase::EolMismatch);
        assert!(text.contains("outcome = \"eol-mismatch\""), "{text}");
        assert!(text.contains("eol_mismatch = [\"keel-cli/src/scaffold.rs\", \".tracking/backlog.sysml\"]"), "{text}");
        assert!(text.contains("passed = 0\nfailed = 0\n"), "{text}");
        assert!(!text.contains("\"pass\"") && !text.contains("\"running\""), "{text}");
    }

    /// issue468, known-negative: a finished run's receipt carries its verdict and counts.
    #[test]
    fn a_finished_run_renders_its_verdict() {
        let run = Run { passed: 5, failed: 1, failing: vec!["lib".into()], seconds: 9, cargo_ok: false, log: std::path::PathBuf::from("x.log") };
        let text = render_receipt(&fixture(), "1234567", 100, Phase::Done(&run));
        assert!(text.contains("outcome = \"fail\""), "{text}");
        assert!(text.contains("passed = 5") && text.contains("failed = 1"), "{text}");
        assert!(!text.contains("outcome = \"running\""), "{text}");
    }

    /// issue468, known-positive: the stub written before cargo starts says `running`, counts nothing,
    /// names THIS run's log and set - so a reader during the run, or after a killed one, never sees
    /// the previous run's pass over a different change set.
    #[test]
    fn a_running_stub_is_not_a_verdict() {
        let log = std::path::PathBuf::from(".keel/metrics/touched-100.log");
        let text = render_receipt(&fixture(), "1234567", 103, Phase::Running { started: 100, log: &log });
        assert!(text.contains("outcome = \"running\""), "{text}");
        assert!(text.contains("passed = 0") && text.contains("failed = 0"), "{text}");
        assert!(text.contains("seconds = 3"), "{text}");
        assert!(text.contains("touched-100.log"), "{text}");
        assert!(text.contains("\"scaffold\"") && text.contains("lib = true"), "the stub carries the set it is running:\n{text}");
        assert!(!text.contains("\"pass\""), "{text}");
    }
    use super::{compute, failing_binaries, module_stem, names_stem, render_receipt, test_name, text_carries_acceptance, touched_tests, Phase, Run, Touched};

    #[test]
    fn a_stem_is_the_module_a_path_names() {
        assert_eq!(module_stem("keel-cli/src/sync.rs"), Some("sync".to_string()));
        assert_eq!(module_stem("keel-cli\\src\\view\\mod.rs"), Some("view".to_string()));
        assert_eq!(module_stem("keel-cli/src/view/table.rs"), Some("table".to_string()));
        assert_eq!(module_stem("keel-cli/src/main.rs"), None);
        assert_eq!(module_stem("keel-cli/src/lib.rs"), None);
        assert_eq!(module_stem("keel-cli/tests/sync.rs"), None);
        assert_eq!(module_stem(".engine/tools/x.rs"), None);
        assert_eq!(test_name("keel-cli/tests/land_gate.rs"), Some("land_gate".to_string()));
        assert_eq!(test_name("keel-cli/tests/common/mod.rs"), None);
    }

    #[test]
    fn a_stem_is_named_as_a_whole_word_only() {
        assert!(names_stem("use keel_cli::sync::cmd_land;", "sync"));
        assert!(names_stem("runs `keel sync` twice", "sync"));
        assert!(names_stem("sync.rs owns it", "sync"));
        assert!(names_stem("sync", "sync"));
        assert!(!names_stem("the tree is synced and resync is off", "sync"));
        assert!(!names_stem("nothing here", "sync"));
        assert!(!names_stem("anything", ""));
    }

    #[test]
    fn the_set_is_the_tests_naming_a_changed_stem_plus_the_tests_that_changed() {
        let tests = vec![
            ("land_gate".to_string(), "keel_cli::sync::cmd_land".to_string()),
            ("suite_receipt".to_string(), "keel suite writes a receipt".to_string()),
            ("unrelated".to_string(), "nothing named".to_string()),
        ];
        // known positive: sync changed -> land_gate; the changed test itself joins
        let got = touched_tests(&tests, &["sync".to_string()], &["unrelated".to_string()]);
        assert_eq!(got, vec!["land_gate".to_string(), "unrelated".to_string()]);
        // known negative: a module nothing names -> empty
        assert!(touched_tests(&tests, &["orient".to_string()], &[]).is_empty());
    }

    /// Known-positive: the shape `keel accept` writes. Known-negatives: the token named in the decision
    /// prose (issue435, the live failure), a declared acceptance whose outcome is fail, and no token.
    #[test]
    fn arming_reads_the_acceptance_part_not_the_token_in_prose() {
        let accepted = "package D { part d0421 : Decision { :>> decision = \"armed by d0421AcceptR1\"; }\n    part d0421AcceptR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc\"; }\n}\n";
        assert!(text_carries_acceptance(accepted, "d0421"));
        let prose_only = "package D { part d0421 : Decision { :>> decision = \"once this Decision carries d0421AcceptR1 in its file\"; } }\n";
        assert!(!text_carries_acceptance(prose_only, "d0421"));
        let failed = "package D {\n    part d0421AcceptR1 : TestResult { :>> outcome = VerdictKind::fail; }\n}\n";
        assert!(!text_carries_acceptance(failed, "d0421"));
        assert!(!text_carries_acceptance("package D { }", "d0421"));
    }

    /// Known-positive: a capture in cargo's real order - every stdout result first, then stderr with
    /// the headers, the rerun hint and the `--no-fail-fast` summary. Known-negative: a green run.
    #[test]
    fn failing_binaries_are_read_from_cargos_rerun_hint() {
        let out = "running 1 test\ntest x ... FAILED\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored\nrunning 3 tests\ntest result: ok. 3 passed; 0 failed; 0 ignored\n     Running tests\\land_gate.rs (target\\release\\deps\\land_gate-abc.exe)\nerror: test failed, to rerun pass `--test land_gate`\n     Running tests\\other.rs (target\\release\\deps\\other-def.exe)\nerror: 1 target failed:\n    `--test land_gate`\n";
        assert_eq!(failing_binaries(out), vec!["land_gate".to_string()]);
        let two = "error: 2 targets failed:\n    `--test b_gate`\n    `--test a_gate`\n";
        assert_eq!(failing_binaries(two), vec!["a_gate".to_string(), "b_gate".to_string()]);
        let lib = "error: test failed, to rerun pass `--lib`\nerror: 2 targets failed:\n    `--lib`\n    `--test a_gate`\n";
        assert_eq!(failing_binaries(lib), vec!["a_gate".to_string(), "lib".to_string()]);
        assert!(failing_binaries("test result: ok. 1 passed; 0 failed\n     Running tests/other.rs (x)\n").is_empty());
    }

    /// The set is what the WORKING TREE changed, not what HEAD did (issue463): with HEAD equal to the
    /// base and edits uncommitted, the verifier's pre-commit run read an empty set. Known-negative: a
    /// clean tree whose last commit touched no module reads no stem. Known-positive: an uncommitted edit
    /// to a tracked module and an untracked new module both read as changed.
    #[test]
    fn the_changed_set_is_the_working_trees_not_heads() {
        let dir = std::env::temp_dir().join(format!("keel-touched-{}", crate::write::gen_uuid()));
        std::fs::create_dir_all(dir.join("keel-cli").join("src")).unwrap();
        std::fs::create_dir_all(dir.join("keel-cli").join("tests")).unwrap();
        let git = |args: &[&str]| {
            let o = crate::gitx::git().arg("-C").arg(&dir).args(args).output().unwrap();
            assert!(o.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&o.stderr));
            String::from_utf8_lossy(&o.stdout).trim().to_string()
        };
        git(&["init", "-q"]);
        std::fs::write(dir.join("keel-cli").join("src").join("foo.rs"), "pub fn foo() {}\n").unwrap();
        std::fs::write(dir.join("keel-cli").join("tests").join("foo_bites.rs"), "// names foo\n").unwrap();
        git(&["add", "-A"]);
        git(&["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q", "-m", "base"]);
        // A second commit touching no module, so the base (HEAD~1, no remote here) differs from HEAD by nothing under keel-cli.
        git(&["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q", "--allow-empty", "-m", "head"]);
        let clean = compute(&dir).unwrap();
        assert!(clean.stems.is_empty(), "known-negative: a clean tree reads no stem, got {:?}", clean.stems);
        assert!(!clean.lib, "known-negative: no source path changed");

        std::fs::write(dir.join("keel-cli").join("src").join("foo.rs"), "pub fn foo() { /* edited, uncommitted */ }\n").unwrap();
        std::fs::write(dir.join("keel-cli").join("src").join("newmod.rs"), "pub fn newmod() {}\n").unwrap();
        let dirty = compute(&dir).unwrap();
        assert_eq!(dirty.stems, vec!["foo".to_string(), "newmod".to_string()], "known-positive: the uncommitted edit and the untracked module are the change");
        assert!(dirty.lib, "a source path changed, so the lib's own tests run");
        assert_eq!(dirty.tests, vec!["foo_bites".to_string()], "the test naming the changed stem is in the set");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
