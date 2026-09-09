//! `keel suite [ROOT] [-- <cargo test args>]` — run the full suite and record what that run cost.
//!
//! IT NO LONGER GATES ANYTHING (D0356). For one day a green receipt was required before any push.
//! Measured honestly the run costs about eleven wall minutes every time the code moves, against
//! roughly one bad push in twenty-five it could catch — and two of the three most recent failures
//! were platform faults this machine cannot reproduce. The owner withdrew it. What stays is the
//! measurement: the receipt is how anyone knows what a full run costs, which is the number that
//! decided the question.
//!
//! THE RECEIPT is machine-local (`.keel/metrics/suite-receipt.toml`, beside the hook fire-ledger):
//! the fingerprint of the deliverable as it was tested, the HEAD it was tested near, when, and the
//! counts. It is evidence about THIS machine's run and never travels — CI reruns the suite itself.
//!
//! THE FINGERPRINT is over the deliverable's CONTENT ON DISK, tracked or not: `keel-cli/`, the
//! embedded `.engine/`, `keelw`, and the two Cargo manifests. It still answers "was this exact tree
//! tested", which is worth knowing even when nothing refuses on the answer.
//!
//! THE RECEIPT SAYS WHAT WAS MEASURED (issue386). Two ways a run can end without measuring the code
//! are told apart from a red: launched from the very image `cargo test --release` relinks, the
//! command refuses BEFORE the running stub (on Windows the file is locked; the copy remedy is named);
//! and a cargo exit with no `test result:` line restores the previous receipt rather than writing
//! `fail - 0 passed, 0 failed` over a tree the tests never saw.

use sha2::{Digest as _, Sha256};
use std::path::{Path, PathBuf};

/// The paths whose content is the deliverable, repo-relative.
pub const DELIVERABLE_PATHS: [&str; 5] = ["keel-cli", ".engine", "keelw", "Cargo.toml", "Cargo.lock"];

/// Where the receipt lives, repo-relative.
pub const RECEIPT: &str = ".keel/metrics/suite-receipt.toml";

/// Is this repository the self-build (the one with a suite to run)?
#[must_use]
pub fn is_self_build(repo: &Path) -> bool {
    repo.join("keel-cli").join("Cargo.toml").is_file()
}

/// The deliverable fingerprint: SHA-256 over `(path, content)` for every file git knows or would add
/// under `DELIVERABLE_PATHS`, sorted by path, content read from DISK so an uncommitted edit counts.
///
/// # Errors
/// When git cannot list the tree.
pub fn fingerprint(repo: &Path) -> Result<String, String> {
    let mut files: Vec<String> = Vec::new();
    for args in [vec!["ls-files", "-z", "--"], vec!["ls-files", "-z", "-o", "--exclude-standard", "--"]] {
        let mut a: Vec<&str> = args;
        a.extend(DELIVERABLE_PATHS);
        let out = crate::gitx::git().arg("-C").arg(repo).args(&a).output().map_err(|e| format!("git ls-files: {e}"))?;
        if !out.status.success() {
            return Err(format!("git ls-files failed: {}", String::from_utf8_lossy(&out.stderr).trim()));
        }
        files.extend(String::from_utf8_lossy(&out.stdout).split('\0').filter(|p| !p.is_empty()).map(str::to_owned));
    }
    files.sort();
    files.dedup();
    let mut h = Sha256::new();
    for rel in &files {
        let Ok(bytes) = std::fs::read(repo.join(rel)) else { continue }; // deleted on disk: absent from the hash
        h.update(rel.as_bytes());
        h.update([0u8]);
        h.update(&bytes);
        h.update([0u8]);
    }
    Ok(crate::device::hex(&h.finalize()))
}

/// What the last suite run on this machine recorded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub fingerprint: String,
    pub head: String,
    pub at: u64,
    pub passed: u64,
    pub failed: u64,
    pub outcome: String,
}

impl Receipt {
    #[must_use]
    pub fn green(&self) -> bool {
        self.outcome == "pass" && self.failed == 0
    }
}

