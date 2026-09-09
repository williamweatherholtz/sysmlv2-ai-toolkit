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
//! as changed and says so.
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

/// Compute the touched set for `repo` at HEAD. Self-build only (the caller checks).
///
/// # Errors
/// When git cannot list the changed paths (`diff --name-only` against the base, or `ls-files` when
/// no base resolves).
pub fn compute(repo: &Path) -> Result<Touched, String> {
    let (base, changed): (String, Vec<String>) = if let Some(b) = base_ref(repo) {
        let range = format!("{b}...HEAD");
        let out = git_out(repo, &["diff", "--name-only", &range]).or_else(|| git_out(repo, &["diff", "--name-only", &format!("{b}..HEAD")])).ok_or_else(|| format!("git diff --name-only {range} failed"))?;
        (b, out.lines().map(str::to_string).collect())
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
    Ok(Touched { base, stems, unattributed, tests })
}

/// Is the land refusal ARMED - has the human accepted D0421? The D0338 pattern: the decision file
/// carries its first acceptance result once `keel accept` has run.
#[must_use]
pub fn gate_accepted(repo: &Path) -> bool {
    let Ok(rd) = std::fs::read_dir(repo.join(".engine").join("decisions")) else { return false };
    rd.flatten().any(|e| {
        let name = e.file_name().to_string_lossy().to_string();
        name.starts_with("0421-") && std::fs::read_to_string(e.path()).is_ok_and(|t| t.contains("d0421AcceptR1"))
    })
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

fn render_receipt(t: &Touched, head: &str, at: u64, run: Option<&Run>) -> String {
    use std::fmt::Write as _;
    let list = |v: &[String]| v.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", ");
    let mut s = format!(
        "# touched receipt (D0421): the integration tests that NAME a module changed since the base, and what\n# running exactly those cost. An empty set is a receipt too. Beside the suite's receipt, never in it.\nhead = \"{}\"\nat = {}\nbase = \"{}\"\nstems = [{}]\nunattributed = [{}]\ntests = [{}]\n",
        head,
        at,
        t.base,
        list(&t.stems),
        list(&t.unattributed),
        list(&t.tests)
    );
    match run {
        Some(r) => {
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
        None => {
            let _ = write!(s, "outcome = \"{}\"\npassed = 0\nfailed = 0\nseconds = 0\n", if t.tests.is_empty() { "empty" } else { "not-run" });
        }
    }
    s
}

fn write_receipt(repo: &Path, t: &Touched, run: Option<&Run>) {
    let metrics = repo.join(".keel").join("metrics");
    let _ = std::fs::create_dir_all(&metrics);
    let head = git_out(repo, &["rev-parse", "--short", "HEAD"]).unwrap_or_default();
    if let Err(e) = crate::write::write_atomic(&repo.join(RECEIPT), render_receipt(t, &head, now_secs(), run)) {
        eprintln!("touched: receipt could not be written: {e}");
    }
}

/// Run exactly `t.tests` as one cargo invocation (`--release`, so the binaries CI links are the ones
/// exercised; `--no-fail-fast`, so every named test reports). Writes the log and the receipt.
///
/// # Errors
/// When the metrics directory cannot be created or cargo cannot be started at all.
pub fn run(repo: &Path, t: &Touched) -> Result<Run, String> {
    if t.tests.is_empty() {
        write_receipt(repo, t, None);
        return Ok(Run { passed: 0, failed: 0, failing: vec![], seconds: 0, cargo_ok: true, log: PathBuf::new() });
    }
    let metrics = repo.join(".keel").join("metrics");
    std::fs::create_dir_all(&metrics).map_err(|e| format!("cannot create {}: {e}", metrics.display()))?;
    let started = now_secs();
    let log = metrics.join(format!("touched-{started}.log"));
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("test").arg("--release").arg("--manifest-path").arg(repo.join("keel-cli").join("Cargo.toml")).arg("--no-fail-fast");
    for name in &t.tests {
        cmd.arg("--test").arg(name);
    }
    let out = cmd.current_dir(repo).output().map_err(|e| format!("cargo could not be run: {e}"))?;
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    let _ = std::fs::write(&log, &text);
    let (passed, failed) = crate::suite::count_results(&text);
    // A build failure is not a verdict about the tests - but it IS a reason not to push: the
    // binaries CI will link do not link here either. Every named test is reported as not run.
    let failing = if crate::suite::never_ran(out.status.success(), passed, failed) { t.tests.clone() } else { failing_binaries(&text) };
    let r = Run { passed, failed, failing, seconds: now_secs().saturating_sub(started), cargo_ok: out.status.success(), log };
    write_receipt(repo, t, Some(&r));
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
        s.push_str("; set EMPTY - no integration test names a changed module, nothing to run");
    } else {
        let _ = write!(s, "; set [{}]", t.tests.join(", "));
    }
    s
}

/// `land`'s call, after the tree gate and before the first push. `None` = continue to push;
/// `Some(code)` = refuse with that exit code. Self-build only; a downstream tree is untouched.
#[must_use]
pub fn before_push(repo: &Path) -> Option<i32> {
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
    if t.tests.is_empty() {
        write_receipt(repo, &t, None);
        return None;
    }
    if !gate_accepted(repo) {
        println!("keel land: not run - D0421 is proposed; the touched-test refusal is declared but INERT until the human's word (D0337). Run them yourself with `keel suite --touched`.");
        write_receipt(repo, &t, None);
        return None;
    }
    if let Some(reason) = crate::suite::own_image_refusal(repo, "keel land") {
        eprintln!("{reason}");
        return Some(2);
    }
    println!("keel land: running {} touched test binar{} before the push (cargo test --release --test ...)", t.tests.len(), if t.tests.len() == 1 { "y" } else { "ies" });
    match run(repo, &t) {
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
    if t.tests.is_empty() {
        write_receipt(repo, &t, None);
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
    use super::{failing_binaries, module_stem, names_stem, test_name, touched_tests};

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

    /// Known-positive: a capture in cargo's real order - every stdout result first, then stderr with
    /// the headers, the rerun hint and the `--no-fail-fast` summary. Known-negative: a green run.
    #[test]
    fn failing_binaries_are_read_from_cargos_rerun_hint() {
        let out = "running 1 test\ntest x ... FAILED\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored\nrunning 3 tests\ntest result: ok. 3 passed; 0 failed; 0 ignored\n     Running tests\\land_gate.rs (target\\release\\deps\\land_gate-abc.exe)\nerror: test failed, to rerun pass `--test land_gate`\n     Running tests\\other.rs (target\\release\\deps\\other-def.exe)\nerror: 1 target failed:\n    `--test land_gate`\n";
        assert_eq!(failing_binaries(out), vec!["land_gate".to_string()]);
        let two = "error: 2 targets failed:\n    `--test b_gate`\n    `--test a_gate`\n";
        assert_eq!(failing_binaries(two), vec!["a_gate".to_string(), "b_gate".to_string()]);
        assert!(failing_binaries("test result: ok. 1 passed; 0 failed\n     Running tests/other.rs (x)\n").is_empty());
    }
}
