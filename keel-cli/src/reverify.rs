//! `keel record reverify` (D0101) — auto-re-verify drift-suspect REPRODUCIBLE `method=test` verifications by
//! actually re-running the configured gate at HEAD and appending a fresh `TestResult` on green.
//!
//! Honest by construction: a fresh result is stamped ONLY after the real command passed at HEAD — it
//! never fabricates a pass (the honest-state invariant, D0098). Judgment-method verifications
//! (confirmation/inspect/analyze) are out of scope; only reproducible deliverable-drift tasks are
//! refreshed. The reverify gate is declared in `.engine/contracts/reverify.toml` (downstream-overridable).
//!
//! `keel record reverify --demos` (D0444) is the one examined method that crosses over: a `method=demo` pass
//! whose `// RAN:` receipt IS a command beginning with a prefix the contract declares under
//! `[demo] replayable` is exercised in substance, so the write path keeps it a `pass` (`is_replayable`,
//! read by `write::proposed_tier`) and `--demos` re-runs it at HEAD - a fresh pass with the same receipt,
//! or a fail naming the command and its exit code.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(serde::Deserialize, Default)]
struct ReverifyConfig {
    #[serde(default)]
    commands: Vec<String>,
    #[serde(default)]
    demo: DemoConfig,
}

/// `[demo]` (D0444): the command prefixes a `method=demo` receipt may begin with to count as
/// REPLAYABLE - a command the project declares safe for `keel record reverify --demos` to run.
#[derive(serde::Deserialize, Default)]
struct DemoConfig {
    #[serde(default)]
    replayable: Vec<String>,
}

/// Parse the reverify command list from `reverify.toml` text.
///
/// # Errors
/// Returns the TOML error string on malformed input.
pub fn parse_commands(toml_str: &str) -> Result<Vec<String>, String> {
    toml::from_str::<ReverifyConfig>(toml_str).map(|c| c.commands).map_err(|e| e.to_string())
}

/// Parse the `[demo] replayable` prefix list from `reverify.toml` text (D0444).
///
/// # Errors
/// Returns the TOML error string on malformed input.
pub fn parse_demo_prefixes(toml_str: &str) -> Result<Vec<String>, String> {
    toml::from_str::<ReverifyConfig>(toml_str).map(|c| c.demo.replayable).map_err(|e| e.to_string())
}

/// The project's declared demo prefixes. An absent or unparsable contract declares NONE, so a project
/// that never adopted the section has every AI demo pass land `proposed` exactly as before D0444.
#[must_use]
pub fn demo_prefixes(root: &Path) -> Vec<String> {
    std::fs::read_to_string(root.join(".engine").join("contracts").join("reverify.toml"))
        .ok()
        .and_then(|t| parse_demo_prefixes(&t).ok())
        .unwrap_or_default()
}

/// English function words no command line carries as a bare token. The receipt census that sized this
/// rule (2026-09-11, 283 prefixed `// RAN:` receipts in this tree) found exactly one prose receipt the
/// character alphabet alone let through - `keel validate . 762 clean and keel check-engine . clean at
/// 829e0f0 per VERIFIER_RECEIPT ...` - and every one of its joints is a word from this list. That census is
/// committed as `scripts/probes/receipt_shape_census.py`; it reads this list VERBATIM from this file, so
/// re-run it before changing the list (dcReceiptShapeCensusIsASensor).
const PROSE_WORDS: [&str; 16] = ["a", "an", "and", "at", "for", "from", "in", "is", "of", "on", "per", "the", "then", "to", "was", "with"];

/// D0444: is a `// RAN:` receipt a command a machine can re-run?
///
/// The receipt must BE the command, alone: one line, tokens from the shell-safe alphabet
/// (`[A-Za-z0-9_./:=@-]`) separated by single spaces, none of them an English function word, beginning
/// with one of the declared prefixes - or D0323's `ci-run id=<id> workflow=<name>` receipt, which CI
/// re-runs itself. A narrative that merely STARTS with `keel suite 571 passed 0 failed (receipt ...)`
/// is testimony about a command, not the command, and stays a proposal. `prefixes` is passed in so the
/// unit tests hold the rule against fixed cases; `is_replayable` binds it to the project's contract.
#[must_use]
pub fn is_replayable_with(evidence: &str, prefixes: &[String]) -> bool {
    let e = evidence.trim();
    if e.starts_with("ci-run id=") {
        return true;
    }
    if e.is_empty() || e.lines().count() != 1 || !prefixes.iter().any(|p| !p.is_empty() && e.starts_with(p.as_str())) {
        return false;
    }
    let token_ok = |t: &str| !t.is_empty() && !PROSE_WORDS.contains(&t) && t.chars().all(|c| c.is_ascii_alphanumeric() || "_./:=@-".contains(c));
    e.split(' ').all(token_ok)
}