/// Parse a receipt's text (pure, tested).
#[must_use]
pub fn parse_receipt(text: &str) -> Option<Receipt> {
    let v = text.parse::<toml::Value>().ok()?;
    let s = |k: &str| v.get(k).and_then(toml::Value::as_str).map(str::to_owned);
    let n = |k: &str| v.get(k).and_then(toml::Value::as_integer).and_then(|i| u64::try_from(i).ok());
    Some(Receipt { fingerprint: s("fingerprint")?, head: s("head").unwrap_or_default(), at: n("at").unwrap_or(0), passed: n("passed").unwrap_or(0), failed: n("failed").unwrap_or(0), outcome: s("outcome").unwrap_or_else(|| "fail".into()) })
}

fn render_receipt(r: &Receipt, log: &Path) -> String {
    format!(
        "# suite receipt: the deliverable as the full suite last saw it ON THIS MACHINE, and what that run\n# cost. Nothing refuses on it (D0356) - it is a measurement, not a gate. `outcome = \"running\"` is the\n# stub written before cargo starts (D0387): a run in progress, or one that was killed - not an answer.\nfingerprint = \"{}\"\nhead = \"{}\"\nat = {}\npassed = {}\nfailed = {}\noutcome = \"{}\"\nlog = \"{}\"\n",
        r.fingerprint, r.head, r.at, r.passed, r.failed, r.outcome, log.to_string_lossy().replace('\\', "/")
    )
}

/// Read this machine's receipt, if any.
#[must_use]
pub fn receipt(repo: &Path) -> Option<Receipt> {
    parse_receipt(&std::fs::read_to_string(repo.join(RECEIPT)).ok()?)
}

/// Sum `test result:` lines of a cargo test run (pure, tested).
#[must_use]
pub fn count_results(output: &str) -> (u64, u64) {
    let mut passed = 0u64;
    let mut failed = 0u64;
    for l in output.lines().filter(|l| l.starts_with("test result:")) {
        let words: Vec<&str> = l.split_whitespace().collect();
        for (i, w) in words.iter().enumerate() {
            if *w == "passed;" || *w == "passed" {
                passed += words.get(i.wrapping_sub(1)).and_then(|n| n.parse::<u64>().ok()).unwrap_or(0);
            }
            if *w == "failed;" || *w == "failed" {
                failed += words.get(i.wrapping_sub(1)).and_then(|n| n.parse::<u64>().ok()).unwrap_or(0);
            }
        }
    }
    (passed, failed)
}

