//! `keel suite [-- <cargo test args>]` and the receipt `keel land` demands (D0353,
//! dcLandRequiresASuiteReceipt).
//!
//! On 2026-09-02 sprint 521 landed on validate plus guards alone; the full suite had last run at
//! sprint 520; CI went red for twenty minutes on a test the suite would have caught locally. CI is
//! DETECTIVE - the computed control structure says so - and the commit was already on main. The
//! preventive control is the one place a push originates: `land` refuses a tree whose DELIVERABLE
//! changed since the last green suite run, unless the suite ran at this tree.
//!
//! THE RECEIPT is machine-local (`.keel/metrics/suite-receipt.toml`, beside the hook fire-ledger):
//! the fingerprint of the deliverable as it was tested, the HEAD it was tested near, when, and the
//! counts. It is evidence about THIS machine's run and never travels - CI reruns the suite itself.
//!
//! THE FINGERPRINT is over the deliverable's CONTENT ON DISK, tracked or not: `keel-cli/`, the
//! embedded `.engine/`, `keelw`, and the two Cargo manifests - what the binary is built from and what
//! the tests read. A docs-only or `.tracking/` change leaves it unchanged, so a receipt taken before
//! such a change still covers the tree; a source edit after the receipt is exactly what must refuse.

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
        "# suite receipt (D0353): the deliverable as the full suite last saw it ON THIS MACHINE. `keel land`\n# refuses a tree whose deliverable fingerprint differs from this one, or whose run was not green.\nfingerprint = \"{}\"\nhead = \"{}\"\nat = {}\npassed = {}\nfailed = {}\noutcome = \"{}\"\nlog = \"{}\"\n",
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

/// Why `land` may not push this tree, or `None` when the receipt covers it (or there is no suite).
///
/// The receipt must exist, be green, and name the fingerprint of the deliverable AS IT IS NOW.
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
        Some(r) if !r.green() => Some(format!("the last suite run on this machine was RED ({} passed, {} failed; head {}). Fix, run `keel suite` to green, then land.", r.passed, r.failed, r.head)),
        Some(r) if r.fingerprint != now => Some(format!("the deliverable CHANGED since the last green suite run (receipt {} at head {}, {} passed; the tree now fingerprints {}). Run `keel suite`, then land.", &r.fingerprint[..12], r.head, r.passed, &now[..12])),
        Some(_) => None,
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// `keel suite [-- <cargo test args>]`: run the full suite, write the log and the receipt, exit as
/// cargo did. Always `--no-fail-fast`, so the receipt's counts are the whole population.
#[must_use]
pub fn cmd(args: &[String], repo: &Path) -> i32 {
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
    let started = now_secs();
    let log = metrics.join(format!("suite-{started}.log"));
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
    let outcome = if out.status.success() && failed == 0 { "pass" } else { "fail" };
    let head = crate::gitx::git().arg("-C").arg(repo).args(["rev-parse", "--short", "HEAD"]).output().ok().filter(|o| o.status.success()).map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
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

/// A path for tests to plant a receipt.
#[must_use]
pub fn receipt_path(repo: &Path) -> PathBuf {
    repo.join(RECEIPT)
}

#[cfg(test)]
mod tests {
    use super::{count_results, parse_receipt};

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
}