/// `is_replayable_with` under the project's own `[demo] replayable` prefixes.
#[must_use]
pub fn is_replayable(root: &Path, evidence: &str) -> bool {
    is_replayable_with(evidence, &demo_prefixes(root))
}

/// One demo pass whose receipt replays (D0444): where it lives, what it verifies, what to run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoReplay {
    /// The `.tracking` file holding the Test and its results.
    pub file: PathBuf,
    /// The verification's name (`sDemoGate`, or `<task>DoD` for a task's `DoD`).
    pub test: String,
    /// The task when the verification is a `<task>DoD` the file declares as `action <task>;`.
    pub task: Option<String>,
    /// The `// RAN:` receipt - the command, verbatim.
    pub command: String,
}

/// The names of every `method=demo` verification declared in `text`.
#[must_use]
pub fn demo_tests_in_text(text: &str) -> HashSet<String> {
    let mut demos: HashSet<String> = HashSet::new();
    for cap in text.split("verification ").skip(1) {
        let Some(name) = cap.split([' ', ':']).next() else { continue };
        let head = cap.split('}').next().unwrap_or("");
        if head.contains(":>> method = VerificationMethod::demo") {
            demos.insert(name.to_string());
        }
    }
    demos
}

/// Every `method=demo` verification in `text` whose LATEST result is a `pass` carrying a replayable
/// receipt (the `// RAN:` line directly above it, or on the same line). Textual, like the census.
#[must_use]
pub fn demo_replays_in_text(text: &str, prefixes: &[String]) -> Vec<(String, String)> {
    let demos = demo_tests_in_text(text);
    let lines: Vec<&str> = text.lines().collect();
    // test -> (n, outcome, receipt) of the highest-numbered result
    let mut latest: std::collections::BTreeMap<String, (u32, String, Option<String>)> = std::collections::BTreeMap::new();
    for (i, line) in lines.iter().enumerate() {
        if !line.contains(" : TestResult {") {
            continue;
        }
        let Some(part) = line.split(" : TestResult").next().and_then(|s| s.split("part ").nth(1)) else { continue };
        let Some((base, n)) = part.trim().rsplit_once('R') else { continue };
        let Ok(n) = n.parse::<u32>() else { continue };
        if !demos.contains(base) {
            continue;
        }
        let outcome = line.split("VerdictKind::").nth(1).and_then(|s| s.split([';', ' ', '}']).next()).unwrap_or("").to_string();
        let receipt = line
            .split("// RAN:")
            .nth(1)
            .or_else(|| i.checked_sub(1).and_then(|j| lines.get(j)).and_then(|p| p.trim_start().strip_prefix("// RAN:")))
            .map(|r| r.trim().to_string());
        let e = latest.entry(base.to_string()).or_insert((0, String::new(), None));
        if n >= e.0 {
            *e = (n, outcome, receipt);
        }
    }
    latest
        .into_iter()
        .filter_map(|(test, (_, outcome, receipt))| {
            let r = receipt?;
            (outcome == "pass" && is_replayable_with(&r, prefixes) && !r.starts_with("ci-run id=")).then_some((test, r))
        })
        .collect()
}

/// The replayable demo passes of the whole `.tracking` tree.
#[must_use]
pub fn demo_replays(root: &Path) -> Vec<DemoReplay> {
    let prefixes = demo_prefixes(root);
    let mut out = Vec::new();
    for file in crate::collect_sysml(&root.join(".tracking")) {
        let Ok(text) = std::fs::read_to_string(&file) else { continue };
        for (test, command) in demo_replays_in_text(&text, &prefixes) {
            let task = test.strip_suffix("DoD").filter(|t| text.contains(&format!("action {t};"))).map(str::to_string);
            out.push(DemoReplay { file: file.clone(), test, task, command });
        }
    }
    out
}