/// Whether the last suite run covers the tree as it stands — `None` when it does (or there is no
/// suite), otherwise the reason it does not.
///
/// NOT WIRED TO ANYTHING SINCE D0356: `land` no longer consults it. Kept because "is this tree
/// tested" is a real question a human or a later control may want answered, and the answer is
/// cheap; deleting it would also delete the only place that knows what staleness looks like.
#[must_use]
pub fn land_refusal(repo: &Path) -> Option<String> {
    if !is_self_build(repo) {
        return None;
    }
    let now = match fingerprint(repo) {
        Ok(f) => f,
        Err(e) => return Some(format!("the deliverable could not be fingerprinted ({e})")),
    };
    match receipt(repo) {
        None => Some(format!("no suite receipt at {RECEIPT} - the full suite has not run on this machine since the receipt existed. Run `keel suite` (it writes the receipt), then land.")),
        Some(r) if r.outcome == "running" => Some(format!("a suite run started at {} on this machine and has not completed (or was killed) - its receipt is a running stub, not a verdict. Wait for it or run `keel suite` again.", r.at)),
        Some(r) if !r.green() => Some(format!("the last suite run on this machine was RED ({} passed, {} failed; head {}). Fix, run `keel suite` to green, then land.", r.passed, r.failed, r.head)),
        Some(r) if r.fingerprint != now => Some(format!("the deliverable CHANGED since the last green suite run (receipt {} at head {}, {} passed; the tree now fingerprints {}). Run `keel suite`, then land.", &r.fingerprint[..12], r.head, r.passed, &now[..12])),
        Some(_) => None,
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// The directory `cargo test --release` writes for this repository: `CARGO_TARGET_DIR` when set,
/// else `<repo>/target`.
#[must_use]
pub fn target_dir(repo: &Path) -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR").map_or_else(|| repo.join("target"), PathBuf::from)
}

/// Is `exe` a file the release build REWRITES - `<target>/release/keel(.exe)` or anything under
/// `<target>/release/deps/`? Pure, so the collision is testable on a host that would not lock it.
///
/// issue386: on Windows a running image cannot be replaced, so a suite launched from the binary
/// cargo is about to relink fails with `Access is denied (os error 5)` before any test runs, and
/// used to record `fail - 0 passed, 0 failed` as if the code had been measured. Deliberately NARROW:
/// a copy at `<target>/release/keel-serve.exe` (the documented remedy, issue150) sits in the same
/// directory and does not collide, because cargo never writes it.
#[must_use]
pub fn image_collides(exe: &Path, target: &Path) -> bool {
    let release = target.join("release");
    let Ok(rel) = exe.strip_prefix(&release) else { return false };
    let mut parts = rel.components();
    let Some(first) = parts.next() else { return false };
    let first = first.as_os_str().to_string_lossy();
    if parts.next().is_none() {
        return first == format!("keel{}", std::env::consts::EXE_SUFFIX);
    }
    first == "deps"
}

/// The reason `who` (`keel suite`, `keel land`, ...) will not run cargo from this image, or `None`.
///
/// Refuses only where the lock is real (Windows); elsewhere the build replaces the file under a
/// running process without harm. Shared with the touched set (D0421), which links the same binaries.
#[must_use]
pub fn own_image_refusal(repo: &Path, who: &str) -> Option<String> {
    if !cfg!(windows) {
        return None;
    }
    let exe = std::env::current_exe().ok()?;
    let target = target_dir(repo);
    let exe_c = exe.canonicalize().unwrap_or_else(|_| exe.clone());
    let target_c = target.canonicalize().unwrap_or_else(|_| target.clone());
    if !(image_collides(&exe, &target) || image_collides(&exe_c, &target_c)) {
        return None;
    }
    let release = target.join("release");
    let copy = release.join(format!("keel-serve{}", std::env::consts::EXE_SUFFIX));
    Some(format!(
        "{who}: this command is running from {} - the very file `cargo test --release` relinks. On this host a running image cannot be replaced, so the build would fail with `Access is denied` before any test ran (issue386). Run it from a copy cargo does not write:\n  cp {} {}\n  {} {}\nNo receipt was written: nothing was measured.",
        exe.display(),
        release.join(format!("keel{}", std::env::consts::EXE_SUFFIX)).display(),
        copy.display(),
        copy.display(),
        who.strip_prefix("keel ").unwrap_or(who)
    ))
}

/// `keel suite [-- <cargo test args>]`: run the full suite, write the log and the receipt, exit as
/// cargo did. Always `--no-fail-fast`, so the receipt's counts are the whole population.
#[must_use]
pub fn cmd(args: &[String], repo: &Path) -> i32 {
    // --help must not RUN the suite. It did, once, and cost 185 seconds to discover.
    if args.iter().take_while(|a| *a != "--").any(|a| a == "--help" || a == "-h") {
        println!("usage: keel suite [ROOT] [--touched] [-- <cargo test args>]");
        println!("  runs the full suite (--release --no-fail-fast), logs under .keel/metrics/, and writes");
        println!("  {RECEIPT}: the deliverable fingerprint, counts and outcome of that run.");
        println!("  --touched: instead run ONLY the integration tests that name a module changed since the base");
        println!("  of the push (origin/<branch>, else the last suite receipt's head, else HEAD~1) and write");
        println!("  {}: the base, stems, set and cost - an empty set is recorded too (D0421).", crate::touched::RECEIPT);
        return 0;
    }
    if args.iter().take_while(|a| *a != "--").any(|a| a == "--touched") {
        return crate::touched::cmd(repo);
    }
    if !is_self_build(repo) {
        eprintln!("keel suite: {} holds no keel-cli/Cargo.toml - there is no suite to run here (a downstream project's gate is `keel gate`)", repo.display());
        return 2;
    }
    let extra: Vec<&String> = args.iter().skip_while(|a| *a != "--").skip(1).collect();
    let metrics = repo.join(".keel").join("metrics");
    if let Err(e) = std::fs::create_dir_all(&metrics) {
        eprintln!("keel suite: cannot create {}: {e}", metrics.display());
        return 1;
    }
    if let Some(reason) = own_image_refusal(repo, "keel suite") {
        eprintln!("{reason}");
        return 2;
    }
    let started = now_secs();
    let log = metrics.join(format!("suite-{started}.log"));
    // Kept so a run that never reaches a test can put it back: a receipt says what was MEASURED, and
    // a build failure measured nothing (issue386).
    let previous = std::fs::read_to_string(repo.join(RECEIPT)).ok();
    // D0387/issue399: the previous receipt is REPLACED by a running stub before cargo starts, so a run
    // that is killed leaves `outcome = "running"` - not green, not counted - rather than the last
    // completed run's verdict standing over a tree it never saw. Same fingerprint as the final receipt
    // will carry, so a reader comparing fingerprints is told the run is in progress, not stale.
    let head = crate::gitx::git().arg("-C").arg(repo).args(["rev-parse", "--short", "HEAD"]).output().ok().filter(|o| o.status.success()).map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
    if let Ok(fp) = fingerprint(repo) {
        let stub = Receipt { fingerprint: fp, head: head.clone(), at: started, passed: 0, failed: 0, outcome: "running".to_string() };
        if let Err(e) = crate::write::write_atomic(&repo.join(RECEIPT), render_receipt(&stub, &log)) {
            eprintln!("keel suite: running stub could not be written: {e}");
        }
    }
    println!("keel suite: cargo test --release --no-fail-fast (log -> {})", log.display());
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("test").arg("--release").arg("--manifest-path").arg(repo.join("keel-cli").join("Cargo.toml")).arg("--no-fail-fast");
    for a in extra {
        cmd.arg(a);
    }
    let out = match cmd.current_dir(repo).output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("keel suite: cargo could not be run: {e}");
            return 2;
        }
    };
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    let _ = std::fs::write(&log, &text);
    let (passed, failed) = count_results(&text);
    if never_ran(out.status.success(), passed, failed) {
        // The build (or cargo itself) failed before a single test binary reported: no verdict about
        // the code exists, so none is recorded. The previous receipt stands as what was last measured.
        match previous {
            Some(p) => {
                if let Err(e) = crate::write::write_atomic(&repo.join(RECEIPT), p) {
                    eprintln!("keel suite: previous receipt could not be restored: {e}");
                }
            }
            None => {
                let _ = std::fs::remove_file(repo.join(RECEIPT));
            }
        }
        for l in text.lines().filter(|l| l.starts_with("error")).take(5) {
            eprintln!("  {l}");
        }
        eprintln!("keel suite: cargo exited {} before any test ran - the deliverable was NOT measured, no receipt written (log -> {})", out.status.code().map_or_else(|| "by signal".to_string(), |c| c.to_string()), log.display());
        return 2;
    }
    let outcome = if out.status.success() && failed == 0 { "pass" } else { "fail" };
    let fp = match fingerprint(repo) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("keel suite: ran ({passed} passed, {failed} failed) but the deliverable could not be fingerprinted: {e} - no receipt written");
            return if outcome == "pass" { 1 } else { 101 };
        }
    };
    let r = Receipt { fingerprint: fp, head, at: started, passed, failed, outcome: outcome.to_string() };
    if let Err(e) = crate::write::write_atomic(&repo.join(RECEIPT), render_receipt(&r, &log)) {
        eprintln!("keel suite: receipt could not be written: {e}");
    }
    for l in text.lines().filter(|l| l.contains("FAILED") || l.contains("panicked at")).take(20) {
        println!("  {l}");
    }
    println!("keel suite: {outcome} - {passed} passed, {failed} failed; receipt {} (fingerprint {}...)", RECEIPT, &r.fingerprint[..12]);
    if outcome == "pass" { 0 } else { 101 }
}