/// How one replay runs: a leading `keel ` is THIS binary invoked directly with the receipt's tokens
/// (D0230 - the replay and the gate are the same engine; a replayable receipt's tokens carry no quote
/// or inner space by the alphabet rule, so splitting on whitespace is exact and no shell quoting can
/// mangle the binary's path), anything else runs as written through the shell.
#[derive(Debug, PartialEq, Eq)]
enum Replay {
    ThisBinary(Vec<String>),
    Shell(String),
}

fn replay_command(command: &str) -> Replay {
    command.strip_prefix("keel ").map_or_else(
        || Replay::Shell(command.to_string()),
        |rest| Replay::ThisBinary(rest.split_whitespace().map(str::to_string).collect()),
    )
}

/// Run one replay in `root`, returning the exit code (`None` when the process could not start or was
/// killed). Same isolation as `run_shell`: its own `CARGO_TARGET_DIR`, so a `cargo test` receipt never
/// relinks the running image (issue150).
fn replay_status(root: &Path, replay: &Replay) -> Option<i32> {
    let mut c = match replay {
        Replay::ThisBinary(args) => {
            let me = std::env::current_exe().ok()?;
            let mut c = Command::new(me);
            c.args(args);
            c
        }
        Replay::Shell(cmd) if cfg!(windows) => {
            let mut c = Command::new("cmd");
            c.arg("/C").arg(cmd);
            c
        }
        Replay::Shell(cmd) => {
            let mut c = Command::new("sh");
            c.arg("-c").arg(cmd);
            c
        }
    };
    c.current_dir(root)
        .env("CARGO_TARGET_DIR", root.join("target").join("reverify"))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .ok()
        .and_then(|s| s.code())
}

/// `keel record reverify --demos` (D0444): re-run every replayable demo receipt at HEAD and record each verdict.
///
/// A fresh `pass` carrying the SAME receipt on exit 0, a `fail` naming the command and its exit code
/// otherwise. The verdict is a line the replay produced, never an assertion about it.
///
/// Exit code: 0 when every replay passed (or there was nothing to replay), 1 when any failed, 2 when
/// no prefix is declared - the flag is inert until the project's contract says which commands run.
#[must_use]
pub fn replay_demos(root: &Path, by: &str) -> i32 {
    let prefixes = demo_prefixes(root);
    if prefixes.is_empty() {
        eprintln!("reverify --demos: .engine/contracts/reverify.toml declares no [demo] replayable prefixes - nothing is a replayable receipt here (D0444)");
        return 2;
    }
    let replays = demo_replays(root);
    if replays.is_empty() {
        println!("reverify --demos: no demo pass carries a replayable receipt - nothing to re-run");
        return 0;
    }
    let head = git_capture(root, &["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let date = crate::scaffold::today();
    println!("reverify --demos: re-running {} replayable demo receipt(s) at {head}", replays.len());
    let (mut passed, mut failed) = (0usize, 0usize);
    for r in &replays {
        let code = replay_status(root, &replay_command(&r.command));
        let (verdict, evidence) = match code {
            Some(0) => {
                passed += 1;
                ("pass", r.command.clone())
            }
            other => {
                failed += 1;
                let code_text = other.map_or_else(|| "no exit code (killed or not started)".to_string(), |c| format!("exit {c}"));
                ("fail", format!("replay of {} -> {code_text} (keel record reverify --demos at {head})", r.command))
            }
        };
        let written = r.task.as_ref().map_or_else(
            || crate::write::append_gate_result(&r.file, &r.test, &head, verdict, &date, by, None, Some(&evidence)),
            |task| crate::write::append_result(&r.file, task, &head, verdict, &date, by, Some(&evidence)),
        );
        match written {
            Ok(id) => println!("reverify --demos: {} {verdict} - `{}` ({id})", r.test, r.command),
            Err(e) => eprintln!("reverify --demos: {} {verdict} but could not record it: {e}", r.test),
        }
    }
    println!("reverify --demos: {passed} replayed green, {failed} failed, each recorded at HEAD {head} ({date})");
    i32::from(failed > 0)
}

/// The deliverable-manifest task names (`task: NAME | paths`) — the reproducible-reverify-eligible set.
#[must_use]
pub fn manifest_task_names(manifest_text: &str) -> HashSet<String> {
    manifest_text
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.starts_with('#') {
                return None;
            }
            let rest = l.strip_prefix("task:")?;
            let name = rest.split('|').next()?.trim();
            (!name.is_empty()).then(|| name.to_string())
        })
        .collect()
}

/// The drift-suspect deliverable tasks: the orient suspect set ∩ the manifest task names (source-drift,
/// reproducibly re-verifiable — excludes transitive/criterion suspects, which need judgment).
fn drift_tasks(root: &Path) -> Vec<String> {
    let manifest = std::fs::read_to_string(root.join(".engine").join("deliverable-manifest.txt")).unwrap_or_default();
    let names = manifest_task_names(&manifest);
    let mut out: Vec<String> = crate::orient::compute(root).suspect.into_iter().filter(|t| names.contains(t)).collect();
    out.sort();
    out
}

/// The `.tracking` file declaring `action <task>;` (where the task's `DoD` + results live).
fn find_task_file(root: &Path, task: &str) -> Option<PathBuf> {
    let needle = format!("action {task};");
    crate::collect_sysml(&root.join(".tracking")).into_iter().find(|f| std::fs::read_to_string(f).is_ok_and(|t| t.contains(&needle)))
}

fn git_capture(root: &Path, args: &[&str]) -> Option<String> {
    let out = crate::gitx::git().arg("-C").arg(root).args(args).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Run one reverify command as a shell command in `root`; returns true on exit 0.
///
/// Builds into an ISOLATED `target/reverify` dir (`CARGO_TARGET_DIR`) so a `cargo` gate never tries to
/// overwrite the running `keel` binary (self-replacement lock — "Access is denied" on Windows). Harmless
/// for non-cargo commands.
fn run_shell(root: &Path, cmd: &str) -> bool {
    let mut c = if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(cmd);
        c
    } else {
        let mut c = Command::new("sh");
        c.arg("-c").arg(cmd);
        c
    };
    c.current_dir(root).env("CARGO_TARGET_DIR", root.join("target").join("reverify")).status().is_ok_and(|s| s.success())
}