/// Did cargo fail without a single `test result:` line - a build or tool failure, not a verdict?
/// Pure: `(cargo succeeded, passed, failed)`.
#[must_use]
pub const fn never_ran(cargo_ok: bool, passed: u64, failed: u64) -> bool {
    !cargo_ok && passed == 0 && failed == 0
}

/// A path for tests to plant a receipt.
#[must_use]
pub fn receipt_path(repo: &Path) -> PathBuf {
    repo.join(RECEIPT)
}

#[cfg(test)]
mod tests {
    use super::{count_results, image_collides, never_ran, parse_receipt, render_receipt, Receipt};
    use std::path::Path;

    /// THE CONTROL for issue386, meaningful on any host: the image cargo relinks collides, the
    /// documented copy beside it does not, and a run with no `test result:` line is not a verdict.
    #[test]
    fn the_suite_knows_its_own_image_and_a_run_that_never_ran() {
        let target = Path::new("repo").join("target");
        let release = target.join("release");
        let keel = format!("keel{}", std::env::consts::EXE_SUFFIX);
        assert!(image_collides(&release.join(&keel), &target), "the binary cargo writes");
        assert!(image_collides(&release.join("deps").join("keel-0123abcd.exe"), &target), "a test binary under deps");
        assert!(!image_collides(&release.join(format!("keel-serve{}", std::env::consts::EXE_SUFFIX)), &target), "the issue150 copy is what the remedy names");
        assert!(!image_collides(&target.join("debug").join(&keel), &target), "the debug build is not relinked by --release");
        assert!(!image_collides(Path::new("elsewhere").join(&keel).as_path(), &target), "an installed keel");
        assert!(never_ran(false, 0, 0), "cargo failed and nothing reported: not a verdict");
        assert!(!never_ran(false, 3, 1), "a real red is a verdict");
        assert!(!never_ran(true, 0, 0), "cargo succeeded with an empty filter: measured, trivially");
    }