/// Run `keel record reverify`: re-run the configured gate at HEAD and stamp a fresh result on green.
///
/// On all commands exiting 0, appends a fresh `TestResult` to each drift-suspect task. Returns a process
/// exit code (0 = ok/no-op, 1 = gate failed, 2 = config error).
#[must_use]
pub fn run(root: &Path, task_filter: Option<&str>, by: &str) -> i32 {
    let drift: Vec<String> = drift_tasks(root).into_iter().filter(|t| task_filter.is_none_or(|f| f == t)).collect();
    if drift.is_empty() {
        println!("reverify: no drift-suspect deliverable task(s) to re-verify");
        return 0;
    }
    let cfg_path = root.join(".engine").join("contracts").join("reverify.toml");
    let Ok(cfg_text) = std::fs::read_to_string(&cfg_path) else {
        println!("reverify: no reverify command configured (.engine/contracts/reverify.toml absent) — nothing re-run; {} task(s) stay suspect", drift.len());
        return 0;
    };
    let commands = match parse_commands(&cfg_text) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("reverify: reverify.toml parse error: {e}");
            return 2;
        }
    };
    if commands.is_empty() {
        println!("reverify: reverify.toml declares no commands — nothing re-run");
        return 0;
    }
    println!("reverify: re-verifying {} drift task(s) via {} command(s)…", drift.len(), commands.len());
    for cmd in &commands {
        println!("reverify: running `{cmd}`");
        if !run_shell(root, cmd) {
            eprintln!("reverify: `{cmd}` FAILED at HEAD — no fresh result stamped; {} task(s) stay suspect (honest)", drift.len());
            return 1;
        }
    }
    let head = git_capture(root, &["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let date = git_capture(root, &["show", "-s", "--format=%cs", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let mut stamped = 0;
    for task in &drift {
        match find_task_file(root, task) {
            Some(file) => // reverify RE-RAN the declared gate, so unlike a hand-stamped result it can say exactly
                // what produced the verdict (D0232).
                match crate::write::append_result(&file, task, &head, "pass", &date, by, Some("keel record reverify --all-drift (re-ran the declared gate at HEAD)")) {
                Ok(id) => {
                    stamped += 1;
                    println!("reverify: {task} re-verified pass @ {head} ({id})");
                }
                Err(e) => eprintln!("reverify: could not stamp {task}: {e}"),
            },
            None => eprintln!("reverify: could not locate the .tracking file declaring `action {task};`"),
        }
    }
    println!("reverify: gate green — stamped {stamped}/{} drift task(s) fresh at HEAD {head} ({date})", drift.len());
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_commands_reads_the_list() {
        let toml = "commands = [\"cargo test --workspace\", \"cargo clippy -- -D warnings\"]\n";
        assert_eq!(parse_commands(toml).unwrap(), vec!["cargo test --workspace", "cargo clippy -- -D warnings"]);
    }

    #[test]
    fn parse_commands_absent_key_is_empty() {
        // A reverify.toml with no `commands` key => no-op (empty), not an error.
        assert!(parse_commands("# just a comment\n").unwrap().is_empty());
    }

    #[test]
    fn manifest_task_names_extracts_tasks_skipping_comments() {
        let m = "# header comment\ntask: rustS1Lexer | a.rs b.rs\ntask: rustS9writeApi | c.rs\n# task: ignoredComment | x\n";
        let got = manifest_task_names(m);
        assert_eq!(got.len(), 2);
        assert!(got.contains("rustS1Lexer"));
        assert!(got.contains("rustS9writeApi"));
        assert!(!got.contains("ignoredComment"));
    }

    fn prefixes() -> Vec<String> {
        ["keel ", "cargo test", "python scripts/probes/"].iter().map(ToString::to_string).collect()
    }

    /// D0444 / D0388 pair, named before any tree is read. Positive: the command alone,
    /// `keel show control-structure . --svg`, is replayable. Negative: `looked at the picture` (no
    /// prefix), `rm -rf target` (no declared prefix), and the narrative shapes this tree's 283 prefixed
    /// receipts take - a parenthesis, a semicolon, a function word - are not. The one prose receipt the
    /// alphabet alone admitted (the census of 2026-09-11) is held as the case the word list exists for.
    #[test]
    fn a_receipt_is_replayable_only_when_it_is_the_command_alone() {
        let p = prefixes();
        assert!(is_replayable_with("keel show control-structure . --svg", &p), "positive: the command alone");
        assert!(is_replayable_with("  cargo test --release --test judge_set_records_each_item  ", &p), "whitespace around the command is not the command");
        assert!(is_replayable_with("python scripts/probes/guard_dispatch_order.py --ab", &p));
        assert!(is_replayable_with("ci-run id=34582485303 workflow=ci", &p), "D0323's receipt CI re-runs itself");
        assert!(!is_replayable_with("looked at the picture", &p), "negative: prose with no prefix");
        assert!(!is_replayable_with("rm -rf target", &p), "negative: a command under no declared prefix");
        assert!(!is_replayable_with("keel suite 571 passed 0 failed (receipt 199146c397b7)", &p), "a parenthesis is narrative");
        assert!(!is_replayable_with("keel guard judgment-request-quality . PASS; keel validate . clean", &p), "a semicolon is two things said, not one command");
        assert!(!is_replayable_with("keel validate . 762 clean and keel check-engine . clean at 829e0f0 per VERIFIER_RECEIPT 2026-09-11", &p), "the census's one alphabet-clean prose receipt");
        assert!(!is_replayable_with("keel show priority .\nkeel whats-next .", &p), "two lines are two commands");
        assert!(!is_replayable_with("", &p));
        assert!(!is_replayable_with("keel show control-structure . --svg", &[]), "no declared prefix - nothing replays (a project that never adopted the section)");
    }

    #[test]
    fn parse_demo_prefixes_reads_the_section_and_defaults_empty() {
        let toml = "commands = [\"cargo test\"]\n[demo]\nreplayable = [\"keel \", \"cargo test\"]\n";
        assert_eq!(parse_demo_prefixes(toml).unwrap(), vec!["keel ", "cargo test"]);
        assert_eq!(parse_commands(toml).unwrap(), vec!["cargo test"], "the gate list is untouched by the section");
        assert!(parse_demo_prefixes("commands = []\n").unwrap().is_empty(), "no section, no prefixes");
    }

    /// The `--demos` scan reads the LATEST result of each demo Test: a replayable pass is a replay; a
    /// pass whose latest result is a later fail, a prose receipt, a ci-run receipt, an inspect Test,
    /// and a receiptless pass are not.
    #[test]
    fn demo_replays_take_the_latest_replayable_pass_of_each_demo_test() {
        let text = "package S {\n\
            verification aDemo : Test { :>> id = \"e2e00000-0000-4000-8000-0000000000a1\"; :>> method = VerificationMethod::demo; :>> procedureText = \"draw it\"; }\n\
            // RAN: keel show control-structure . --svg\n\
            part aDemoR1 : TestResult { :>> id = \"e2e00000-0000-4000-8000-0000000000a2\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-11\"; :>> judgedBy = \"bot\"; }\n\
            verification bDemo : Test { :>> id = \"e2e00000-0000-4000-8000-0000000000b1\"; :>> method = VerificationMethod::demo; :>> procedureText = \"then broke\"; }\n\
            // RAN: keel show priority .\n\
            part bDemoR1 : TestResult { :>> id = \"e2e00000-0000-4000-8000-0000000000b2\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-11\"; :>> judgedBy = \"bot\"; }\n\
            // RAN: replay of keel show priority . -> exit 2 (keel record reverify --demos at abc1235)\n\
            part bDemoR2 : TestResult { :>> id = \"e2e00000-0000-4000-8000-0000000000b3\"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = \"abc1235\"; :>> judgedAt = \"2026-09-11\"; :>> judgedBy = \"bot\"; }\n\
            verification cDemo : Test { :>> id = \"e2e00000-0000-4000-8000-0000000000c1\"; :>> method = VerificationMethod::demo; :>> procedureText = \"prose\"; }\n\
            // RAN: keel suite 571 passed 0 failed (receipt 199146c397b7)\n\
            part cDemoR1 : TestResult { :>> id = \"e2e00000-0000-4000-8000-0000000000c2\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-11\"; :>> judgedBy = \"bot\"; }\n\
            verification dDemo : Test { :>> id = \"e2e00000-0000-4000-8000-0000000000d1\"; :>> method = VerificationMethod::demo; :>> procedureText = \"ci\"; }\n\
            // RAN: ci-run id=1 workflow=ci\n\
            part dDemoR1 : TestResult { :>> id = \"e2e00000-0000-4000-8000-0000000000d2\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-11\"; :>> judgedBy = \"bot\"; }\n\
            verification eInsp : Test { :>> id = \"e2e00000-0000-4000-8000-0000000000e1\"; :>> method = VerificationMethod::inspect; :>> procedureText = \"eyes\"; }\n\
            // RAN: keel show priority .\n\
            part eInspR1 : TestResult { :>> id = \"e2e00000-0000-4000-8000-0000000000e2\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-11\"; :>> judgedBy = \"bot\"; }\n\
            verification fDemo : Test { :>> id = \"e2e00000-0000-4000-8000-0000000000f1\"; :>> method = VerificationMethod::demo; :>> procedureText = \"no receipt\"; }\n\
            part fDemoR1 : TestResult { :>> id = \"e2e00000-0000-4000-8000-0000000000f2\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-11\"; :>> judgedBy = \"bot\"; }\n\
            }\n";
        let got = demo_replays_in_text(text, &prefixes());
        assert_eq!(got, vec![("aDemo".to_string(), "keel show control-structure . --svg".to_string())], "{got:?}");
        assert!(demo_replays_in_text(text, &[]).is_empty(), "no prefixes, no replays");
    }

    #[test]
    fn a_keel_receipt_replays_through_this_binary_and_others_run_as_written() {
        assert_eq!(replay_command("keel show priority ."), Replay::ThisBinary(vec!["show".into(), "priority".into(), ".".into()]));
        assert_eq!(replay_command("cargo test --lib"), Replay::Shell("cargo test --lib".into()));
        // a shell `exit 3` replays as exit 3 (the `keel ` path is exercised end to end by the
        // reverify_demos_replays_each_receipt integration test - here current_exe is the test harness)
        let here = std::env::current_dir().expect("cwd");
        let exit3 = if cfg!(windows) { "cmd /C exit 3" } else { "sh -c \"exit 3\"" };
        assert_eq!(replay_status(&here, &Replay::Shell(exit3.into())), Some(3));
    }
}