    #[test]
    fn results_are_summed_across_every_test_binary() {
        let out = "test result: ok. 12 passed; 0 failed; 0 ignored\nnoise\ntest result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured\n";
        assert_eq!(count_results(out), (15, 1));
        assert_eq!(count_results("no results"), (0, 0));
    }

    #[test]
    fn a_receipt_round_trips_and_a_red_one_is_not_green() {
        let r = parse_receipt("fingerprint = \"abc\"\nhead = \"1234567\"\nat = 5\npassed = 10\nfailed = 0\noutcome = \"pass\"\n").expect("parses");
        assert!(r.green() && r.fingerprint == "abc" && r.passed == 10);
        let red = parse_receipt("fingerprint = \"abc\"\npassed = 9\nfailed = 1\noutcome = \"fail\"\n").expect("parses");
        assert!(!red.green());
        assert!(parse_receipt("nonsense = ").is_none());
    }

    #[test]
    fn a_running_stub_is_not_green_and_a_killed_run_leaves_it() {
        // D0387/issue399: the stub `cmd` writes before cargo starts is what a killed run leaves behind;
        // it must read as no verdict, never as the previous run's pass.
        let stub = Receipt { fingerprint: "abc".into(), head: "1234567".into(), at: 7, passed: 0, failed: 0, outcome: "running".into() };
        let text = render_receipt(&stub, Path::new("x.log"));
        let back = parse_receipt(&text).expect("parses");
        assert_eq!(back, stub);
        assert!(!back.green(), "a run in progress has no verdict");
        assert!(text.contains("running"), "the file says so in its own text");
    }
}
