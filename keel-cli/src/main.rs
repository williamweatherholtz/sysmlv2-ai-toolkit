//! `keel` — CLI entry point.
//!
//! Subcommands:
//!   `validate [ROOT]`         — semantic-validate all `.tracking/` files
//!   `check FILE...`           — parse-check one or more `.sysml` files
//!   `orient [ROOT]`           — print orient state (cursor + ready/done/outstanding) as JSON
//!   `whats-next [ROOT]`       — print ready task names, one per line
//!   `advance <sprint> [--to G]` — process cursor: the sprint's current ceremony step; `--to` is
//!                               refused until every earlier step's verify-Test passes (D0209 clause 3)
//!   `append-result [FLAGS]`   — append a `TestResult` to a tracking file
//!   `append-gate-result [FLAGS]` — append a `TestResult` for a ceremony gate (`verification`)
//!   `add-task [FLAGS]`        — add a task + `DoD` verification to an action def
//!   `coverage [ROOT]`         — assurance-coverage view (D0079 C): Need/Requirement/Decision evidence
//!   `critique-coverage [ROOT]` — per-element x required-lens critique coverage (D0080)
//!   `critique-policy [ROOT]`   — the active declared critique policy: required lenses per type (D0097)
//!   `concern-coverage [ROOT]` — which declared viewpoint concerns are served vs planned (D0057)
//!   `dispositions [ROOT]`     — >= Medium findings + their typed disposition verdict (D0092)
//!   `sitting-coverage [ROOT]` — which delivery sprints are covered by a per-sitting review (D0049)
//!   `assured [ROOT]`           — composite assurance-readiness verdict + blockers (D0079 c)
//!   `decisions [ROOT]`         — load-bearing decisions ranked by dependence + antiquation flags
//!   `diagram [ROOT]`           — comprehensive interactive traceability diagram (HTML; computed #View)
//!   `init DIR`                 — scaffold the engine into a new project (D0093 cold start)
//!   `serve [--port N] [ROOT]`  — the interactive console: localhost read dashboard (D0094 m1)
#![forbid(unsafe_code)]
#![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]
// D0074 fail-loud: authority-bearing CLI code has no silent failure paths.
// (clippy::indexing_slicing deferred to M0b with the parser cleanup — see rustFailLoudLints.)
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::todo,
    clippy::unimplemented
)]

use std::{path::{Path, PathBuf}, process};

use include_dir::{include_dir, Dir};
use keel_cli::{check_files, collect_sysml, validate_root};
use keel_cli::orient;
use keel_cli::write as w;

// ── engine scaffold payload (D0093 `init`): the reusable engine tree + operating manual, embedded at
//    compile time so `keel init` is self-contained (no external fetch — the cytoscape precedent). ──
static ENGINE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../.engine");
// A DOWNSTREAM CLAUDE.md template (issue057): a fresh project is TRACKED BY keel, not keel itself.
// The self-build repo's own CLAUDE.md (about building the engine) is NEVER shipped to init'd projects.
const CLAUDE_MD: &str = include_str!("../assets/claude-md-template.md");
const TRACKING_STARTER: &str = "# .tracking/ — your project's instance data\n\nThis directory holds THIS project's authored facts (needs, requirements, work items, issues,\ndecisions, test results) — the per-project INSTANCE. The reusable engine lives in `.engine/`.\n\nGetting started: run the `introduction` skill (guided onboarding), or author your first `Need`\nfollowing `.engine/docs/tracking-template.sysml`. State is COMPUTED — run `keel orient .` to\nsee where things stand. The engine's design rationale is read-only in `.engine/reference/decisions/`;\nyour project authors its OWN decisions fresh in `.engine/decisions/`.\n";
/// A fresh project's deliverable-suspicion manifest is EMPTY — the shipped one lists the ENGINE's own
/// deliverable tasks (instance-specific), which would fail manifest-coverage on a new project (D0093
/// engine/instance boundary). The new project adds entries as it builds source-dependent verifications.
const STARTER_MANIFEST: &str = "# deliverable-manifest.txt — declares which verification tasks depend on which DELIVERABLE SOURCE\n# files (D0050), so `keel suspect` flags a task suspect when its source changed since it was\n# verified. One entry per line:  task: <taskName> | <relpath> <relpath> ...\n# Empty for a new project — add an entry when you have a deliverable-source-dependent verification.\n";
/// A starter actor registry scaffolded into a fresh project (`.tracking/actors.sysml`). Without it
/// the newcomer's FIRST recorded fact (any `createdBy`/`judgedBy`) fails the actors guard (D0037) —
/// there'd be no `ProjectActors` to reference. Ships placeholder actors (a human + the AI) the newcomer
/// edits to their real identities; the declared part name is the id that `createdBy`/`judgedBy` reference.
const STARTER_ACTORS: &str = "// ProjectActors — this project's actor registry (INSTANCE data). EDIT to your real actors.\n// The declared part name is the id that createdBy/judgedBy reference (enforced by `keel guard actors`).\npackage ProjectActors {\n    private import EngineElement::*;\n\n    part you : Person { :>> name = \"Your Name\"; :>> email = \"you@example.com\"; }\n    part ai : Actor { :>> name = \"AI assistant\"; :>> kind = ActorKind::ai; }\n}\n";
/// A RUST-ONLY pre-commit gate scaffolded into a fresh project (`.githooks/pre-commit`). Runs
/// `keel validate` + `keel guard` — NO conda/JVM kernel (D0048: the Rust path is the authority).
/// Enabled by the user with `git config core.hooksPath .githooks` (printed in the init Next steps).
///
/// FAILS LOUD without the binary (K2/P0.3, D0174): the previous version skipped with a printed
/// notice, so an uninstalled downstream machine committed ungated while looking gated — the exact
/// silent-pass class the proposal's §1.1 recorded. The remedy line names the documented install
/// path (D0175's fence). POSIX sh.
const PRECOMMIT_HOOK: &str = "#!/bin/sh\n# keel pre-commit gate (Rust-only; no JVM kernel) — scaffolded by `keel init` (D0048/D0093/D0174).\n# Enable: git config core.hooksPath .githooks   |   bypass once: SKIP_KEEL=1 git commit ...\n[ \"$SKIP_KEEL\" = \"1\" ] && { echo 'pre-commit: SKIP_KEEL=1 — keel gate skipped'; exit 0; }\n# BINARY RESOLUTION, pinned-first (D0230). A project that wants its gate DECOUPLED from whatever\n# keel happens to be on PATH drops a RELEASED binary at .keel/bin/keel - machine-local, so each\n# contributor installs their own - and it wins over PATH. That is the whole pin: no script, no\n# wrapper, nothing to keep in sync. Without it a sibling working tree on the same machine silently\n# decides this project's gate, which is how one project's gate came to run an unreleased build.\nKEEL=\"${KEEL_BIN:-}\"\n[ -z \"$KEEL\" ] && [ -x .keel/bin/keel ] && KEEL=./.keel/bin/keel\n[ -z \"$KEEL\" ] && [ -x .keel/bin/keel.exe ] && KEEL=./.keel/bin/keel.exe\nKEEL=\"${KEEL:-keel}\"\ncommand -v \"$KEEL\" >/dev/null 2>&1 || { echo \"pre-commit: keel binary NOT FOUND — commit BLOCKED (K2: an absent gate must not pass silently).\"; echo \"pre-commit: install keel from https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/releases and put it on PATH (or set KEEL_BIN).\"; exit 1; }\n# THE GATE IS WORKSPACE-SCOPED, ALWAYS (D0234/issue278). git allows ONE core.hooksPath per\n# repository, so a hook inside a project directory can never gate a sibling project - which is\n# why this hook is installed at the REPOSITORY ROOT. It used to branch on `[ ! -d .engine ]` to\n# decide whether it was at a workspace root; that test is wrong for the commonest layout, a repo\n# whose root is itself a project with peers beside it, where .engine exists and every peer\n# therefore rode out UNGATED. `keel gate --workspace` gates every project the commit touches and\n# is identical to the old single-project path when the repo holds exactly one project.\necho 'pre-commit: keel gate --workspace (every project this commit touches)'\n\"$KEEL\" gate --workspace . || { echo 'pre-commit: keel gate FAILED — commit aborted'; exit 1; }\n";

/// Scaffolded `.gitignore`. Machine-local state only — nothing here is a build artifact of the
/// project, it is state that is TRUE OF ONE CLONE and false of every other.
const GITIGNORE: &str = "# keel machine-local state — never commit these.
#
# .keel/actor is THIS MACHINE's identity binding (D0129). Committing it hands your identity to every
# other clone: two contributors who both commit one end up conflicting over who they are, and a
# merge that picks a side silently makes one of them write as the other.
.keel/

# `keel serve` per-clone preferences.
.keel-serve.json

# Generated views/reports — regenerable from the model, so they are outputs, not facts.
*.keel.html
";

// ── repo-root discovery ───────────────────────────────────────────────────────

fn find_repo_root() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join(".engine").is_dir() {
            return Some(dir);
        }
        // STOP AT THE REPOSITORY BOUNDARY (issue281). This walk had none while workspace discovery
        // did, so standing in a directory nested under an unrelated keel project, `keel validate`
        // with no argument walked OUT of the repository and validated the OUTER repo's project —
        // reporting it clean. A command that answers about a repository the caller is not in is worse
        // than one that refuses: the answer looks right.
        if dir.join(".git").exists() {
            return None;
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Drop each named flag AND ITS VALUE, leaving only true positionals for [`root_arg`].
///
/// THE CLASS THIS ENDS, third instance in one session: `root_arg` takes the first bare token as ROOT
/// and cannot know which flags consume the token after them, so every command that adds a
/// value-taking flag re-creates the bug. `keel github-decider --root X` read X as a login,
/// `keel recall --prompt -` read `-` as a root, and `keel why t --budget 1500` read 1500 as a root —
/// each silently answering about the wrong thing until the issue281 project precondition started
/// refusing outright, which is the only reason the last two were visible at all.
///
/// It is NOT fixed inside `root_arg` because a known flag's following positional is legitimately the
/// root for existing callers (`--explain /r`), so the distinction has to be stated by the caller that
/// knows it.
fn without_flag_values(args: &[String], value_flags: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    let mut skip = false;
    for a in args {
        if skip {
            skip = false;
            continue;
        }
        if let Some(name) = a.strip_prefix("--") {
            if value_flags.contains(&name) {
                skip = true;
                continue;
            }
        }
        out.push(a.clone());
    }
    out
}

fn root_arg(args: &[String], usage: &str, known: &[&str], positionals: usize) -> Result<PathBuf, i32> {
    let mut positional: Vec<&String> = Vec::new();
    for a in args {
        if let Some(name) = a.strip_prefix("--") {
            // A flag's VALUE is consumed by the caller's own parse; only the flag NAME is judged here.
            if !known.contains(&name) {
                eprintln!("error: unknown flag `{a}`");
                eprintln!("usage: {usage}");
                return Err(2);
            }
        } else {
            positional.push(a);
        }
    }
    if let Some(p) = positional.get(positionals) {
        let root = PathBuf::from(p.as_str());
        keel_cli::workspace::require_project(&root, usage)?;
        return Ok(root);
    }
    let root = find_repo_root().ok_or_else(|| {
        eprintln!("error: no .engine/ directory found from the current directory upward");
        eprintln!("  (the search stops at the repository boundary — it will not answer for another repo).");
        eprintln!("usage: {usage}");
        2
    })?;
    keel_cli::workspace::require_project(&root, usage)?;
    Ok(root)
}

// ── subcommands ───────────────────────────────────────────────────────────────


/// D0190: the engine-version parity warning. The DECLARED version (engine-version.toml, stamped by
/// init and re-stamped by migrate) answers one question only: which binary's checks is this on-disk
/// engine defined against? A mismatch WARNS and names `keel migrate` - never blocks, because skew is
/// not dishonest state (D0098), and never gates or skips anything (migrate derives its vintage from
/// the TREE, per its own no-stamp rule - this declaration exists for the warning, the two designs
/// answer different questions). Absent declaration = pre-D0190 project, silent (forward-only, issue068).
fn engine_version_skew(root: &Path) -> Option<String> {
    let text = std::fs::read_to_string(root.join(".engine").join("contracts").join("engine-version.toml")).ok()?;
    let declared = text
        .lines()
        .find_map(|l| l.trim().strip_prefix("engine").map(|r| r.trim_start_matches(['=', ' ']).trim_matches('"').to_string()))
        .filter(|v| !v.is_empty())?;
    let binary = env!("CARGO_PKG_VERSION");
    if declared == binary {
        return None;
    }
    Some(format!(
        "[keel] engine-version SKEW: this binary is {binary} but this project PINS {declared} (engine-version.toml).          The pin is BINDING (D0251): writes and gates REFUSE under skew; reads warn and proceed. Run the pinned          version, or `keel migrate` to bring the tree to this one (it re-stamps the pin)."
    ))
}

fn cmd_validate(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel validate [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };

    // REFUSE A NON-PROJECT (issue269/D0234). Pointed at a directory with no `.engine/`, validate used
    // to print "0 tracking file(s) validated clean" and exit 0 — a vacuous pass on the FIRST line of
    // every gate. A hook placed at a repo root holding several projects would therefore gate NOTHING
    // while reporting success. `keel guard` already fails in that position; the two halves of the gate
    // disagreed about whether an absent project is a clean tree or a usage error. It is a usage error.
    //
    // A real project holding zero tracking files still exits 0: that is a true statement about a
    // project that exists.
    if !keel_cli::workspace::is_project(&root) {
        eprintln!("error: {} is not a keel project — it has no .engine/ (and .tracking/) directory.", root.display());
        let ws = keel_cli::workspace::discover(&root);
        if ws.is_multi() {
            eprintln!("  This repository holds {} projects. validate takes ONE project root:", ws.projects.len());
            for p in &ws.projects {
                eprintln!("    keel validate {}", ws.label(p));
            }
            // issue278: this used to advise `keel hook pre-commit`, which is not a hook event -
            // at a workspace root it prints nothing and exits 0, so anyone who wired it in
            // installed a gate that passes while checking nothing. Name the real command.
            eprintln!("  To gate every project in this repository, run `keel gate --workspace .`");
            eprintln!("  (that is what the scaffolded .githooks/pre-commit at the REPO ROOT runs).");
        } else {
            eprintln!("  Validating a directory that is not a project would report a clean tree over zero");
            eprintln!("  files, which is how a gate passes while checking nothing (issue269).");
        }
        return 2;
    }
    if let Some(w) = engine_version_skew(&root) {
        // D0251: for a GATE surface the skew REFUSES rather than warns — a verdict from an engine
        // the project did not declare is not the project's verdict. `orient` still answers (reads
        // warn), and `migrate` is the repair path.
        eprintln!("{w}");
        eprintln!("validate REFUSED under engine-version skew (D0251). Run the pinned version, or `keel migrate`.");
        return 2;
    }
    let report = validate_root(&root);

    for (path, diag) in &report.diagnostics {
        println!("ERROR: {}:{} — {}", path.display(), diag.line, diag.message);
        if let Some(hint) = &diag.suggestion {
            println!("       hint: {hint}");
        }
    }
    for err in &report.errors {
        println!("FAIL:  {} — {}", err.file.display(), err.message);
    }

    if report.is_clean() {
        println!("{} tracking file(s) validated clean.", report.validated);
        0
    } else {
        eprintln!(
            "{} tracking file(s) validated — {} parse error(s), {} semantic diagnostic(s).",
            report.validated,
            report.errors.len(),
            report.diagnostics.len()
        );
        1
    }
}

/// `keel check-engine [ROOT]` (D0112 phase 2, issue067) — semantically validate the `.engine` INSTANCE
/// files (decisions/processes/views + registry + template) against the schema, KERNEL-FREE — the Rust
/// backstop for the `unresolved` reference class the JVM `validate_instances.py` used to be the sole
/// source of.
/// `keel hook <stop|post-edit>` (D0134) — the in-loop gates, IN THE BINARY.
///
/// These were python wrappers. The CHECKING was always Rust; python only parsed the hook's stdin
/// JSON and emitted the response — pure glue, bought with a SECOND RUNTIME DEPENDENCY on every
/// contributor's machine. That is the issue076 class inside the governance layer again: where python
/// is absent the gate silently does not run, and D0129 puts five mixed-OS machines in scope. Since
/// `keel` is already a hard requirement for these gates to mean anything, and `serde_json` is already
/// a dependency, the glue belongs here and the dependency goes away.
///
/// Reads the hook payload on stdin, writes the hook protocol on stdout, and NEVER fails a turn:
/// any internal error exits 0 silently.
fn cmd_hook(args: &[String]) -> i32 {
    use std::io::Read as _;
    // issue179: an unrecognised event already errors, but a flag should be told it is a flag.
    if let Some(f) = args.first().filter(|a| a.starts_with('-')) {
        eprintln!("error: `{f}` looks like a flag, not a hook event (issue179).");
        return 2;
    }
    let Some(event) = args.first().map(String::as_str) else {
        eprintln!("usage: keel hook <stop|post-edit|pre-bash|user-prompt>");
        return 2;
    };
    // BOUNDED LIFETIME (issue180b). `read_to_string` on stdin blocks until EOF, and if the parent goes
    // away WITHOUT closing stdin the hook waits forever - holding a Windows file lock on
    // `target/release/keel.exe`, so every later `cargo build` fails with `Access is denied`. That
    // happened three times in one turn, and the error names cargo and a file permission, so it reads as
    // a toolchain problem rather than as the hook. An in-loop gate that can wedge the build it gates is
    // the worst failure mode available to it.
    //
    // The watchdog exits 0, never nonzero: D0134 says a hook NEVER fails a turn, so a hook that gave up
    // waiting must look exactly like a hook with nothing to say — to the HARNESS. To the ledger it
    // must not (panel R1, robotics finding 4 / K2): a wedged hook that vanished without a line was
    // the one fail-SILENT branch inside the layer the enforcement-report reads, invisible to D0180's
    // single instrumentation path. The watchdog now appends its own line before exiting.
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(HOOK_DEADLINE_SECS));
        if let Some(root) = find_repo_root() {
            ledger_emit(&root, "", "hook-watchdog-timeout", 0, u128::from(HOOK_DEADLINE_SECS) * 1000);
        }
        std::process::exit(0);
    });
    let mut raw = String::new();
    let _ = std::io::stdin().read_to_string(&mut raw);
    let payload: serde_json::Value = serde_json::from_str(&raw).unwrap_or(serde_json::Value::Null);
    let root = find_repo_root().unwrap_or_else(|| PathBuf::from("."));
    if !root.join(".tracking").is_dir() {
        return 0; // not a keel project -> silent no-op, correctly
    }

    // Fire-ledger + subagent baseline (D0174/P0.1, D0180): every fire leaves one machine-local
    // JSONL line keyed by session id and event — the SINGLE instrumentation path the
    // hooks-actually-fired checks read. A session's FIRST fire also stores the tree fingerprint,
    // which is the SubagentStop baseline (P0.6).
    let session = payload.get("session_id").and_then(serde_json::Value::as_str).unwrap_or("").to_string();
    // Never write the baseline from the subagent-stop event itself: a subagent whose FIRST fire is
    // its own stop would baseline against the post-work tree and silently skip the gate — found by
    // running the branch, not by reading it.
    if !session.is_empty() && event != "subagent-stop" {
        let bl = root.join(".keel").join("metrics").join(format!("baseline-{session}.fp"));
        if !bl.exists() {
            let _ = std::fs::create_dir_all(root.join(".keel").join("metrics"));
            let _ = std::fs::write(&bl, keel_cli::fingerprint::of(&root).to_string());
        }
    }
    let started = std::time::Instant::now();
    let code = match event {
        "post-edit" => hook_post_edit(&payload, &root),
        "stop" => hook_stop(&payload, &root),
        "user-prompt" => hook_user_prompt(&root, &payload),
        "pre-bash" => hook_pre_bash(&payload, &root, &session),
        "pre-write" => hook_pre_write(&payload, &root),
        "subagent-stop" => hook_subagent_stop(&payload, &root, &session),
        other => {
            eprintln!("unknown hook event '{other}' (expected stop|post-edit|pre-bash|user-prompt|pre-write|subagent-stop)");
            2
        }
    };
    ledger_emit(&root, &session, event, code, started.elapsed().as_millis());
    code
}

/// Append one fire-ledger line (machine-local, `.keel/metrics/hooks.jsonl`, gitignored class).
/// Best-effort by design: the ledger is evidence infrastructure, and a full disk must not turn an
/// advisory hook into a blocker — but a write failure is still printed, never swallowed (K2).
fn ledger_emit(root: &Path, session: &str, event: &str, exit: i32, ms: u128) {
    use std::io::Write as _;
    let dir = root.join(".keel").join("metrics");
    if std::fs::create_dir_all(&dir).is_err() {
        eprintln!("[keel] fire-ledger unavailable: cannot create {}", dir.display());
        return;
    }
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let decision = if exit == 0 { "allow" } else { "block" };
    let line = format!(
        "{}\n",
        serde_json::json!({"ts": ts, "session": session, "event": event, "decision": decision, "exit": exit, "ms": u64::try_from(ms).unwrap_or(u64::MAX)})
    );
    let appended = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("hooks.jsonl"))
        .and_then(|mut f| f.write_all(line.as_bytes()));
    if let Err(e) = appended {
        eprintln!("[keel] fire-ledger write failed: {e}");
    }
}


/// Minimal raw-HTTP call to the LOCAL console (dependency-free; localhost only). Returns the body.
fn console_http(method: &str, path: &str, body: Option<&str>) -> Option<String> {
    use std::io::{Read as _, Write as _};
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], CONSOLE_PORT));
    let mut s = std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(500)).ok()?;
    let _ = s.set_read_timeout(Some(std::time::Duration::from_millis(1500)));
    let payload = body.unwrap_or("");
    let req = format!(
        "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:{CONSOLE_PORT}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{payload}",
        payload.len()
    );
    s.write_all(req.as_bytes()).ok()?;
    let mut buf = String::new();
    let _ = s.read_to_string(&mut buf);
    buf.split("\r\n\r\n").nth(1).map(str::to_string)
}

/// The D0182 headless-ask mapping, hook side: inside a LAUNCHED RUN (`KEEL_RUN_ID` set) there is no
/// harness prompt, so an ask-tier write maps to the console approve queue - the ask-pending entry
/// is registered BEFORE any wait, the wait is bounded WELL under the hook deadline, and expiry or
/// an absent console maps to DENY plus a queued obligation (charter note 2: never let the harness
/// timeout decide).
fn headless_ask(root: &Path, path: &str, session: &str, run_id: &str) -> bool {
    let ask_body = serde_json::json!({"path": path, "session": session}).to_string();
    let ask_id = console_http("POST", "/api/run/ask", Some(&ask_body))
        .and_then(|b| serde_json::from_str::<serde_json::Value>(&b).ok())
        .and_then(|v| v.get("id").and_then(serde_json::Value::as_str).map(str::to_string));
    let mut denied_by: Option<String> = None;
    if let Some(id) = ask_id {
        // ~60s bounded wait (the hook deadline is 100s+; margin per charter note 2)
        for _ in 0..30 {
            std::thread::sleep(std::time::Duration::from_secs(2));
            let v = console_http("GET", &format!("/api/run/answer?id={id}"), None)
                .and_then(|b| serde_json::from_str::<serde_json::Value>(&b).ok());
            let answer = v.as_ref().and_then(|v| v.get("answer").and_then(serde_json::Value::as_str).map(str::to_string));
            let by = v.as_ref().and_then(|v| v.get("by").and_then(serde_json::Value::as_str)).unwrap_or("").to_string();
            match answer.as_deref() {
                Some("allow") => {
                    // issue200/K7: the ALLOW is a human authorization of a protected write and must
                    // leave a record naming the human, exactly as the override tier's consumption
                    // does. Recorded under the APPROVER — theirs is the judgment being recorded.
                    let approver = if by.is_empty() { "unknown".to_string() } else { by };
                    let _ = keel_cli::write::record_obligation(
                        root,
                        "ask-allow",
                        &format!("headless run write ALLOWED by {approver}: {path}"),
                        &format!("{approver} allowed a launched run's ({run_id}) ask-tier write to {path} from the console approve queue (D0182 headless mapping; issue200/K7 - an authorization is a recorded fact, not an evaporating click). Discharge: review the landed write, then triage with a #Resolves edge."),
                        &approver,
                    );
                    return true;
                }
                Some("deny") => {
                    if !by.is_empty() {
                        denied_by = Some(by);
                    }
                    break;
                }
                _ => {}
            }
        }
    }
    // deny + queued obligation: a HUMAN's deny is recorded as their judgment (issue200); expiry or
    // an absent console stays attributed to the run's own actor, because nobody judged anything.
    let (actor, why) = denied_by.as_ref().map_or_else(
        || (keel_cli::actor::resolve(root, None), "no human approval arrived within the bounded wait, so it was DENIED".to_string()),
        |by| (Ok(by.clone()), format!("{by} DENIED it from the console approve queue")),
    );
    if let Ok(actor) = actor {
        let _ = keel_cli::write::record_obligation(
            root,
            "headless-ask",
            &format!("headless run write denied pending review: {path}"),
            &format!("A launched run ({run_id}) requested an ask-tier write to {path}; {why} and queued (D0182 headless mapping). Discharge: review whether the write should happen, perform or decline it, and triage with a #Resolves edge."),
            &actor,
        );
    }
    false
}

/// The declared adoption profile: `strict`, `guided`, or `undeclared` (pre-adoption trees).
fn adoption_profile(root: &Path) -> &'static str {
    match std::fs::read_to_string(root.join(".engine").join("contracts").join("adoption-profile.toml")) {
        Ok(t) if t.lines().any(|l| l.trim().starts_with("profile") && l.contains("strict")) => "strict",
        Ok(t) if t.lines().any(|l| l.trim().starts_with("profile") && l.contains("guided")) => "guided",
        _ => "undeclared",
    }
}

/// A pending override unlock (D0176 tier 3): single-use, target-path-bound, expiring. The consuming
/// session is recorded in the obligation fact — the CLI cannot know the harness session a priori, so
/// session-binding is realized as short expiry + single use + consumed-by-session recorded.
const OVERRIDE_TTL_SECS: u64 = 900;

fn override_path(root: &Path) -> PathBuf {
    root.join(".keel").join("override.json")
}

/// Consume a matching unlock: returns the reason when `path` is covered. Deletes the unlock (single
/// use) and records the tracked obligation naming the path ACTUALLY written (K7); on a failed
/// tracked write, degrades to a local ledger entry with a sync obligation (charter note 1).
fn consume_override(root: &Path, written_path: &str, session: &str) -> Option<String> {
    let op = override_path(root);
    let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&op).ok()?).ok()?;
    let target = v.get("path").and_then(serde_json::Value::as_str)?.replace('\\', "/");
    let created = v.get("ts").and_then(serde_json::Value::as_u64).unwrap_or(0);
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    if now.saturating_sub(created) > OVERRIDE_TTL_SECS {
        let _ = std::fs::remove_file(&op);
        eprintln!("[keel] override unlock EXPIRED ({OVERRIDE_TTL_SECS}s) — run `keel override` again if still needed");
        return None;
    }
    if !written_path.contains(&target) && !target.contains(written_path) {
        return None; // path-bound: an unlock for one file covers no other
    }
    let reason = v.get("reason").and_then(serde_json::Value::as_str).unwrap_or("(no reason recorded)").to_string();
    let actor = v.get("actor").and_then(serde_json::Value::as_str).unwrap_or("unknown").to_string();
    let _ = std::fs::remove_file(&op); // single-use
    let title = format!("override used: direct write to {written_path}");
    let desc = format!(
        "A recorded override unlocked a direct write (D0176 tier 3). Path actually written: {written_path}. Reason given: {reason}. Session: {session}. Discharge: a human reviews the write and triages this obligation with a #Resolves edge."
    );
    match keel_cli::write::record_obligation(root, "override", &title, &desc, &actor) {
        Ok(p) => {
            // issue207/D0193 (srK14): the NORMAL consumption is counted too - without this line the
            // report's override counter saw only the UNSYNCED failure path and structurally
            // under-read override pressure in the very evidence promotion reviews must cite.
            ledger_emit(root, session, "override-consumed", 0, 0);
            eprintln!("[keel] override consumed — obligation recorded: {}", p.display());
        }
        Err(e) => {
            // The tracked write failed (possibly BECAUSE the target is the corrupted file being
            // repaired) — local ledger + sync obligation, never a silent unlock (K7).
            ledger_emit(root, session, "override-obligation-UNSYNCED", 1, 0);
            eprintln!("[keel] override consumed but the tracked obligation could not be written ({e}) — a local ledger entry holds it; SYNC OBLIGATION: record it with `keel record issue` once the tree is writable");
        }
    }
    Some(reason)
}

/// `PreToolUse` on Write|Edit over `.tracking`/`.engine` fact surfaces — the D0176 three-tier model,
/// invoked by the scaffolded pure-shell test when the binary is present.
///
///   tier 1 — API-owned surfaces: hard deny ABSENT A RECORDED OVERRIDE, refusal naming the command;
///   tier 2 — other `.tracking` writes: `permissionDecision: "ask"` (the harness prompt is a human
///            channel; the headless mapping is D0182's, exercised by the P5 launcher);
///   tier 3 — the recorded override reaches every tier (a corrupted API-owned file is repairable).
///
/// Profile-aware (P0.4/D0176): `strict` blocks as above; `guided`/`undeclared` is ADVISORY-FIRST —
/// the same detection runs, the fire-ledger accrues the D0180 evidence, and promotion to blocking is
/// a recorded decision citing it (K14).
fn hook_pre_write(payload: &serde_json::Value, root: &Path) -> i32 {
    let path = payload
        .pointer("/tool_input/file_path")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .replace('\\', "/");
    let session = payload.get("session_id").and_then(serde_json::Value::as_str).unwrap_or("");
    // Control plane first (D0179/K7): weakening or redirecting enforcement is approval-gated and,
    // under strict, leaves an orient-visible record — never a quiet config edit.
    if keel_cli::claude_surface::CONTROL_PLANE_PATHS.iter().any(|p| path.contains(p)) {
        if adoption_profile(root) == "strict" {
            if let Ok(actor) = keel_cli::actor::resolve(root, None) {
                let _ = keel_cli::write::record_obligation(
                    root,
                    "control-plane",
                    &format!("control-plane write requested: {path}"),
                    &format!("A Write|Edit touched enforcement configuration ({path}), session {session} (D0179/K7). The harness asked the human; this fact records that the control plane moved. Discharge: review the diff and triage with a #Resolves edge."),
                    &actor,
                );
            }
            println!(
                "{}",
                serde_json::json!({"hookSpecificOutput": {"hookEventName": "PreToolUse", "permissionDecision": "ask",
                    "permissionDecisionReason": format!("[keel] {path} is CONTROL PLANE (D0179/K7) - approve only if you intend to change enforcement; the write is recorded")}})
            );
        } else {
            println!("[keel] control-plane write: {path} (D0179 - ask-tier under strict; the fire-ledger records this fire)");
        }
        return 0;
    }
    if !(path.contains(".tracking/") || path.contains(".engine/decisions/")) {
        return 0;
    }
    if let Some(reason) = consume_override(root, &path, session) {
        println!("[keel] override active for this write (reason: {reason}) - recorded, single-use");
        return 0;
    }
    let tier1 = keel_cli::claude_surface::PROTECTED_PATHS.iter().find(|(p, _)| path.contains(p));
    let profile = adoption_profile(root);
    match (tier1, profile) {
        (Some((surface, sanctioned)), "strict") => {
            let reason = format!(
                "[keel] {surface} is an API-owned fact surface (D0176 tier 1). Use the sanctioned path: {sanctioned} - or, for what the API cannot express, `keel override {path} --reason \"...\"` (single-use, recorded, reviewed)."
            );
            println!(
                "{}",
                serde_json::json!({"hookSpecificOutput": {"hookEventName": "PreToolUse", "permissionDecision": "deny", "permissionDecisionReason": reason}})
            );
        }
        (None, "strict") => {
            if let Ok(run_id) = std::env::var("KEEL_RUN_ID") {
                // Headless launched run: no prompt exists - console proxy, else deny + obligation.
                if headless_ask(root, &path, session, &run_id) {
                    println!("[keel] headless ask APPROVED from the console queue - write allowed");
                } else {
                    println!(
                        "{}",
                        serde_json::json!({"hookSpecificOutput": {"hookEventName": "PreToolUse", "permissionDecision": "deny",
                            "permissionDecisionReason": format!("[keel] headless run: the ask-tier write to {path} was not approved within the bounded wait - DENIED and queued as an obligation (D0182)")}})
                    );
                }
                return 0;
            }
            println!(
                "{}",
                serde_json::json!({"hookSpecificOutput": {"hookEventName": "PreToolUse", "permissionDecision": "ask",
                    "permissionDecisionReason": format!("[keel] direct write to {path} (D0176 tier 2) - approve, or route through the write API")}})
            );
        }
        (Some((surface, sanctioned)), _) => {
            println!("[keel advisory] {surface} is an API-owned fact surface (D0176 tier 1 in strict) - prefer: {sanctioned}");
        }
        (None, _) => {} // tier 2 stays silent in advisory profiles: a comment on every ordinary edit is noise (issue094 rule)
    }
    0
}

/// `SubagentStop` (D0174/P0.6): gate ONLY when the tree changed during the subagent's lifetime.
/// Baseline = the fingerprint stored at the session's first hook fire; no baseline → a
/// `systemMessage` advisory, never a block (a read-only subagent pays nothing).
fn hook_subagent_stop(payload: &serde_json::Value, root: &Path, session: &str) -> i32 {
    let bl = root.join(".keel").join("metrics").join(format!("baseline-{session}.fp"));
    let Ok(baseline) = std::fs::read_to_string(&bl) else {
        println!(
            "{}",
            serde_json::json!({"systemMessage": "[keel] subagent tree not gated: no baseline fingerprint for this session (first hook fire was this one)"})
        );
        return 0;
    };
    if baseline.trim() == keel_cli::fingerprint::of(root).to_string() {
        return 0; // wrote nothing — pays nothing
    }
    // The tree changed under this subagent: same gate as the turn boundary.
    hook_stop(payload, root)
}

/// `PreToolUse` on Bash: advise on host/shell adaptation before the command runs (issue094).
///
/// ADVISORY ONLY — always returns 0. A blocking heuristic over shell commands is the issue076/
/// issue081 dynamic where an over-strict gate trains its actor to disable it, and the checks here
/// cannot be exact: whether an MSYS path is wrong depends on what reads it.
///
/// Prints nothing when there is nothing to say, which is the common case and the point: a hook that
/// comments on ordinary commands becomes noise the reader skips, at which point it looks like
/// coverage while providing none.
/// Whitespace tokenization with single/double-quote awareness — argv-level, so a commit MESSAGE
/// describing `--no-verify` never matches (D0176/P1.2: no raw-string regex over commands).
fn bash_tokens(cmd: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    for c in cmd.chars() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (None, '\'' | '"') => quote = Some(c),
            (None, c) if c.is_whitespace() => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            (_, c) => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// The D0176/D0178 Bash matcher verdict for one command.
enum BashVerdict {
    Clean,
    /// Unambiguous control-bypass pattern — blocking in strict, advisory otherwise.
    Block(String),
    /// Human-judgment / actor-identity operation — never keel-exempt (K6); ask in strict.
    Ask(String),
}

/// Classify a Bash command per the accepted tiering. UNQUOTED tokens only for operator detection —
/// tokenization strips quotes, so `>` inside a message is not a redirect... it IS stripped into the
/// token stream; operators are matched as standalone tokens or known flags, never substrings of
/// prose (the tokenizer keeps quoted text glued to its word, so `"a > b"` is one token, not three).
fn bash_classify(root: &Path, cmd: &str) -> BashVerdict {
    let toks = bash_tokens(cmd);
    let has_tok = |t: &str| toks.iter().any(|x| x == t);
    let tracking_target =
        toks.iter().any(|x| {
            let tracked = (x.contains(".tracking/") || x.contains(".tracking\\")) && std::path::Path::new(x).extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml"));
            tracked || x.contains(".claude/settings") || x.contains(".claude\\settings")
        });
    // Unambiguous operator-level bypass patterns (issue116 vector included).
    if (has_tok(">") || has_tok(">>") || has_tok("tee")) && tracking_target {
        return BashVerdict::Block("redirection into a fact surface - use the keel write API".to_string());
    }
    if has_tok("sed") && has_tok("-i") && tracking_target {
        return BashVerdict::Block("in-place sed over a fact surface - use the keel write API".to_string());
    }
    if toks.windows(2).any(|w| matches!(w, [a, b] if a.ends_with("git") && b == "commit")) && has_tok("--no-verify") {
        return BashVerdict::Block("git commit --no-verify skips the commit gate".to_string());
    }
    if toks.iter().any(|x| x == "SKIP_VALIDATE=1" || x == "SKIP_KEEL=1") {
        return BashVerdict::Block("SKIP_VALIDATE/SKIP_KEEL bypasses the gate - fix the red instead".to_string());
    }
    if toks.windows(3).any(|w| matches!(w, [a, b, c] if a.ends_with("git") && b == "config" && c.contains("core.hooksPath"))) {
        return BashVerdict::Block("git config core.hooksPath is the issue116 control-bypass vector (K7)".to_string());
    }
    // The keel carve-out (D0178): keel-invoking commands are exempt EXCEPT the derived
    // human-judgment/actor-identity set, which is never exempt.
    let keel_idx = toks.iter().position(|t| {
        let base = t.rsplit(['/', '\\']).next().unwrap_or(t);
        base == "keel" || base == "keel.exe"
    });
    if let Some(i) = keel_idx {
        if let Some(sub) = toks.get(i + 1) {
            if keel_cli::write::human_judgment_ops().contains(&sub.as_str()) {
                return BashVerdict::Ask(format!(
                    "keel {sub} records human judgment or mutates actor identity (K6/K7) - it runs only from a channel the human holds"
                ));
            }
        }
    }
    // Invocations carrying a PERSON's identity (KEEL_ACTOR= / --by / --judged-by naming a Person).
    let persons = keel_cli::actor::person_names(root);
    if !persons.is_empty() {
        for (j, t) in toks.iter().enumerate() {
            let named = t
                .strip_prefix("KEEL_ACTOR=")
                .map(str::to_string)
                .or_else(|| (t == "--by" || t == "--judged-by").then(|| toks.get(j + 1).cloned().unwrap_or_default()));
            if let Some(n) = named {
                if persons.iter().any(|p| p == &n) {
                    return BashVerdict::Ask(format!(
                        "this command writes as the PERSON `{n}` (K6) - a human identity is asserted only from the human's own channel"
                    ));
                }
            }
        }
    }
    BashVerdict::Clean
}

fn hook_pre_bash(payload: &serde_json::Value, root: &Path, session: &str) -> i32 {
    let cmd = payload.pointer("/tool_input/command").and_then(serde_json::Value::as_str).unwrap_or_default();
    // D0176/D0178 tiering first: unambiguous bypass patterns and the never-exempt set.
    let profile = adoption_profile(root);
    match bash_classify(root, cmd) {
        BashVerdict::Block(why) => {
            if profile == "strict" {
                println!(
                    "{}",
                    serde_json::json!({"hookSpecificOutput": {"hookEventName": "PreToolUse", "permissionDecision": "deny",
                        "permissionDecisionReason": format!("[keel] {why} (D0176; blocking under the strict profile)")}})
                );
                return 0;
            }
            println!("[keel] BLOCKED-under-strict pattern: {why} (advisory here; promotion cites the fire-ledger, D0180)");
            ledger_advisory(root, session, &why);
        }
        BashVerdict::Ask(why) => {
            if profile == "strict" {
                println!(
                    "{}",
                    serde_json::json!({"hookSpecificOutput": {"hookEventName": "PreToolUse", "permissionDecision": "ask",
                        "permissionDecisionReason": format!("[keel] {why}")}})
                );
                return 0;
            }
            println!("[keel] human-channel operation: {why} (ask-tier under strict; advisory here)");
            ledger_advisory(root, session, &why);
        }
        BashVerdict::Clean => {}
    }
    let advisories = keel_cli::shellcheck::inspect(cmd);
    if advisories.is_empty() {
        return 0;
    }
    println!("[shell-adaptation -- CLAUDE.md sec 6, the #1 avoidable-friction class (issue094)]");
    let mut spoken = String::new();
    for a in &advisories {
        println!("  {}", a.what);
        println!("    fix: {}", a.fix);
        spoken.push_str(&a.what);
    }
    println!("  Advisory only -- nothing is blocked. If the command errors or hangs, SWITCH TOOLS rather than re-issuing the same form.");
    // issue230 (D0197's untriggerable revisit condition): an advisory that SPEAKS leaves its own
    // ledger event, and speaking the SAME advice again in the same session leaves a repeat event —
    // the mechanical ignore signal. Heeded is then computable as issued-without-repeat
    // (approximate, and enforcement-report says so). Silent fires stay one plain pre-bash line.
    ledger_advisory(root, session, &spoken);
    0
}

/// The issue230 advisory instrumentation: emit `advisory-issued` for a spoken advisory, plus
/// `advisory-repeated` when the same advice hash was already the session's last one — all within
/// the frozen 6-field schema (events are declared vocabulary, fields are not touched).
fn ledger_advisory(root: &Path, session: &str, spoken: &str) {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    spoken.hash(&mut h);
    let digest = format!("{:x}", h.finish());
    let dir = root.join(".keel").join("metrics");
    let _ = std::fs::create_dir_all(&dir);
    let marker = dir.join(format!("last-advice-{}.hash", if session.is_empty() { "nosession" } else { session }));
    let repeated = std::fs::read_to_string(&marker).is_ok_and(|prev| prev.trim() == digest);
    let _ = std::fs::write(&marker, &digest);
    ledger_emit(root, session, "advisory-issued", 0, 0);
    if repeated {
        ledger_emit(root, session, "advisory-repeated", 0, 0);
    }
}

/// `UserPromptSubmit`: inject the route-first checklist, plus a warning about out-of-band writes.
///
/// ASCII-only on purpose: this text is injected into the model's context via stdout, and a non-UTF-8
/// Windows console turns non-ASCII into mojibake.
/// Recalled facts for this prompt, or `None` when nothing should be pushed (D0242 part 3).
///
/// # PUSH, not pull — and why this function is the whole point
///
/// The source process (`.engine/processes/knowledge-graph-memory.sysml`, step `kgInjection`) states it
/// plainly: "THIS STEP IS WHAT MAKES IT PUSH RATHER THAN PULL — a graph the model must decide to query
/// is still pull, and inherits every cost this process exists to remove." Seven of the eight spine
/// steps were built in sprint 424; this is the eighth, and its absence is why the store existed for a
/// week with no measurable effect.
///
/// # THREE WAYS TO TURN IT OFF, in the order a human would reach for them
///
/// 1. `keel deactivate knowledge-graph-memory` — the DECLARED off. The process is a deactivatable unit
///    (D0138), so the switch is committed, auditable, and travels with the project. This is the one to
///    use: a project that turned recall off has DECLARED it, rather than having it quietly not work.
/// 2. `KEEL_RECALL=off` — the immediate, machine-local off, for "it is misbehaving right now" without a
///    commit. Deliberately env-based so it cannot be mistaken for a project decision.
/// 3. Doing nothing — a prompt with no informative token pushes nothing. This is NOT a confidence
///    gate: one was built, measured at 2/8 against 5/8 for no gate at all, and removed (issue297). The
///    surviving check asks only whether there is anything to say, never whether to trust it.
///
/// Note what is NOT a kill switch any more: deleting `.knowledge/`. D0243 made seeding corpus-derived,
/// so removing the store costs aliases and question-coverage, not recall. Saying so matters, because
/// that used to be the removability story.
///
/// FAIL-OPEN, always. Any error, any timeout, any absent store returns `None` and the turn proceeds:
/// injection is an advantage, never a dependency, and a broken index must never cost a turn.
fn recalled_facts(root: &Path, payload: &serde_json::Value) -> Option<String> {
    if std::env::var("KEEL_RECALL").is_ok_and(|v| v.eq_ignore_ascii_case("off")) {
        return None;
    }
    if !keel_cli::activation::Activation::load(root).is_process_active("knowledge-graph-memory") {
        return None;
    }
    let prompt = payload.get("prompt").and_then(serde_json::Value::as_str)?;
    if prompt.trim().is_empty() {
        return None;
    }
    let started = std::time::Instant::now();
    // The confidence verdict and the payload are computed from the same model build, so the cost is
    // paid once. `recall_for_prompt` returns its own "nothing pushed" text for an uninformative prompt,
    // which is a PULL answer - useful at a CLI, noise in a prompt - so the verdict gates it here.
    if !keel_cli::view::has_pushable_facts(root, prompt).unwrap_or(false) {
        return None;
    }
    let facts = keel_cli::view::recall_for_prompt(root, prompt, RECALL_BUDGET).ok()?;
    let ms = started.elapsed().as_millis();
    // A LATENCY CAP that reports rather than hides: past the cap the facts are dropped for this turn
    // and the reader is told, because a recall that silently doubles every turn's latency is the kind
    // of cost that gets discovered months later.
    if ms > RECALL_CAP_MS {
        return Some(format!(
            "[keel recall] SKIPPED — recall took {ms}ms (cap {RECALL_CAP_MS}ms). Facts not pushed this turn.\n"
        ));
    }
    // The VISIBLE recall count and elapsed time the process names as this step's produced artifact.
    Some(format!("[keel recall — pushed before the model, {ms}ms]\n{facts}"))
}

fn hook_user_prompt(root: &Path, payload: &serde_json::Value) -> i32 {
    // PUSH FIRST, then the routing contract: the facts have to be in front of the model when it wakes,
    // and the contract is what it should do with them.
    if let Some(facts) = recalled_facts(root, payload) {
        print!("{facts}");
    }
    // D0064/D0106: routing is structural, fired every turn rather than left to vigilance.
    println!(
        "[engine-triage -- route FIRST (D0064)] Break the request into parts and route EACH before \
acting: CHANGE (sec 3a: workflow/phase/gate/schema) | EXECUTE (sec 3b: tracked artifact, sprinted) \
| RECORD (sec 3c: one atomic fact -- decision/test result/issue) | VIEW (sec 3d: computed answer) | \
ORIENT (sec 3f: where things stand). Flag anything that does NOT cleanly map -- ask, don't \
force-fit. Substantive work goes through a sprint (only trivial one-off edits are exempt). \
method=confirmation needs explicit human sign-off. Invoke the engine-triage skill if unsure."
    );

    // While `keel serve` is live the human authors facts straight into the tree (accepting Decisions,
    // editing items). Those land uncommitted with no signal, and a blanket `git add -A` once swept an
    // accepted D0126/D0127 into a sprint commit unnoticed. Silent when the tree is clean.
    let git = |args: &[&str]| -> String {
        keel_cli::gitx::git()
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default()
    };
    let status = git(&["status", "--porcelain", "--", ".tracking", ".engine"]);
    let status = status.trim();
    if status.is_empty() {
        return 0;
    }
    println!("[out-of-band-writes] Uncommitted changes in .tracking/.engine at turn start - possibly authored");
    println!("via keel serve (human accepts/edits/creates), NOT by me. Run `git diff` and stage DELIBERATELY;");
    println!("do NOT blanket `git add -A` over human-attested writes (accepted Decisions, edits). Changed:");
    for line in status.lines().take(40) {
        println!("  {line}");
    }

    // Surface acceptance/status changes specifically: those are the HUMAN's sign-off and must stay
    // attributed to them, never folded silently into an AI commit.
    let diff = git(&["diff", "--", ".engine/decisions"]);
    let accepts: Vec<&str> = diff
        .lines()
        .filter(|l| {
            l.starts_with('+')
                && (l.contains("DecisionStatus::accepted")
                    || l.contains("DecisionStatus::rejected")
                    || l.contains("Accept :")
                    || l.contains("Reject :")
                    || l.contains("judgedBy"))
        })
        .take(20)
        .collect();
    if !accepts.is_empty() {
        println!("DECISION ACCEPTANCE / STATUS CHANGES (human sign-off - verify + keep as THEIR attributed record):");
        for l in accepts {
            println!("  {l}");
        }
    }
    0
}

/// Emit a hook-protocol JSON object and exit 0 (the harness reads stdout).
fn hook_emit(v: &serde_json::Value) -> i32 {
    println!("{v}");
    0
}

/// Run the fast gate over an edited `.sysml` file; block with the violations if it broke the model.
fn hook_post_edit(payload: &serde_json::Value, root: &Path) -> i32 {
    let path = payload
        .pointer("/tool_input/file_path")
        .or_else(|| payload.pointer("/tool_response/filePath"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default();
    if !std::path::Path::new(path).extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml")) {
        return 0; // only model files are gated
    }

    let mut problems: Vec<String> = Vec::new();
    let report = keel_cli::validate_root(root);
    for (p, d) in &report.diagnostics {
        problems.push(format!("ERROR: {}:{} — {}", p.display(), d.line, d.message));
    }
    for e in &report.errors {
        problems.push(format!("PARSE: {} — {}", e.file.display(), e.message));
    }
    // Only the EXACT guards — a per-edit gate must never fire on a heuristic (see cmd_gate).
    for name in ["duplicate-identity", "marker-vocabulary"] {
        if let Some(r) = keel_cli::guards::run_one(name, root) {
            for v in &r.violations {
                problems.push(format!("GUARD [{name}]: {v}"));
            }
        }
    }
    if problems.is_empty() {
        // Fast tier clean -> the model is not BROKEN, but the edit may have broken something
        // DOWNSTREAM. D0209 clause 4 (dcProactivePostEdit): surface it as NON-BLOCKING guidance so
        // the author fixes it at the point of edit, not at commit. Silent when there is nothing.
        let rel = std::path::Path::new(path)
            .strip_prefix(root)
            .unwrap_or_else(|_| std::path::Path::new(path))
            .to_string_lossy()
            .replace('\\', "/");
        let advisories = keel_cli::proactive::post_edit_advisories(root, &rel);
        if advisories.is_empty() {
            return 0; // clean -> silent, so a passing gate costs nothing
        }
        let mut body = advisories.join("\n");
        body.truncate(2000);
        return hook_emit(&serde_json::json!({
            "systemMessage": format!(
                "[proactive — non-blocking] That edit may have broken something downstream (D0209 clause 4):\n\n{body}\n\nThe model still parses, so this does NOT block — but fix it now, at the point of the edit, before it reaches a commit gate."
            )
        }));
    }
    let mut body = problems.join("\n");
    body.truncate(2000);
    hook_emit(&serde_json::json!({
        "decision": "block",
        "reason": format!(
            "[edit gate] That edit left the model broken — fix it now, at the point of the edit:\n\n{body}\n\nThis is the FAST tier (validate + duplicate-identity + marker-vocabulary, all exact). Author through the keel write API where one exists."
        )
    }))
}

/// The default console port. One constant, because the hook advisory names it to the reader and a
/// wrong number in an advisory is worse than no advisory.
const CONSOLE_PORT: u16 = 7777;

/// How long a hook process may live before it gives up and exits 0 (issue180b).
///
/// Generous enough that the real work always finishes - the stop hook runs validate plus 38 guards,
/// measured at 6-10s - and short enough that an orphaned process releases the binary before it blocks a
/// build. The alternative, an unbounded wait, wedged three builds in a single turn.
const HOOK_DEADLINE_SECS: u64 = 120;

/// Is a KEEL console answering on `127.0.0.1:<port>`?
///
/// It asks `/api/version` and looks for `apiVersion` rather than merely opening a socket, because
/// "something is listening on 7777" is not the claim being made. Reporting an unrelated process as a
/// running console would be the same class of defect as issue140 and issue149 - a tool asserting more
/// than it checked - and here it would tell the human their work is reachable when it is not.
///
/// Raw TCP with std, no HTTP client dependency, and short timeouts so a turn boundary can never hang on
/// it. Any failure at all answers `false`: the advisory that follows is non-blocking, so a false negative
/// costs one redundant line and a false positive costs the human their queue.
fn console_is_up(port: u16) -> bool {
    use std::io::{Read as _, Write as _};
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let Ok(mut s) = std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(300)) else {
        return false;
    };
    let _ = s.set_read_timeout(Some(std::time::Duration::from_millis(400)));
    let req = format!("GET /api/version HTTP/1.1
Host: 127.0.0.1:{port}
Connection: close

");
    if s.write_all(req.as_bytes()).is_err() {
        return false;
    }
    let mut buf = Vec::new();
    let _ = s.take(4096).read_to_end(&mut buf);
    String::from_utf8_lossy(&buf).contains("apiVersion")
}

/// Turn-boundary gate: refuse to end the turn while the model is dishonest. Loop-safe.
fn hook_stop(payload: &serde_json::Value, root: &Path) -> i32 {
    if let Some(w) = engine_version_skew(root) {
        // D0251: the turn boundary is a gate. The skew is reported as a blocking problem (with the
        // repair named) rather than a warning nobody reads.
        eprintln!("{w}");
    }
    let already = payload.get("stop_hook_active").and_then(serde_json::Value::as_bool).unwrap_or(false);

    let mut problems: Vec<String> = Vec::new();
    let report = keel_cli::validate_root(root);
    if !report.diagnostics.is_empty() || !report.errors.is_empty() {
        use std::fmt::Write as _;
        let mut s = String::from("keel validate:\n");
        for (p, d) in report.diagnostics.iter().take(10) {
            let _ = writeln!(s, "  {}:{} — {}", p.display(), d.line, d.message);
        }
        for e in report.errors.iter().take(10) {
            let _ = writeln!(s, "  {} — {}", e.file.display(), e.message);
        }
        problems.push(s);
    }
    // Through run_all, not a run_one loop: run_all applies the ACTIVATION filter, so a guard whose
    // process this project deactivated no longer blocks turns (D0177/P1.5 fixing the guards.rs
    // bypass the proposal cited — hook_stop was the one caller that skipped the filter).
    let mut failing: Vec<String> = Vec::new();
    for r in keel_cli::guards::run_all(root) {
        for v in r.violations.iter().take(5) {
            failing.push(format!("  [{}] {v}", r.name));
        }
    }
    if !failing.is_empty() {
        problems.push(format!("keel guard:\n{}", failing.join("\n")));
    }
    // Declared rules gate the turn (D0177/P1.5): blocking rules block; warning rules report only at
    // their own surfaces (`keel rules`), not here — a turn boundary repeats no warning noise.
    match keel_cli::view::check(root) {
        Ok(json) => {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&json) {
                let empty = Vec::new();
                let mut broken: Vec<String> = Vec::new();
                for r in v.get("rules").and_then(|x| x.as_array()).unwrap_or(&empty) {
                    if r.get("severity").and_then(|s| s.as_str()) == Some("blocking") {
                        for viol in r.get("violations").and_then(|x| x.as_array()).unwrap_or(&empty).iter().take(5) {
                            broken.push(format!("  [rule {}] {viol}", r.get("rule").and_then(|s| s.as_str()).unwrap_or("?")));
                        }
                    }
                }
                if !broken.is_empty() {
                    problems.push(format!("keel rules (blocking):\n{}", broken.join("\n")));
                }
            }
        }
        Err(e) => problems.push(format!("keel rules: cannot evaluate declared rules: {e}")),
    }

    // THE HUMAN MAY BE BLOCKED WHETHER OR NOT THE MODEL IS (issue150, answering their own question about
    // how this went unnoticed). The first version of this advisory sat INSIDE the green branch, so a red
    // model masked it entirely: the turn reported the model and said nothing about an unreachable queue -
    // the same single-channel blindness that let the console stay down while 70 items waited. The console
    // is their oversight lens (D0093) and its availability is not a function of the model's honesty.
    //
    // ADVISORY, NEVER BLOCKING, in both branches: whether the console should be up is their call, and a
    // gate that blocked on it would be the over-strict control that trains its actor to disable it
    // (issue076/issue081), taking the honest-state checks in this same hook down with it.
    // WHERE THE HUMAN IS CHANGES WHAT TO DO ABOUT IT (their request: on remote control, serve HTML
    // feedback instead of a localhost console). A BRIDGED session is one driven through claude.ai rather
    // than this terminal, and for a human on the other end of that bridge a localhost console is not
    // merely down - it is UNREACHABLE, so telling me to start one is advice that cannot help them.
    //
    // THE BASIS IS STATED IN THE MESSAGE rather than asserted: the signal is that
    // CLAUDE_CODE_BRIDGE_SESSION_ID is set, which is the id a remote client attaches to the session. I
    // have not verified that it appears ONLY under remote control, so the message says what it observed
    // instead of claiming to know where they are.
    let bridged = std::env::var("CLAUDE_CODE_BRIDGE_SESSION_ID").is_ok_and(|v| !v.is_empty());
    let oversight = keel_cli::serve::obligations_total(root)
        .filter(|total| *total > 0 && (bridged || !console_is_up(CONSOLE_PORT)))
        .map(|total| if bridged {
            format!(
                "[oversight] {total} item(s) are waiting on the HUMAN, and this session is BRIDGED (CLAUDE_CODE_BRIDGE_SESSION_ID is set), so a localhost console may not be reachable for them. Publish the review deck instead of starting `keel serve`: run the `obligation-review` skill (`keel deck . --out <path>`, publish per the skill with the mcp inbox declared), and hand them the URL."
            )
        } else {
            format!(
                "[oversight] {total} item(s) are waiting on the HUMAN and no keel console is answering on 127.0.0.1:{CONSOLE_PORT}. Start one so they can act: `keel serve . --port {CONSOLE_PORT}`. It holds target/release/keel.exe, so serve a COPY (e.g. keel-serve.exe) if you will rebuild. Only port {CONSOLE_PORT} is checked; a console elsewhere is not detected."
            )
        });

    if problems.is_empty() {
        // green, and the human is not blocked -> silent
        return oversight.map_or(0, |msg| hook_emit(&serde_json::json!({ "systemMessage": msg })));
    }
    if already {
        // Second consecutive red: allow the stop (loop-avoidance stands, issue081) but the yield is
        // now a TRACKED obligation visible in orient, surviving console downtime (D0176/P1.7) — a
        // yield that lives only in a hook message evaporates with the transcript.
        let session = payload.get("session_id").and_then(serde_json::Value::as_str).unwrap_or("");
        let first = problems.first().map(|p| p.chars().take(500).collect::<String>()).unwrap_or_default();
        // An unresolvable actor goes straight to the ledger: a tracked fact with a fabricated
        // createdBy would deepen the red it records (the actors guard would fire on it).
        let recorded = keel_cli::actor::resolve(root, None).map_err(keel_cli::write::WriteError::Parse).and_then(|actor| {
            keel_cli::write::record_obligation(
                root,
                "red-yield",
                "turn gate yielded while red - the tree needs a correction pass",
                &format!("The Stop hook's second red pass yielded (loop avoidance). Session: {session}. First problem at yield: {first}. Discharge: make the tree green and triage this obligation with a #Resolves edge."),
                &actor,
            )
        });
        let note = match recorded {
            Ok(p) => format!("A tracked obligation was recorded at {} (orient-visible).", p.display()),
            Err(e) => {
                ledger_emit(root, session, "red-yield-obligation-UNSYNCED", 1, 0);
                format!("The obligation could NOT be tracked ({e}) - it sits in the local ledger; record it once the tree is writable.")
            }
        };
        return hook_emit(&serde_json::json!({
            "systemMessage": format!("[in-loop gate] Still red after a correction pass — allowing the stop to avoid a loop. Do NOT commit until keel validate + guard are green. {note}")
        }));
    }
    let mut body = problems.join("\n\n");
    body.truncate(4000);
    // Carry it into the BLOCKING reason too, so a dishonest model never swallows the fact that the
    // human cannot reach their queue.
    if let Some(msg) = &oversight {
        body.push_str("\n\n");
        body.push_str(msg);
    }
    hook_emit(&serde_json::json!({
        "decision": "block",
        "reason": format!(
            "[in-loop gate] The model is not in honest state — resolve before ending the turn:\n\n{body}\n\nFix through the keel write API (append-result / add-task / record decision); run `keel guard <name>` for detail. Then end the turn."
        )
    }))
}

/// `keel gate --fast [ROOT]` (D0128 Tier-2) — the per-EDIT in-loop gate.
///
/// Runs only the checks that are (a) fast enough for every edit and (b) EXACT, so blocking is safe:
/// `validate` (227ms — parse + semantic reference resolution), `duplicate-identity` (128ms) and
/// `marker-vocabulary` (140ms). Measured total ~0.5s, against 1.9s for the full guard suite — which is
/// why the full set stays at the TURN boundary (Tier-3 Stop hook) and commit, not per edit.
///
/// Deliberately excludes every heuristic/warning-level guard: a per-edit gate that fires on a prose
/// heuristic would block work mid-thought and train the actor to disable it — the issue076/issue081
/// dynamic that cost eight bypassed commits this sitting.
fn cmd_gate(args: &[String]) -> i32 {
    // D0234: `--workspace` is a SCOPE (every project in this git repo), `--fast` is a TIER (the
    // per-edit subset). A repo holding several projects can only have one core.hooksPath, so its
    // pre-commit hook calls this rather than a per-project gate that could cover just one of them.
    if args.iter().any(|a| a == "--workspace") {
        return keel_cli::workspace::gate_cmd(args);
    }
    let fast = args.iter().any(|a| a == "--fast");
    let root = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(PathBuf::from)
        .or_else(find_repo_root)
        .unwrap_or_else(|| PathBuf::from("."));
    // D0251: the fast tier is a gate too — a per-edit verdict from an undeclared engine misleads
    // in-loop exactly as a commit verdict would at the boundary.
    if let Some(w) = engine_version_skew(&root) {
        eprintln!("{w}");
        eprintln!("gate REFUSED under engine-version skew (D0251). Run the pinned version, or `keel migrate`.");
        return 2;
    }
    if !fast {
        eprintln!("usage: keel gate --fast [ROOT]   (the per-edit in-loop gate: validate + duplicate-identity + marker-vocabulary + scaffold-placeholder)");
        eprintln!("       keel gate --workspace [ROOT]   (the COMMIT gate for a repo holding several projects: every project the commit touches, D0234)");
        return 2;
    }

    let report = keel_cli::validate_root(&root);
    let mut failed = false;
    for (path, d) in &report.diagnostics {
        println!("ERROR: {}:{} — {}", path.display(), d.line, d.message);
        failed = true;
    }
    for e in &report.errors {
        println!("PARSE: {} — {}", e.file.display(), e.message);
        failed = true;
    }
    // The EXACT fast-tier guards — set membership, duplicate detection, unfilled scaffolds. No heuristics.
    for name in ["duplicate-identity", "marker-vocabulary", "scaffold-placeholder"] {
        if let Some(r) = keel_cli::guards::run_one(name, &root) {
            for v in &r.violations {
                println!("GUARD [{name}]: {v}");
                failed = true;
            }
        }
    }
    if failed {
        println!("\ngate: FAST GATE FAILED — fix before continuing (this is the per-edit tier; the full guard set runs at turn end + commit).");
        return 1;
    }
    println!("gate: fast gate clean ({} file(s))", report.validated);
    0
}

fn cmd_check_engine(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel check-engine [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    let diags = keel_cli::validate_engine_instances(&root);
    for (path, d) in &diags {
        println!("ERROR: {}:{} — {}", path.display(), d.line, d.message);
        if let Some(hint) = &d.suggestion {
            println!("       hint: {hint}");
        }
    }
    if diags.is_empty() {
        println!(".engine instance files validated clean (kernel-free; D0112 phase 2).");
        0
    } else {
        eprintln!("{} .engine semantic diagnostic(s).", diags.len());
        1
    }
}

fn cmd_spec_version(args: &[String]) -> i32 {
    use keel_parser::spec_compat as sc;
    println!("grammar version (baked): {}", sc::SYSML_V2_GRAMMAR_VERSION);
    println!("pinned sha:              {}", sc::SYSML_V2_GRAMMAR_SHA);
    println!("spec url:                {}", sc::SYSML_V2_SPEC_URL);
    if sc::is_offline() || args.iter().any(|a| a == "--no-fetch") {
        println!("live check:              skipped (offline)");
        return 0;
    }
    let fetched = std::process::Command::new("curl")
        .args(["-sSL", sc::SYSML_V2_SPEC_URL])
        .output();
    let Ok(out) = fetched else {
        println!("live check:              unavailable (curl not found)");
        return 0;
    };
    if !out.status.success() || out.stdout.is_empty() {
        println!("live check:              unavailable (no network)");
        return 0;
    }
    let live = sc::sha256_hex(&out.stdout);
    println!("live sha:                {live}");
    let pinned = sc::SYSML_V2_GRAMMAR_SHA;
    if pinned.bytes().all(|b| b == b'0') {
        println!("status:                  not pinned — baked version is the reference; pin the live sha to enable drift detection");
        0
    } else if live == pinned {
        println!("status:                  CURRENT");
        0
    } else {
        println!("status:                  STALE — upstream changed since the pin");
        1
    }
}

fn cmd_check(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--spec-version") {
        return cmd_spec_version(args);
    }
    if args.is_empty() {
        eprintln!("usage: keel check FILE [FILE...]  |  keel check --spec-version [--no-fetch]");
        return 2;
    }
    let files: Vec<PathBuf> = args.iter().map(PathBuf::from).collect();
    let report = check_files(&files);

    for err in &report.errors {
        println!("ERROR: {} — {}", err.file.display(), err.message);
    }
    if report.is_clean() {
        println!("{} file(s) checked clean.", files.len());
        0
    } else {
        eprintln!(
            "{} file(s) checked — {} error(s).",
            files.len(),
            report.errors.len()
        );
        1
    }
}

fn cmd_serve(args: &[String]) -> i32 {
    let mut port: u16 = 7777;
    let mut root_arg: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--port" {
            if let Some(v) = it.next() {
                if let Ok(p) = v.parse::<u16>() {
                    port = p;
                }
            }
        } else if !a.starts_with("--") {
            root_arg = Some(a.clone());
        }
    }
    let root = match root_arg {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel serve [--port N] [ROOT] [--stop] [--forget]");
                return 2;
            }
        }
    };
    if !keel_cli::workspace::is_project(&root) {
        eprintln!("serve: {} is not a keel project (needs .engine/ and .tracking/).", root.display());
        return 2;
    }

    // `--forget`: stop listing this project in the console selector, without touching the project.
    if args.iter().any(|a| a == "--forget") {
        return match keel_cli::console_registry::deregister(&root) {
            Ok(true) => {
                println!("console: {} deregistered — it will no longer appear in the selector.", root.display());
                0
            }
            Ok(false) => {
                println!("console: {} was not registered; nothing to forget.", root.display());
                0
            }
            Err(e) => {
                eprintln!("console: {e}");
                1
            }
        };
    }

    // ATTACH, DO NOT SPAWN (D0245 clause 2). This is the whole fix for "too many keel serve windows":
    // running the command in a second project used to start a second server, because binding was the
    // first thing tried. Now the first thing asked is whether one of ours is already answering — and
    // the check distinguishes OUR console from any program holding the socket, because those two
    // situations need opposite responses: attach, or refuse loudly.
    let today = keel_cli::scaffold::today();
    if keel_cli::console_registry::console_on(port) {
        if let Err(e) = keel_cli::console_registry::register(&root, Some(port), &today) {
            eprintln!("console: registered nothing ({e}) — the console is running but this project");
            eprintln!("  will not appear in its selector until the registry is writable.");
            return 1;
        }
        println!("Keel console is ALREADY RUNNING on http://127.0.0.1:{port} — attached, did not start a second.");
        println!("  registered: {}", keel_cli::workspace::canon(&root).display());
        println!("  open:       http://127.0.0.1:{port}/  then pick it from the project selector");
        println!("  forget it:  keel serve --forget {}", root.display());
        return 0;
    }
    // No console of ours. Register BEFORE binding, so the project is in the selector the moment the
    // surface comes up rather than one restart later.
    if let Err(e) = keel_cli::console_registry::register(&root, Some(port), &today) {
        eprintln!("console: could not record this project in the registry: {e}");
        eprintln!("  starting anyway — the selector will show only the active project.");
    }
    keel_cli::serve::run(root, port)
}

fn cmd_orient(args: &[String]) -> i32 {
    let html = args.iter().any(|a| a == "--html");
    let root = match root_arg(args, "keel orient [ROOT] [--html]", &["html"], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    // D0251 clause C: a READ view proceeds under skew — blocking orient would block the command
    // that diagnoses the skew — but it warns LOUDLY, because what this view shows may not be what
    // the pinned engine's gate would say. Stderr, so the JSON stays parseable.
    if let Some(w) = engine_version_skew(&root) {
        eprintln!("{w}");
    }
    if html {
        return match keel_cli::view::orient_html(&root) {
            Ok(h) => {
                println!("{h}");
                0
            }
            Err(e) => {
                eprintln!("orient --html error: {e}");
                1
            }
        };
    }
    // K2 visibility (D0174/P0.3): a scaffolded commit gate that git is not wired to run is a
    // silently-open enforcement point. Warn LOUDLY on stderr — the JSON on stdout stays pure.
    // issue240: ARMED means git can REACH the hook, not that a setting points somewhere. The old
    // check passed on `core.hooksPath = nul`, so the gate was silently dead while this warned nothing.
    if let Err(why) = keel_cli::gitx::commit_gate_armed(&root) {
        if root.join(".githooks").join("pre-commit").exists() {
            eprintln!("[keel] WARNING: the commit gate is NOT ARMED — {why}. Fix: git config core.hooksPath .githooks (D0174/K2).");
        }
    }
    println!("{}", orient::compute(&root).to_json());
    0
}

fn cmd_attestation_coverage(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel attestation-coverage [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::attestation_coverage(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("attestation-coverage error: {e}");
            1
        }
    }
}

fn cmd_orphans(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel orphans [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::algo::orphans(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("orphans error: {e}");
            1
        }
    }
}

fn cmd_audit(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel audit [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::algo::audit(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("audit error: {e}");
            1
        }
    }
}

fn resolve_guard_root(arg: Option<&String>) -> Option<PathBuf> {
    arg.map_or_else(find_repo_root, |p| Some(PathBuf::from(p)))
}

/// A string that names a runnable guard (an enforced one, or a runnable-only diagnostic).
fn is_guard_name(s: &str) -> bool {
    keel_cli::guards::GUARD_NAMES.contains(&s) || matches!(s, "assured" | "critique" | "critique-rigor" | "defect-guard-coverage")
}

/// Classify `keel guard` args into `(guard name to run, root arg)`. A first arg that is a known guard
/// name runs THAT guard; `all`, no arg, or a non-name first arg (a ROOT path like `.` or a dir) runs
/// ALL guards on that root. This is what lets `keel guard <ROOT>` work like `keel validate <ROOT>`.
fn classify_guard_args(args: &[String]) -> (Option<&str>, Option<&str>) {
    match args.first().map(String::as_str) {
        None => (None, None),
        Some("all") => (None, args.get(1).map(String::as_str)),
        Some(a) if is_guard_name(a) => (Some(a), args.get(1).map(String::as_str)),
        Some(a) => (None, Some(a)), // a bare ROOT, not a guard name
    }
}

fn cmd_guard(args: &[String]) -> i32 {
    // `keel guard` / `guard [ROOT]` / `guard all [ROOT]` → run all; `guard <name> [ROOT]` → run one.
    let (name, root_arg) = classify_guard_args(args);
    let Some(root) = resolve_guard_root(root_arg.map(String::from).as_ref()) else {
        eprintln!("error: no .engine/ directory found. usage: keel guard [<name>] [ROOT]");
        return 2;
    };
    if let Some(w) = engine_version_skew(&root) {
        eprintln!("{w}");
        eprintln!("guard REFUSED under engine-version skew (D0251). Run the pinned version, or `keel migrate`.");
        return 2;
    }
    let Some(name) = name else {
        let reports = keel_cli::guards::run_all(&root);
        let mut all_ok = true;
        for r in &reports {
            r.print();
            all_ok &= r.ok();
        }
        // issue244: the verdict used to be the bare word ALL PASS while 80 warnings scrolled above
        // it, including a live recorded-release contradiction that had shipped unread. Detected-but-
        // unread is this repo's highest-frequency drift mechanism, and it grows with every control
        // added — so the SUBTRACTIVE fix is to state the warning population where the verdict is
        // read, rather than build another detector for what was already detected.
        let warned: Vec<&str> =
            reports.iter().filter(|r| !r.warnings.is_empty()).map(|r| r.name).collect();
        let total: usize = reports.iter().map(|r| r.warnings.len()).sum();
        let tail = if total == 0 {
            String::new()
        } else {
            format!(
                " — {total} warning(s) across {} guard(s), NOT violations and NOT blocking, but UNREAD until someone reads them: {}",
                warned.len(),
                warned.join(", ")
            )
        };
        println!("[guard] {}{tail}", if all_ok { "ALL PASS" } else { "FAILED" });
        return i32::from(!all_ok);
    };
    let Some(report) = keel_cli::guards::run_one(name, &root) else {
        eprintln!(
            "unknown guard '{name}' (enforced: {} | runnable diagnostics: assured, critique, critique-rigor, defect-guard-coverage)",
            keel_cli::guards::GUARD_NAMES.join(", ")
        );
        return 2;
    };
    report.print();
    // Asking for ONE guard by name is a diagnostic, so the check still RUNS and its findings are still
    // shown — but the exit code must agree with the enforced gate (D0138). Without this, `keel guard
    // issues` exits 1 on a project that never adopted issue-resolution while `keel guard` exits 0, and a
    // script wired to the single-guard form would block on a control the project deliberately does not
    // enforce.
    if let keel_cli::activation::GuardState::Inactive(p) =
        keel_cli::activation::Activation::load(&root).guard_state(name)
    {
        println!(
            "[guard:{name}] NOT ACTIVE — process `{p}` is not in this project's active set, so the findings above are informational and do NOT block (`keel activate {p}` to enforce them)"
        );
        return 0;
    }
    i32::from(!report.ok())
}

// Root-only query: `keel <name> [ROOT]`.
fn cmd_query0(args: &[String], usage: &str, f: fn(&std::path::Path) -> String) -> i32 {
    let root = match root_arg(args, &format!("keel {usage} [ROOT]"), &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    println!("{}", f(&root));
    0
}

// Name + optional root: `keel <name> <arg> [ROOT]`.
fn cmd_query1(args: &[String], usage: &str, f: fn(&std::path::Path, &str) -> String) -> i32 {
    let arg = match positional_arg(args, &format!("keel {usage} <name> [ROOT]"), "an item name") {
        Ok(a) => a,
        Err(code) => return code,
    };
    let root = match root_arg(args, &format!("keel {usage} <name> [ROOT]"), &[], 1) {
        Ok(r) => r,
        Err(code) => return code,
    };
    // AN UNRESOLVABLE NAME IS A FAILURE, NOT AN EMPTY RESULT (issue177). Every command routed through
    // here used to exit 0 for a name that does not exist, answering `{upstream: [], downstream: []}` -
    // which a script reads as "no relations", the reassuring wrong answer. `report`, `render` and `arch`
    // already exit nonzero on an unknown argument; these six did not, so the CLI was inconsistent with
    // itself on the one interface D0093 makes the automation substrate.
    if !keel_cli::queries::is_declared(&root, arg) {
        eprintln!(
            "keel {usage}: no item named `{arg}` is declared in this model.
               An unknown name exits nonzero rather than answering with an empty result, because an empty              result reads as `this item has no relations` (issue177)."
        );
        return 1;
    }
    println!("{}", f(&root, arg));
    0
}

/// `keel reverify [--all-drift | --task NAME] [--by ACTOR] [ROOT]` (D0101) — re-run the configured gate
/// at HEAD and stamp a fresh `TestResult` on each drift-suspect task on green.
fn cmd_reverify(args: &[String]) -> i32 {
    let mut task: Option<String> = None;
    let mut by: Option<String> = None;
    let mut root: Option<PathBuf> = None;
    let mut i = 0;
    while let Some(a) = args.get(i) {
        match a.as_str() {
            "--all-drift" => {}
            "--task" => {
                i += 1;
                task = args.get(i).cloned();
            }
            // The actor error names `--judged-by`, `--author` and `--by` as equivalents, so all three
            // are accepted here. They were not: `--judged-by claudeOpus5` fell through to the ROOT
            // arm, made the root `claudeOpus5`, and the command then refused with "no acting actor"
            // — an error about provenance for what was really an unknown flag, pointing at the one
            // fix that could not work.
            "--by" | "--judged-by" | "--author" => {
                i += 1;
                by = args.get(i).cloned();
            }
            // An unrecognised FLAG is a mistake, not a path. Swallowing it as a root turned a typo
            // into a confident wrong answer somewhere further downstream, which is this session's
            // most-repeated defect shape.
            other if other.starts_with("--") => {
                eprintln!("error: unknown flag `{other}`");
                eprintln!("usage: keel reverify [--all-drift | --task NAME] [--by ACTOR] [ROOT]");
                return 2;
            }
            other => root = Some(PathBuf::from(other)),
        }
        i += 1;
    }
    let root = root.or_else(find_repo_root).unwrap_or_else(|| PathBuf::from("."));
    // reverify STAMPS a fresh TestResult, so it needs a true attributable actor (D0129/issue072).
    let by = match keel_cli::actor::resolve(&root, by.as_deref()) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("{msg}");
            return 2;
        }
    };
    keel_cli::reverify::run(&root, task.as_deref(), &by)
}

/// `keel intake [ROOT]` (D0166) — what was said, what it became, what nobody acted on.
fn cmd_intake(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel intake [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::intake(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

fn cmd_open_issues(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel open-issues [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::open_issues(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("open-issues error: {e}");
            1
        }
    }
}

fn cmd_dispositions(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel dispositions [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::dispositions(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("dispositions error: {e}");
            1
        }
    }
}

fn cmd_sitting_coverage(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel sitting-coverage [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::sitting_coverage(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("sitting-coverage error: {e}");
            1
        }
    }
}

fn cmd_deck(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel deck [ROOT] [--out FILE]", &["out"], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::deck::html(&root) {
        Ok(h) => {
            if let Some(out) = flag(args, "out") {
                if let Err(e) = keel_cli::write::write_atomic(std::path::Path::new(&out), &h) {
                    eprintln!("keel deck: writing {out}: {e}");
                    return 1;
                }
                println!("deck -> {out}");
            } else {
                println!("{h}");
            }
            0
        }
        Err(e) => {
            eprintln!("keel deck: {e}");
            1
        }
    }
}

/// `keel mint [N]` (us019/issue170) — engine-minted v4 UUIDs, one per line, nothing else on stdout,
/// composable into any authoring script.
///
/// Exists so no authoring path depends on an AI generating identity by hand: two hand-minted ids
/// were mangled before guard 38 existed, and manual diligence is not a control (D0047). What this
/// prints is tested against guard 38's OWN shape predicate, so mint and guard stay one truth.
fn cmd_mint(args: &[String]) -> i32 {
    const USAGE: &str = "keel mint [N]   (N >= 1, default 1)";
    let n: u64 = match args {
        [] => 1,
        [a] => {
            if a.starts_with('-') {
                // the positional_arg convention (issue179): a leading dash is never a count
                eprintln!("error: `{a}` looks like a flag, not a count.");
                eprintln!("usage: {USAGE}");
                return 2;
            }
            match a.parse::<u64>() {
                Ok(n) if n >= 1 => n,
                _ => {
                    eprintln!("error: `{a}` is not a count of at least 1.");
                    eprintln!("usage: {USAGE}");
                    return 2;
                }
            }
        }
        _ => {
            eprintln!("usage: {USAGE}");
            return 2;
        }
    };
    let mut out = String::new();
    for _ in 0..n {
        out.push_str(&keel_cli::write::gen_uuid());
        out.push('\n');
    }
    print!("{out}");
    0
}

/// `keel new sprint <N> <slug> --charter <decision> [--points P]` (dcSprintScaffold/us019) — the
/// engine scaffolds the ceremony record: ids minted, provenance from the bound actor (refused when
/// absent), placeholders the fast gate rejects. See [`keel_cli::scaffold`].
fn cmd_new(args: &[String]) -> i32 {
    const USAGE: &str = "keel new sprint <NUMBER> <slug> --charter <decision> [--points P]";
    if args.first().map(String::as_str) != Some("sprint") {
        eprintln!("usage: {USAGE}");
        return 2;
    }
    let rest = args.get(1..).unwrap_or(&[]);
    let positionals: Vec<&String> = {
        let mut out = Vec::new();
        let mut skip = false;
        for a in rest {
            if skip {
                skip = false;
                continue;
            }
            if a.starts_with("--") {
                skip = true; // every flag here takes a value
                continue;
            }
            out.push(a);
        }
        out
    };
    let [number_arg, slug] = positionals.as_slice() else {
        eprintln!("usage: {USAGE}");
        return 2;
    };
    let Ok(number) = number_arg.parse::<u32>() else {
        eprintln!("error: `{number_arg}` is not a sprint number.");
        eprintln!("usage: {USAGE}");
        return 2;
    };
    let Some(charter) = flag(rest, "charter") else {
        eprintln!("error: --charter <decision> is required — a sprint's story is chartered, never orphaned.");
        eprintln!("usage: {USAGE}");
        return 2;
    };
    let points: u32 = match flag(rest, "points") {
        None => 1,
        Some(p) => match p.parse() {
            Ok(n) if n >= 1 => n,
            _ => {
                eprintln!("error: --points takes a count of at least 1.");
                return 2;
            }
        },
    };
    let root = find_repo_root().unwrap_or_else(|| PathBuf::from("."));
    // The provenance rule: the author is the bound actor, REFUSED when absent — never defaulted.
    let actor = match keel_cli::actor::resolve(&root, None) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("keel new sprint: {msg}");
            return 1;
        }
    };
    match keel_cli::scaffold::sprint(&root, number, slug, &charter, points, &actor) {
        Ok(path) => {
            println!("scaffolded -> {}", path.display());
            println!("fill every {} before judging any gate - `keel gate --fast` rejects it until then", keel_cli::scaffold::PLACEHOLDER);
            0
        }
        Err(e) => {
            eprintln!("keel new sprint: {e}");
            1
        }
    }
}

/// `keel decision-follow-through [ROOT] [--table]` (dcDecisionFollowThroughView/us020) — JSON is the
/// authority; `--table` renders the same data for eyes: one line per accepted Decision with
/// downstream work, then the gaps.
fn cmd_decision_follow_through(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel decision-follow-through [ROOT] [--table]", &["table"], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    let json = match keel_cli::view::decision_follow_through(&root) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("keel decision-follow-through: {e}");
            return 1;
        }
    };
    if !args.iter().any(|a| a == "--table") {
        println!("{json}");
        return 0;
    }
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&json) else {
        eprintln!("keel decision-follow-through: internal: view emitted unparsable JSON");
        return 1;
    };
    let empty = Vec::new();
    println!(
        "accepted {}   with-downstream {}   gaps {}",
        v.get("acceptedDecisions").and_then(serde_json::Value::as_i64).unwrap_or(0),
        v.get("withDownstream").and_then(serde_json::Value::as_i64).unwrap_or(0),
        v.get("gapCount").and_then(serde_json::Value::as_i64).unwrap_or(0),
    );
    for d in v.get("decisions").and_then(|x| x.as_array()).unwrap_or(&empty) {
        let items = d.get("items").and_then(|x| x.as_array()).cloned().unwrap_or_default();
        let summary: Vec<String> = items
            .iter()
            .map(|i| {
                format!(
                    "{} ({})",
                    i.get("item").and_then(|x| x.as_str()).unwrap_or("?"),
                    i.get("evidence").and_then(|x| x.as_str()).unwrap_or("?"),
                )
            })
            .collect();
        println!("  {}  <-  {}", d.get("decision").and_then(|x| x.as_str()).unwrap_or("?"), summary.join(", "));
    }
    let gaps: Vec<&str> =
        v.get("gaps").and_then(|x| x.as_array()).unwrap_or(&empty).iter().filter_map(|g| g.as_str()).collect();
    if !gaps.is_empty() {
        println!("  GAPS (no downstream tracked item): {}", gaps.join(", "));
    }
    0
}

/// `keel sync-claude [ROOT] [--check]` (D0174/P0.2) — regenerate the keel-owned subset of the
/// `.claude/` surface in place (foreign entries survive), or with `--check` report drift and
/// version skew without writing. `--check` IS the `claude-surface-drift` guard's implementation.
fn cmd_sync_claude(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel sync-claude [ROOT] [--check]", &["check"], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    let check = args.iter().any(|a| a == "--check");
    match keel_cli::claude_surface::sync_claude(&root, check) {
        Ok(r) => {
            if check {
                for d in &r.drift {
                    println!("DRIFT: {d}");
                }
                if let Some((old, new)) = &r.version_skew {
                    println!("REGENERATE: surface stamped by generator {old}, this binary is {new} — run `keel sync-claude` (an obligation, not a violation)");
                }
                if r.drift.is_empty() {
                    println!("claude-surface: keel-owned subset matches this binary's generator ({} registry skill(s))", r.registry_count);
                    0
                } else {
                    1
                }
            } else {
                println!(
                    "claude-surface synced: settings.json merged (keel-owned entries only), output style, {}/{} skill(s), version {} stamped.",
                    r.skills_written,
                    r.registry_count,
                    keel_cli::claude_surface::SURFACE_VERSION
                );
                0
            }
        }
        Err(e) => {
            eprintln!("keel sync-claude: {e}");
            1
        }
    }
}


/// `keel override <path> --reason "<text>"` (D0176 tier 3) — the sanctioned unlock for a direct
/// write the API cannot express. Single-use, target-path-bound, expiring; consumption records an
/// orient-visible obligation naming the path actually written (K7). Never a silent env var.
fn cmd_override(args: &[String]) -> i32 {
    const USAGE: &str = "keel override <path> --reason \"why the API cannot express this write\"";
    let target = match positional_arg(args, USAGE, "a file path") {
        Ok(a) => a.replace('\\', "/"),
        Err(code) => return code,
    };
    let Some(reason) = flag(args, "reason").filter(|r| r.trim().len() >= 10) else {
        eprintln!("error: --reason is required (at least 10 characters) - the reason IS the record (D0176).");
        eprintln!("usage: {USAGE}");
        return 2;
    };
    let root = find_repo_root().unwrap_or_else(|| PathBuf::from("."));
    let actor = match keel_cli::actor::resolve(&root, None) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("keel override: {msg}");
            return 1;
        }
    };
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let _ = std::fs::create_dir_all(root.join(".keel"));
    let unlock = serde_json::json!({"path": target, "reason": reason, "actor": actor, "ts": ts});
    if let Err(e) = keel_cli::write::write_atomic(&override_path(&root), unlock.to_string()) {
        eprintln!("keel override: cannot write the unlock: {e}");
        return 1;
    }
    println!("override armed for `{target}` - SINGLE USE, expires in {OVERRIDE_TTL_SECS}s; consumption records an obligation (D0176/K7).");
    0
}


/// `keel enforcement-report [ROOT]` (D0180/K14) — fires, blocks, overrides, red-yields, and the
/// adherence trend, computed from the machine-local fire-ledger. Promotion decisions cite this.
fn cmd_enforcement_report(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel enforcement-report [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::pm::enforcement_report(&root) {
        Ok(j) => {
            println!("{j}");
            0
        }
        Err(e) => {
            eprintln!("keel enforcement-report: {e}");
            1
        }
    }
}

fn cmd_hardening(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel hardening [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::hardening::hardening(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("keel hardening: {e}");
            1
        }
    }
}

fn cmd_concern_coverage(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel concern-coverage [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::concern_coverage(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("concern-coverage error: {e}");
            1
        }
    }
}

// `keel rules [ROOT]` (D0105 EXPAND step 2): evaluate the DECLARED rules (`keel check` is taken by the
// spec-compat file checker; the D0105 name reconciliation is a tracked follow-up). Runs ALONGSIDE
// `keel guard` until parity retires each guard (guardsToRulesMigration).
fn cmd_rules(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel rules [ROOT] [--enforce]", &["enforce"], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::check(&root) {
        Ok(json) => {
            if !args.iter().any(|a| a == "--enforce") {
                println!("{json}");
                return 0;
            }
            // The gate form (D0177/P1.5): blocking rules FAIL the caller; warnings print.
            let Ok(v) = serde_json::from_str::<serde_json::Value>(&json) else {
                eprintln!("rules --enforce: internal: unparsable rule report");
                return 1;
            };
            let empty = Vec::new();
            let mut blocked = 0usize;
            for r in v.get("rules").and_then(|x| x.as_array()).unwrap_or(&empty) {
                let viols = r.get("violations").and_then(|x| x.as_array()).cloned().unwrap_or_default();
                if viols.is_empty() {
                    continue;
                }
                let name = r.get("rule").and_then(|s| s.as_str()).unwrap_or("?");
                let sev = r.get("severity").and_then(|s| s.as_str()).unwrap_or("?");
                for viol in &viols {
                    println!("[rule {name} - {sev}] {viol}");
                }
                if sev == "blocking" {
                    blocked += viols.len();
                }
            }
            if blocked > 0 {
                println!("rules: {blocked} blocking violation(s)");
                1
            } else {
                println!("rules: enforced clean");
                0
            }
        }
        Err(e) => {
            eprintln!("rules error: {e}");
            1
        }
    }
}

// `keel launchables [ROOT]` (srServeModelDrivenRegistry, Tier 1a): the model-declared launchable set.
fn cmd_launchables(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel launchables [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::launchables(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("launchables error: {e}");
            1
        }
    }
}

// `keel business [ROOT]` (serveBusinessNeedsView): the Business layer (Brief/Personas/Needs/UseCases).
fn cmd_business(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel business [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::business(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("business error: {e}");
            1
        }
    }
}

fn cmd_coverage(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel coverage [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::coverage(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("coverage error: {e}");
            1
        }
    }
}

fn cmd_diagram(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel diagram [ROOT]  (redirect to a .html file)", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::diagram_html(&root) {
        Ok(html) => {
            println!("{html}");
            0
        }
        Err(e) => {
            eprintln!("diagram error: {e}");
            1
        }
    }
}

fn cmd_decisions(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel decisions [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::decisions_report(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("decisions error: {e}");
            1
        }
    }
}

fn cmd_assured(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel assured [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::assured(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("assured error: {e}");
            1
        }
    }
}

fn cmd_critique_coverage(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel critique-coverage [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::critique_coverage(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("critique-coverage error: {e}");
            1
        }
    }
}

fn cmd_critique_policy(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel critique-policy [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::critique_policy(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("critique-policy error: {e}");
            1
        }
    }
}

fn cmd_governing_version(args: &[String]) -> i32 {
    let item = match positional_arg(
        args,
        "keel governing-version <delivery Story name> [ROOT]",
        "an item name",
    ) {
        Ok(a) => a,
        Err(code) => return code,
    };
    let root = match root_arg(args, "keel governing-version <delivery Story name> [ROOT]", &[], 1) {
        Ok(r) => r,
        Err(code) => return code,
    };
    // Same rule as `cmd_query1` (issue177). This command has its own wrapper, which is exactly how it
    // escaped the first fix: `keel governing-version .` reported a process AND a process definition for
    // a name that does not exist, which is the most confidently wrong answer of the six.
    if !keel_cli::queries::is_declared(&root, item) {
        eprintln!("keel governing-version: no item named `{item}` is declared in this model.");
        return 1;
    }
    println!("{}", keel_cli::govern::governing_version(&root, item));
    0
}

fn cmd_reprocess_candidates(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel reprocess-candidates [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    println!("{}", keel_cli::govern::reprocess_candidates(&root));
    0
}

fn cmd_suspect(args: &[String]) -> i32 {
    let explain = args.iter().any(|a| a == "--explain");
    let root = match root_arg(args, "keel suspect [--explain] [ROOT]", &["explain"], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    println!("{}", keel_cli::govern::suspect(&root, explain));
    0
}

fn cmd_view(args: &[String]) -> i32 {
    // issue179: a view name resolves to a file under `.engine/views`, so a flag here is a path lookup.
    if let Some(f) = args.first().filter(|a| a.starts_with('-')) {
        eprintln!("error: `{f}` looks like a flag, not a view name (issue179).");
        return 2;
    }
    let Some(name) = args.first() else {
        eprintln!("usage: keel view <name> [ROOT]");
        return 2;
    };
    let root = match root_arg(args, "keel view <name> [ROOT]", &[], 1) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::run(&root, name) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("view error: {e}");
            1
        }
    }
}

fn cmd_whats_next(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel whats-next [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    // issue239/issue247: whats-next used to print nothing and exit 0 whether the frontier was
    // genuinely empty or a filter had failed to compute — identical output for COMPUTED-EMPTY and
    // COULD-NOT-COMPUTE, on the one answer the AI auto-follows (D0052). Now the two are distinct:
    // a failed computation REFUSES rather than answering with silence.
    let out = orient::compute(&root);
    if !out.compute_failures.is_empty() {
        eprintln!("whats-next: COULD-NOT-COMPUTE — refusing to print a frontier that may be wrong:");
        for r in &out.compute_failures {
            eprintln!("  {r}");
        }
        eprintln!("  This is NOT an empty frontier. Fix the model read, then re-run.");
        return 1;
    }
    if out.ready.is_empty() {
        eprintln!("whats-next: COMPUTED-EMPTY — no task is ready (computed over {} outstanding item(s)). This is an answer, not a failure.", out.outstanding);
    }
    for task in out.ready {
        println!("{task}");
    }
    0
}

fn cmd_ls(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel ls [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    let dir = root.join(".tracking");
    for p in collect_sysml(&dir) {
        println!("{}", p.display());
    }
    0
}

/// Parse simple `--key value` flag pairs from a flat args slice.
fn flag(args: &[String], name: &str) -> Option<String> {
    let key = format!("--{name}");
    args.windows(2).find_map(|w| match w {
        [k, v] if *k == key => Some(v.clone()),
        _ => None,
    })
}

/// A provenance DATE, refused rather than defaulted (issue182).
///
/// Five write paths read this as `flag(args, ..).unwrap_or_else(|| "2026-01-01".to_owned())`. CLAUDE.md
/// says provenance is never defaulted; that rule was implemented for the ACTOR, where a missing actor
/// makes the write refuse, and the DATE fell back to a false constant. A result written without a date
/// claimed it happened on 2026-01-01, which corrupts any series and feeds guard 36 - whose whole job is
/// catching evidence that cites a date it could not have had.
///
/// REFUSED, not defaulted to today: an AI has no clock it can honestly attest to, and guessing is what
/// produced the constant in the first place. The caller states the date or the write does not happen.
fn provenance_date(args: &[String], flag_name: &str, usage: &str) -> Result<String, i32> {
    flag(args, flag_name).ok_or_else(|| {
        eprintln!("error: --{flag_name} YYYY-MM-DD is required.");
        eprintln!(
            "  A provenance date is never defaulted (issue182): it used to fall back to 2026-01-01."
        );
        eprintln!("usage: {usage}");
        2
    })
}

fn cmd_append_result(args: &[String]) -> i32 {
    let Some(file_str) = flag(args, "file") else {
        eprintln!("usage: keel append-result --file FILE --task TASK --sha SHA [--verdict pass|fail] [--judged-by ACTOR] [--judged-at DATE]");
        return 2;
    };
    let Some(task) = flag(args, "task") else {
        eprintln!("error: --task required");
        return 2;
    };
    let Some(sha) = flag(args, "sha") else {
        eprintln!("error: --sha required");
        return 2;
    };
    let file = PathBuf::from(file_str);
    let verdict = flag(args, "verdict").unwrap_or_else(|| "pass".to_owned());
    // Provenance is never defaulted (D0129/issue072): refuse rather than attribute falsely.
    let judged_by = match keel_cli::actor::resolve(&keel_cli::actor::root_for(&file), flag(args, "judged-by").as_deref()) {
        Ok(a) => a,
        Err(msg) => { eprintln!("{msg}"); return 2; }
    };
    // Callers should pass --judged-at for determinism; this is a safe fallback.
    let judged_at = match provenance_date(args, "judged-at", "keel <write> --judged-at YYYY-MM-DD ...") {
        Ok(d) => d,
        Err(c) => return c,
    };

    let evidence = flag(args, "evidence");
    match w::append_result(&file, &task, &sha, &verdict, &judged_at, &judged_by, evidence.as_deref()) {
        Ok(uuid) => { println!("{uuid}"); 0 }
        Err(e) => { eprintln!("error: {e}"); 1 }
    }
}

fn cmd_append_gate_result(args: &[String]) -> i32 {
    let Some(file_str) = flag(args, "file") else {
        eprintln!("usage: keel append-gate-result --file FILE --gate GATE --sha SHA [--verdict pass|fail] [--judged-by ACTOR] [--judged-at DATE]");
        return 2;
    };
    let Some(gate) = flag(args, "gate") else {
        eprintln!("error: --gate required");
        return 2;
    };
    let Some(sha) = flag(args, "sha") else {
        eprintln!("error: --sha required");
        return 2;
    };
    let file = PathBuf::from(file_str);
    let verdict = flag(args, "verdict").unwrap_or_else(|| "pass".to_owned());
    // Provenance is never defaulted (D0129/issue072): refuse rather than attribute falsely.
    let judged_by = match keel_cli::actor::resolve(&keel_cli::actor::root_for(&file), flag(args, "judged-by").as_deref()) {
        Ok(a) => a,
        Err(msg) => { eprintln!("{msg}"); return 2; }
    };
    // Callers should pass --judged-at for determinism; this is a safe fallback.
    let judged_at = match provenance_date(args, "judged-at", "keel <write> --judged-at YYYY-MM-DD ...") {
        Ok(d) => d,
        Err(c) => return c,
    };

    let notes = flag(args, "notes");
    let evidence = flag(args, "evidence");
    match w::append_gate_result(&file, &gate, &sha, &verdict, &judged_at, &judged_by, notes.as_deref(), evidence.as_deref()) {
        Ok(uuid) => { println!("{uuid}"); 0 }
        Err(e) => { eprintln!("error: {e}"); 1 }
    }
}

/// `keel record <type> ...` — the closed RMWX `record` verb (D0105/D0106; issue054 C1). Currently
/// records a Decision: `keel record decision --slug S --title T --context C --decision D --rationale R
/// --consequences Q --date YYYY-MM-DD --author A [--root ROOT]` → writes a proposed Decision file
/// (auto NNNN + UUID), killing point-of-decision friction (D0054). Acceptance stays a separate human gate.
/// Parse a `--from FILE` decision draft (issue255): `key: value` headers, then `--- key` sections
/// whose body runs to the next `---` marker. Deliberately NOT TOML/JSON — a Decision's fields are
/// paragraphs, and a format that needs the author to escape quotes reintroduces the very problem
/// the file is here to remove.
///
/// ```text
/// slug: decision-authoring-is-core
/// date: 2026-08-24
/// marker: process-change
/// --- title
/// One line or many; every field takes prose verbatim.
/// --- context
/// Quotes, "double quotes", backticks and $VARIABLES are all literal here.
/// ```
fn decision_fields_from_file(path: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read --from {path}: {e}"))?;
    let mut out: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let mut key: Option<String> = None;
    let mut body: Vec<&str> = Vec::new();
    let flush = |out: &mut std::collections::BTreeMap<String, String>, key: &Option<String>, body: &[&str]| {
        if let Some(k) = key {
            out.insert(k.clone(), body.join(" ").split_whitespace().collect::<Vec<_>>().join(" "));
        }
    };
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("--- ") {
            flush(&mut out, &key, &body);
            body.clear();
            key = Some(rest.trim().to_string());
        } else if key.is_some() {
            body.push(line);
        } else if let Some((k, v)) = line.split_once(':') {
            let (k, v) = (k.trim(), v.trim());
            if !k.is_empty() && !v.is_empty() && !k.starts_with('#') {
                out.insert(k.to_string(), v.to_string());
            }
        }
    }
    flush(&mut out, &key, &body);
    if out.is_empty() {
        return Err(format!("--from {path} declared no fields (expected `key: value` lines and `--- key` sections)"));
    }
    Ok(out)
}

fn cmd_record(args: &[String]) -> i32 {
    if args.first().map(String::as_str) == Some("issue") {
        return cmd_record_issue(args);
    }
    // D0236: intake had NO write path. Every Statement in this repo was hand-edited into a file,
    // which is the one record type where that matters most - D0216 requires the human's words
    // VERBATIM before any Need exists, and a hand-typed "verbatim" field is a paraphrase waiting
    // to happen.
    if args.first().map(String::as_str) == Some("statement") {
        return cmd_record_statement(args);
    }
    if args.first().map(String::as_str) == Some("story") {
        return cmd_record_story(args);
    }
    if args.first().map(String::as_str) != Some("decision") {
        eprintln!("usage: keel record decision --slug S --title T --context C --decision D --rationale R --consequences Q --date YYYY-MM-DD --author A [--root ROOT]");
        eprintln!("       keel record decision --from DRAFT.md   (prose in a file - the sanctioned path, issue255; flags override)");
        eprintln!("       keel record issue --title T --description D --severity Critical|High|Medium|Low --resolver R --date YYYY-MM-DD [--related-task T] [--marker M] [--in-field] [--by A] [--root ROOT]");
        eprintln!("       keel record statement --text \"<their exact words>\" | --from FILE --said-by A --said-at D --title T [--channel C]   (VERBATIM, D0216/D0236)");
        eprintln!("       keel record story --from-statement stNNN --title T --as-a R --i-want C --implication K [--so-that O] [--triage-note W] --at D");
        return 2;
    }
    let root = flag(args, "root").map_or_else(
        || find_repo_root().unwrap_or_else(|| PathBuf::from(".")),
        PathBuf::from,
    );
    // issue255: `--from FILE` is the SANCTIONED authoring path for a Decision's prose. Passing five
    // paragraphs as double-quoted shell arguments is how ~2000 characters of `keel hardening` output
    // ended up inside D0223's `decision` field: a backtick inside double quotes is command
    // substitution, so the shell RAN the command the prose merely named. A file has no such layer.
    // The write path refuses tool-output-shaped prose either way (`reject_injected_output`); this is
    // the road that makes the refusal easy to obey rather than a rule to remember (D0054).
    let from_file = match flag(args, "from").map(|f| decision_fields_from_file(&f)) {
        Some(Ok(fields)) => Some(fields),
        Some(Err(msg)) => { eprintln!("error: {msg}"); return 2; }
        None => None,
    };
    let req = |name: &str| {
        flag(args, name).or_else(|| from_file.as_ref().and_then(|m| m.get(name).cloned()))
    };
    let (Some(slug), Some(title), Some(context), Some(decision), Some(rationale), Some(consequences)) =
        (req("slug"), req("title"), req("context"), req("decision"), req("rationale"), req("consequences"))
    else {
        eprintln!("error: --slug --title --context --decision --rationale --consequences are all required (a substantive why — D0103)");
        return 2;
    };
    let date = req("date").unwrap_or_default();
    // NEVER default to a named human (D0129/issue072): that silently forges a human attestation.
    let author = match keel_cli::actor::resolve(&root, req("author").as_deref()) {
        Ok(a) => a,
        Err(msg) => { eprintln!("{msg}"); return 2; }
    };
    if date.is_empty() {
        eprintln!("error: --date YYYY-MM-DD required (the attestation time is its own irreducible fact)");
        return 2;
    }
    // issue213: the D0070 marker as a first-class flag - forgetting it cost two landing commits.
    let from_marker = from_file.as_ref().and_then(|m| m.get("marker")).map(String::as_str);
    let marker = if args.iter().any(|a| a == "--process-change") || from_marker == Some("process-change") {
        Some("ProspectiveChange")
    } else if args.iter().any(|a| a == "--safety-change") || from_marker == Some("safety-change") {
        Some("SafetyChange")
    } else {
        None
    };
    let research = req("research");
    match w::record_decision(&root, &slug, &title, &date, &author, &context, &decision, &rationale, &consequences, marker, research.as_deref()) {
        Ok((nnnn, path)) => {
            println!("recorded D{nnnn} (proposed) -> {path}");
            println!("accept later via an explicit human sign-off (flip status + add the d{nnnn}Accept event).");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `keel record statement --text "<their exact words>" --said-by A --said-at D --channel C --title T`
///
/// `--text` is passed through VERBATIM (escaped for the literal, otherwise untouched), so it must be
/// their words and not a summary; `--title` is the AI's label and is sanitised like any other field.
/// `--from FILE` reads the text from a file, which is the sanctioned path for anything containing
/// quotes, newlines or backticks - the same reason `record decision --from` exists (D0224/issue255).
fn cmd_record_statement(args: &[String]) -> i32 {
    let root = flag(args, "root").map_or_else(
        || find_repo_root().unwrap_or_else(|| PathBuf::from(".")),
        PathBuf::from,
    );
    let from_file = flag(args, "from").map(std::fs::read_to_string);
    let text = match from_file {
        Some(Ok(t)) => Some(t.trim_end_matches(['\n', '\r']).to_string()),
        Some(Err(e)) => {
            eprintln!("error: cannot read --from: {e}");
            return 2;
        }
        None => flag(args, "text"),
    };
    let (Some(text), Some(said_by), Some(said_at), Some(title)) =
        (text, flag(args, "said-by"), flag(args, "said-at"), flag(args, "title"))
    else {
        eprintln!("usage: keel record statement --text \"<their exact words>\" | --from FILE");
        // Derived, not restated (issue300): the usage line a user reads is the same vocabulary
        // surface as the check, so it must come from the same source or it will drift from it.
        eprintln!(
            "       --said-by ACTOR --said-at YYYY-MM-DD --title T [--channel {}]",
            keel_cli::schema::enum_members_union(&root, "StatementChannel").join("|")
        );
        eprintln!("       [--by RECORDER] [--at YYYY-MM-DD] [--root ROOT]");
        eprintln!("  --text is VERBATIM (D0216): their words, not a summary. --title is your label for it.");
        return 2;
    };
    let channel = flag(args, "channel").unwrap_or_else(|| "chat".to_string());
    let author = match keel_cli::actor::resolve(&root, flag(args, "by").as_deref()) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("{msg}");
            return 2;
        }
    };
    // Provenance is never defaulted (D0129/issue182): the RECORD's date is its own fact, separate
    // from when they said it.
    let created_at = flag(args, "at").unwrap_or_else(|| said_at.clone());
    match keel_cli::intake_write::record_statement(
        &root,
        &keel_cli::intake_write::NewStatement {
            text: &text,
            said_by: &said_by,
            said_at: &said_at,
            channel: &channel,
            title: &title,
            author: &author,
            created_at: &created_at,
        },
    ) {
        Ok((name, path)) => {
            println!("recorded {name} -> {path}");
            println!("  their words are stored VERBATIM. Next: `keel record story --from-statement {name} ...`");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `keel record story --from-statement stNNN --title T --as-a R --i-want C --implication K`
///
/// The `#DerivedFrom` edge to the cited `Statement` is authored WITH the story, and a story citing a
/// `Statement` that does not exist is REFUSED with nothing written: a `UserStory` with no source is an
/// invention wearing a story's clothes (D0216).
fn cmd_record_story(args: &[String]) -> i32 {
    let root = flag(args, "root").map_or_else(
        || find_repo_root().unwrap_or_else(|| PathBuf::from(".")),
        PathBuf::from,
    );
    let (Some(from), Some(title), Some(as_a), Some(i_want), Some(implication)) = (
        flag(args, "from-statement"),
        flag(args, "title"),
        flag(args, "as-a"),
        flag(args, "i-want"),
        flag(args, "implication"),
    ) else {
        eprintln!("usage: keel record story --from-statement stNNN --title T --as-a ROLE --i-want CAPABILITY");
        // Derived, not restated (issue300) — see the note at the `record statement` usage.
        eprintln!(
            "       --implication {}",
            keel_cli::schema::enum_members_union(&root, "ImplicationKind").join("|")
        );
        eprintln!("       [--so-that OUTCOME] [--triage-note WHY] [--by RECORDER] [--at YYYY-MM-DD] [--root ROOT]");
        eprintln!("  --from-statement is REQUIRED: a UserStory with no cited source is an invention (D0216).");
        return 2;
    };
    let author = match keel_cli::actor::resolve(&root, flag(args, "by").as_deref()) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("{msg}");
            return 2;
        }
    };
    let Some(created_at) = flag(args, "at") else {
        eprintln!("error: --at YYYY-MM-DD required (the record's date is its own irreducible fact, issue182)");
        return 2;
    };
    let so_that = flag(args, "so-that");
    let triage = flag(args, "triage-note");
    match keel_cli::intake_write::record_story(
        &root,
        &keel_cli::intake_write::NewStory {
            from_statement: &from,
            title: &title,
            as_a: &as_a,
            i_want: &i_want,
            so_that: so_that.as_deref(),
            implication: &implication,
            triage_note: triage.as_deref(),
            author: &author,
            created_at: &created_at,
        },
    ) {
        Ok((name, path)) => {
            println!("recorded {name} -> {path}  (#DerivedFrom {from} authored with it)");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

fn cmd_add_task(args: &[String]) -> i32 {
    let Some(file_str) = flag(args, "file") else {
        eprintln!("usage: keel add-task --file FILE --def DEF --task TASK --dod TEXT --method METHOD");
        return 2;
    };
    let Some(def_name) = flag(args, "def") else {
        eprintln!("error: --def required");
        return 2;
    };
    let Some(task) = flag(args, "task") else {
        eprintln!("error: --task required");
        return 2;
    };
    let Some(dod) = flag(args, "dod") else {
        eprintln!("error: --dod required");
        return 2;
    };
    let file = PathBuf::from(file_str);
    let method = flag(args, "method").unwrap_or_else(|| "test".to_owned());

    match w::add_task(&file, &def_name, &task, &dod, &method) {
        Ok(uuid) => { println!("{uuid}"); 0 }
        Err(e) => { eprintln!("error: {e}"); 1 }
    }
}

/// `render <view> [--mode graph|table|review] [--root ROOT]` — modular interactive-artifact
/// renderer over the view layer (D0086). Emits self-contained HTML to stdout (redirect to a file).
fn cmd_render(args: &[String]) -> i32 {
    let Some(view) = args.first().filter(|v| !v.starts_with('-')) else {
        eprintln!("usage: keel render <view> [--mode graph|table|review] [--root ROOT]");
        eprintln!("  <view> = a declared view name (e.g. decisions, issues), or 'model' for the whole-model graph");
        return 2;
    };
    let mode = flag(args, "mode").unwrap_or_else(|| "graph".to_owned());
    let root = match flag(args, "root") {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ found from cwd upward; pass --root ROOT");
                return 2;
            }
        }
    };
    match keel_cli::view::render_html(&root, view, &mode) {
        Ok(html) => {
            println!("{html}");
            0
        }
        Err(e) => {
            eprintln!("render error: {e}");
            1
        }
    }
}

/// `report <name> [--html] [--root ROOT]` — computed aggregate scorecard (D0087): assurance |
/// traceability | quality-debt | flow. JSON by default; `--html` emits a human-digestible scorecard.
fn cmd_report(args: &[String]) -> i32 {
    let Some(name) = args.first().filter(|v| !v.starts_with('-')) else {
        eprintln!("usage: keel report <assurance|traceability|quality-debt|flow|governance> [--html] [--trend] [--root ROOT]");
        return 2;
    };
    let root = match flag(args, "root") {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ found from cwd upward; pass --root ROOT");
                return 2;
            }
        }
    };
    let html = args.iter().any(|a| a == "--html");
    let trend = args.iter().any(|a| a == "--trend");
    let result = if html { keel_cli::view::report_html(&root, name, trend) } else { keel_cli::view::report(&root, name, trend) };
    match result {
        Ok(out) => {
            println!("{out}");
            0
        }
        Err(e) => {
            eprintln!("report error: {e}");
            1
        }
    }
}

/// `indicators [--trend] [--root ROOT]` — monitored measures (D0089) with direction-aware status.
/// Computed indicators show current value (full series with `--trend`); pulled/manual show their
/// recorded-Measurement series + status.
fn cmd_indicators(args: &[String]) -> i32 {
    // issue281: this accepted `--root` ONLY, so a POSITIONAL root was silently ignored and the view
    // was computed against `find_repo_root()` — whatever project the process happened to be standing
    // in. `keel indicators <someWorkspace>` therefore reported on a DIFFERENT project and exited 0,
    // which is worse than the zeroed-green this issue is about: the numbers are real, just about
    // something else. Found by a test sweeping every model reader, not by reading the code. Going
    // through `root_arg` gives it the positional, the unknown-flag refusal, and the project
    // precondition that the other model readers already have.
    let root = match root_arg(args, "keel indicators [ROOT] [--trend] [--root ROOT]", &["trend", "root"], 0) {
        Ok(r) => flag(args, "root").map_or(r, PathBuf::from),
        Err(code) => return code,
    };
    if let Err(code) = keel_cli::workspace::require_project(&root, "keel indicators [ROOT] [--trend]") {
        return code;
    }
    let trend = args.iter().any(|a| a == "--trend");
    match keel_cli::view::indicators(&root, trend) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("indicators error: {e}");
            1
        }
    }
}

/// `record-measurement --indicator I --value V [--at DATE] [--source S] [--by ACTOR] [--file F]` —
/// record a Measurement datapoint (D0089) for a pulled/manual indicator (write path).
fn cmd_record_measurement(args: &[String]) -> i32 {
    let Some(indicator) = flag(args, "indicator") else {
        eprintln!("usage: keel record-measurement --indicator I --value V [--at DATE] [--source S] [--by ACTOR] [--file F]");
        return 2;
    };
    let Some(value) = flag(args, "value") else {
        eprintln!("error: --value required");
        return 2;
    };
    let file = flag(args, "file").map_or_else(
        || find_repo_root().map_or_else(|| PathBuf::from(".tracking/indicators.sysml"), |r| r.join(".tracking").join("indicators.sysml")),
        PathBuf::from,
    );
    let at = match provenance_date(args, "at", "keel <write> --at YYYY-MM-DD ...") {
        Ok(d) => d,
        Err(c) => return c,
    };
    let source = flag(args, "source").unwrap_or_default();
    let by = match keel_cli::actor::resolve(&keel_cli::actor::root_for(&file), flag(args, "by").as_deref()) {
        Ok(a) => a,
        Err(msg) => { eprintln!("{msg}"); return 2; }
    };
    match w::append_measurement(&file, &indicator, &value, &at, &source, &by) {
        Ok(name) => {
            println!("{name}");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `snapshot-indicators [--at DATE] [--by ACTOR] [--file F] [--root ROOT]` — take a reading of every
/// COMPUTED indicator (its current `metric_value`) and bank it as a `Measurement` (D0091). Run per
/// sprint/quarter to build a durable, fast series alongside the pulled/manual observations.
fn cmd_snapshot_indicators(args: &[String]) -> i32 {
    let root = match flag(args, "root") {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ found from cwd upward; pass --root ROOT");
                return 2;
            }
        }
    };
    let file = flag(args, "file").map_or_else(|| root.join(".tracking").join("indicators.sysml"), PathBuf::from);
    let at = match provenance_date(args, "at", "keel <write> --at YYYY-MM-DD ...") {
        Ok(d) => d,
        Err(c) => return c,
    };
    let by = match keel_cli::actor::resolve(&root, flag(args, "by").as_deref()) {
        Ok(a) => a,
        Err(msg) => { eprintln!("{msg}"); return 2; }
    };
    let keys = match keel_cli::view::computed_indicator_keys(&root) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let mut count = 0u32;
    for (indicator, key) in &keys {
        let Some(v) = keel_cli::view::metric_value(&root, key) else {
            eprintln!("skip {indicator}: metric '{key}' not computable");
            continue;
        };
        match w::append_measurement(&file, indicator, &format!("{v:.6}"), &at, "snapshot (computed reading)", &by) {
            Ok(name) => {
                println!("{name}  ({indicator} = {v:.2})");
                count += 1;
            }
            Err(e) => {
                eprintln!("error on {indicator}: {e}");
                return 1;
            }
        }
    }
    println!("banked {count} computed-indicator snapshot(s) @ {at} into {}", file.display());
    0
}

#[derive(serde::Deserialize)]
struct ReviewBatch {
    #[serde(default)]
    dispositions: Vec<ReviewDisp>,
    #[serde(default, rename = "judgedBy")]
    judged_by: String,
    #[serde(default, rename = "judgedAgainst")]
    judged_against: String,
}

#[derive(serde::Deserialize)]
struct ReviewDisp {
    element: String,
    verdict: String,
    #[serde(default)]
    lens: String,
    #[serde(default)]
    severity: String,
    #[serde(default)]
    rationale: String,
    #[serde(default)]
    actionable: bool,
}

/// `apply-review --batch FILE [--sha SHA] [--judged-by ACTOR] [--judged-at DATE] [--root ROOT]` —
/// ingest a review batch exported by `render --mode review` and write each disposition back as a new
/// linked critique (D0086) via the write path. `accept`->pass, `finding`/`reject`->fail (a finding,
/// which induces computed suspicion). Writes into `.tracking/critiques.sysml`.
fn cmd_apply_review(args: &[String]) -> i32 {
    let Some(batch_str) = flag(args, "batch") else {
        eprintln!("usage: keel apply-review --batch FILE [--sha SHA] [--judged-by ACTOR] [--judged-at DATE] [--root ROOT]");
        return 2;
    };
    let root = match flag(args, "root") {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ found from cwd upward; pass --root ROOT");
                return 2;
            }
        }
    };
    let text = match std::fs::read_to_string(&batch_str) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error reading batch {batch_str}: {e}");
            return 2;
        }
    };
    let batch: ReviewBatch = match serde_json::from_str(&text) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error: invalid review batch JSON: {e}");
            return 2;
        }
    };
    let judged_by = flag(args, "judged-by").filter(|s| !s.is_empty()).or_else(|| Some(batch.judged_by.clone()).filter(|s| !s.is_empty())).unwrap_or_else(|| "human".to_owned());
    let sha = flag(args, "sha").filter(|s| !s.is_empty()).or_else(|| Some(batch.judged_against.clone()).filter(|s| !s.is_empty())).unwrap_or_else(|| "uncommitted".to_owned());
    let judged_at = match provenance_date(args, "judged-at", "keel <write> --judged-at YYYY-MM-DD ...") {
        Ok(d) => d,
        Err(c) => return c,
    };
    // issue210: apply-review's records land in the JUDGE's per-actor file (forward-only routing).
    let Ok(critiques) = w::per_actor_file(&root, "critiques", &judged_by) else {
        eprintln!("error: cannot open the per-actor critiques file");
        return 1;
    };

    let mut count = 0u32;
    for d in &batch.dispositions {
        // Finding disposition (D0092): act / accept-risk / dismiss target a finding ISSUE, written as a
        // method=confirmation disposition (#Dispositions-linked), not a critique.
        if let Some(verdict) = match d.verdict.as_str() {
            "act" => Some("act"),
            "accept-risk" | "acceptRisk" => Some("acceptRisk"),
            "dismiss" => Some("dismiss"),
            _ => None,
        } {
            let disp = w::Disposition { finding: &d.element, verdict, rationale: &d.rationale, sha: &sha, judged_at: &judged_at, judged_by: &judged_by };
            match w::append_disposition(&critiques, &disp) {
                Ok(name) => {
                    println!("{name}  ({} disposition:{verdict})", d.element);
                    count += 1;
                }
                Err(e) => {
                    eprintln!("error on {}: {e}", d.element);
                    return 1;
                }
            }
            continue;
        }
        let outcome = match d.verdict.as_str() {
            "accept" => "pass",
            "finding" | "reject" => "fail",
            other => {
                eprintln!("skip {}: unknown verdict '{other}'", d.element);
                continue;
            }
        };
        let severity = (outcome == "fail" && !d.severity.is_empty()).then_some(d.severity.as_str());
        let lens = if d.lens.is_empty() { "correctness" } else { d.lens.as_str() };
        let mut rationale = d.rationale.clone();
        if d.actionable {
            rationale.push_str(" [actionable: warrants new implementation]");
        }
        let c = w::Critique {
            element: &d.element,
            method: "critique",
            lens,
            critiqued_by: "human",
            severity,
            rationale: &rationale,
            outcome,
            sha: &sha,
            judged_at: &judged_at,
            judged_by: &judged_by,
        };
        match w::append_critique(&critiques, &c) {
            Ok(name) => {
                println!("{name}  ({} {})", d.element, outcome);
                count += 1;
            }
            Err(e) => {
                eprintln!("error on {}: {e}", d.element);
                return 1;
            }
        }
    }
    println!("applied {count} disposition(s) to {}", critiques.display());
    0
}

/// Write one embedded engine file into `dst_engine`, remapping `decisions/*` -> `reference/decisions/*`
/// (read-only reference, NOT instance — the engine's architecture decisions must not enter the new
/// project's computed views, which scan `.engine/decisions`; D0093 engine/instance boundary).
// The scaffold path rules live in `migrate` — `keel migrate` resyncs a downstream `.engine/` from
// this same embedded tree, so it must map and exclude paths identically to `keel init` or a migrated
// project would differ from a freshly inited one. One definition, both callers.
use keel_cli::migrate::{is_engine_dev_only, remap_engine_content, remap_engine_path};

/// The `.engine/contracts/` files that are THIS project's instance data rather than engine definition
/// (issue243). `keel init` must reset these, not copy them: an adoption declaration and a set of
/// exchange identities belong to the project that made them.
fn is_instance_contract(rel: &Path) -> bool {
    matches!(
        rel.to_string_lossy().replace('\\', "/").as_str(),
        "contracts/activation.toml"
            | "contracts/unit-ids.toml"
            | "contracts/installed-units.toml"
            // D0219: WHO MAY DECIDE is the most consequential instance fact in the tree, and init
            // was shipping it verbatim — so every new project silently inherited THIS project's
            // decider and would have recorded acceptances in their name. Reset to an empty template.
            | "contracts/github-actors.toml"
    )
}

/// The starter content for a reset instance contract. `activation.toml` gets a commented-out template
/// so the honest default (absent section = everything active, D0138) is what a fresh project HAS,
/// while still showing how to declare a subset. The id registries start genuinely empty: an identity
/// is minted on first export, never inherited.
fn starter_for(rel: &Path) -> &'static str {
    if rel.to_string_lossy().replace('\\', "/") == "contracts/github-actors.toml" {
        return "# github-actors - GitHub login -> keel actor mapping (D0205 githubChannel).\n\
#\n\
# THIS TABLE IS WHO MAY DECIDE ON THIS PROJECT, and it starts EMPTY on purpose (D0219): inheriting\n\
# another project's decider would let their login record acceptances in your tree. An unmapped login\n\
# is REFUSED, never defaulted (issue182: provenance is never defaulted).\n\
#\n\
# Add one line per human who may decide here. Logins are matched exactly and case-sensitively as\n\
# GitHub reports them. Only HUMANS belong here: this table exists to attribute human judgment, and\n\
# mapping a bot login would recreate the AI-recorded-as-human class (issue072/073) at the channel\n\
# layer. The repo OWNER is not automatically a decider, and an ORG can never be one - an org is not\n\
# a person and cannot hold judgment.\n\
#\n\
# Check yours with `keel github-decider <login>`.\n\
\n\
[logins]\n\
# yourGithubLogin = \"yourKeelActor\"\n";
    }
    if rel.to_string_lossy().replace('\\', "/") == "contracts/activation.toml" {
        return "# Process activation (D0138) - which processes THIS project has adopted.
#
# NO SECTION BELOW MEANS EVERYTHING IS ACTIVE, which is the honest default for a new project: a
# project that never adopted a control has not violated it (issue090). Declare a subset only when
# you have actually chosen one - `keel activate <process>` / `keel deactivate <process>` write it
# for you, and `keel process list` shows what there is to choose from.
#
# CORE guards (identity, provenance, vocabulary, well-formedness) are in no unit and CANNOT be
# deactivated. Activation stops enforcing procedures you have not adopted; it does not make
# truthfulness optional.
#
# [processes]
# active = [\"agile-workflow\"]
#
# [viewpoints]
# active = [\"orientVP\"]
";
    }
    "# Process-unit identity registry (D0183): a unit's id is its EXCHANGE identity - stable across
# exports, matched by `import --update`. Minted on THIS project's first export; never inherited,
# because an inherited id claims a lineage this project does not have (issue243).
"
}

fn write_engine_file(f: &include_dir::File, dst_engine: &Path, count: &mut u32) -> std::io::Result<()> {
    let rel = f.path();
    if is_engine_dev_only(rel) {
        return Ok(()); // engine-dev-only (kernel/python toolchain) — not shipped to downstream projects
    }
    let dst = dst_engine.join(remap_engine_path(rel));
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // The deliverable-suspicion manifest is instance-specific (lists the ENGINE's own tasks) — reset it
    // to a starter so a fresh project passes manifest-coverage (D0093 engine/instance boundary).
    if rel == Path::new("deliverable-manifest.txt") {
        std::fs::write(&dst, STARTER_MANIFEST)?;
    } else if is_instance_contract(rel) {
        // issue243: these three contracts are THIS project's instance data, not engine definition, and
        // shipping them verbatim made every new project inherit the self-build's choices. A fresh tree
        // reported `declared manifest: yes` for an adoption declaration it never made — falsifying
        // CLAUDE.md's own guarantee that an absent file means everything is active, so a project that
        // never adopted a control has not violated it. The file was not absent; it was someone else's.
        // unit-ids/installed-units are worse than misleading: they are the self-build's EXCHANGE
        // IDENTITIES for units the new project never exported, so an `import --update` could match
        // against a lineage that is not its own. Reset to a starter, exactly as the manifest already is.
        std::fs::write(&dst, starter_for(rel))?;
    } else if let Some(text) = std::str::from_utf8(f.contents())
        .ok()
        .and_then(|s| remap_engine_content(rel, s))
    {
        // issue291: the reference copy's package is renamed so the project's own first decision
        // cannot collide with it. See `remap_engine_content`.
        std::fs::write(&dst, text)?;
    } else {
        std::fs::write(&dst, f.contents())?;
    }
    *count += 1;
    Ok(())
}

/// Recursively scaffold the embedded engine tree into `dst_engine`. `include_dir`'s `File::path()` is
/// root-relative, so the remap in `write_engine_file` sees the full path regardless of nesting.
fn scaffold_engine(dir: &Dir, dst_engine: &Path, count: &mut u32) -> std::io::Result<()> {
    for f in dir.files() {
        write_engine_file(f, dst_engine, count)?;
    }
    for d in dir.dirs() {
        scaffold_engine(d, dst_engine, count)?;
    }
    Ok(())
}

/// `keel init DIR` (D0093) — scaffold a fresh project: the embedded engine (`.engine/`, with the
/// architecture decisions remapped to read-only `reference/`), `CLAUDE.md`, and a starter `.tracking/`.
/// Self-contained cold start; refuses to overwrite an existing `.engine/`.
/// The first positional argument, REFUSING anything that looks like a flag (issue179).
///
/// `keel init --help` created a directory named `--help` and scaffolded a complete engine into it; 277
/// files reached this repository and were committed before the CRLF warnings gave it away. `cmd_init`
/// read `args.first()` directly and so never passed through `root_arg`, which has rejected unknown
/// flags all along - the bypass was the bug, not the parsing.
///
/// A leading `-` is refused wherever a PATH or a NAME is expected. For `init` the stakes are highest,
/// because its whole job is writing a tree to disk, so any string it accepts is a filesystem mutation.
fn positional_arg<'a>(args: &'a [String], usage: &str, what: &str) -> Result<&'a String, i32> {
    let Some(first) = args.first() else {
        eprintln!("usage: {usage}");
        return Err(2);
    };
    if first.starts_with('-') {
        eprintln!("error: `{first}` looks like a flag, not {what}.");
        eprintln!("  Refused rather than used: `keel init --help` once created a directory named");
        eprintln!("  `--help` and scaffolded an engine into it (issue179).");
        eprintln!("usage: {usage}");
        return Err(2);
    }
    Ok(first)
}

/// Write the scaffolded pre-commit gate at `repo_root` and arm `core.hooksPath` there (issue278).
///
/// `repo_root` is the git repository root, which is the only place a hook can be invoked from, and
/// `project` is the project just scaffolded — used only to say which one armed the gate.
///
/// An EXISTING hook is never overwritten. In a workspace the repo-root hook is shared, so a second
/// `keel init` would be silently replacing a file the first project (or a human) owns — D0108: a
/// non-owner may add, never overwrite in place. The scaffolded body is workspace-scoped and needs no
/// per-project edit, so an existing keel hook already covers the new project; anything else is the
/// author's own gate and is reported rather than clobbered.
fn install_commit_gate(repo_root: &Path, project: &Path) -> Result<(), i32> {
    let hooks = repo_root.join(".githooks");
    if let Err(e) = std::fs::create_dir_all(&hooks) {
        eprintln!("error creating {}: {e}", hooks.display());
        return Err(1);
    }
    let hook_path = hooks.join("pre-commit");
    let existed = hook_path.exists();
    if existed {
        let body = std::fs::read_to_string(&hook_path).unwrap_or_default();
        if body.contains("gate --workspace") {
            println!("commit gate already installed at {} — it is workspace-scoped and covers this project too.", hook_path.display());
        } else {
            println!("NOTE: {} exists and is not the scaffolded keel gate — left untouched.", hook_path.display());
            println!("  Add `keel gate --workspace .` to it, or this project is not gated at commit.");
        }
    } else {
        if let Err(e) = std::fs::write(&hook_path, PRECOMMIT_HOOK) {
            eprintln!("error writing {}: {e}", hook_path.display());
            return Err(1);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let _ = std::fs::set_permissions(&hook_path, std::fs::Permissions::from_mode(0o755));
        }
        println!("commit gate written to {} (workspace-scoped).", hook_path.display());
    }
    // Arm it. Without this the hook is a file nothing runs — the K2 failure the drift warning exists
    // for. Armed HERE rather than left to the printed next-steps, because the step a newcomer is most
    // likely to skip is the one that turns the gate on.
    if repo_root.join(".git").exists() {
        let _ = keel_cli::gitx::git()
            .arg("-C")
            .arg(repo_root)
            .args(["config", "core.hooksPath", ".githooks"])
            .status();
        println!("core.hooksPath set to .githooks in {} (the gate is live).", repo_root.display());
    } else {
        println!("NOTE: {} is not a git repository yet — the gate is written but NOT ARMED.", repo_root.display());
        println!("  Run `git init` HERE (not inside the project) and then:");
        println!("    git -C {} config core.hooksPath .githooks", repo_root.display());
    }
    if project != repo_root {
        println!("  (the gate lives at the repository root because git allows one hooks path per repo)");
    }
    Ok(())
}

/// The D0174/P0 slice of `init`: the declared adoption-profile fact, the `.claude/` enforcement
/// surface (five hook events, output style, per-registry skills), the optional CI template, and
/// wiring `core.hooksPath` when a `.git` exists.
fn init_enforcement_surface(dir: &Path, engine_dst: &Path, profile: &str) -> Result<(), i32> {
    let contracts = engine_dst.join("contracts");
    let _ = std::fs::create_dir_all(&contracts);
    let profile_fact = format!(
        "# Adoption profile — DECLARED at init, never inferred (D0174/P0.4).
         # strict: blocking in-loop gates from day one. guided: advisory-first; promote to blocking
         # with `keel sync-claude` after the D0180 evidence window, citing the fire-ledger.
         profile = \"{profile}\"
declaredAt = \"{}\"
",
        keel_cli::scaffold::today()
    );
    if let Err(e) = std::fs::write(contracts.join("adoption-profile.toml"), profile_fact) {
        eprintln!("error writing adoption-profile.toml: {e}");
        return Err(1);
    }
    match keel_cli::claude_surface::sync_claude(dir, false) {
        Ok(r) => println!(
            ".claude/ scaffolded: settings.json (5 hook events), output style, {} skill(s) (= registry count {}).",
            r.skills_written, r.registry_count
        ),
        Err(e) => {
            eprintln!("error scaffolding .claude/: {e}");
            return Err(1);
        }
    }
    let wf = dir.join(".github").join("workflows");
    let _ = std::fs::create_dir_all(&wf);
    if let Err(e) = std::fs::write(wf.join("keel-gate.yml"), keel_cli::claude_surface::CI_TEMPLATE) {
        eprintln!("error writing CI template: {e}");
        return Err(1);
    }
    // `core.hooksPath` is armed by `install_commit_gate` against the REPOSITORY root (issue278).
    // It used to be armed here against the project directory, which is the wrong repository whenever
    // the project is a workspace peer.
    Ok(())
}

/// The scaffold's closing narration: what was written, and the two onboarding steps in order.
///
/// Lifted out of `cmd_init` when adding the D0225 step pushed it past the line limit - it is
/// narration, not logic, and it is the FIRST thing a newcomer reads.
fn print_init_next_steps(dir: &Path, count: u32, profile: &str) {
    println!("Scaffolded the engine into {} ({count} engine file(s)). Adoption profile: {profile} (declared).", dir.display());
    println!();
    println!("Next:");
    println!("  1. cd {}", dir.display());
    // issue278: this step used to read `git init && git config core.hooksPath .githooks`. Followed
    // literally from inside a workspace peer — which step 1 has just put the reader in — `git init`
    // creates a NESTED repository and destroys the workspace: the peer stops being part of the repo
    // whose hook and push cover it. `install_commit_gate` now arms the gate against the repository
    // root at init time, so the step is a check rather than an instruction to run blind.
    if std::path::Path::new(".git").exists() || dir.join(".git").exists() {
        println!("  2. The commit gate is already armed (see above). Confirm: git config core.hooksPath");
    } else {
        println!("  2. `git init` AT THE REPOSITORY ROOT — not inside this project if it is one of");
        println!("     several in a shared repo, where a nested repo would take it out of the workspace.");
        println!("     Then re-run `keel init` here, or arm it by hand: git config core.hooksPath .githooks");
    }
    println!("  3. Read CLAUDE.md — how to work here (text is truth; the AI drives the CLI, you supervise).");
    // D0225: the two are sequenced, not alternatives. `project-onboarding` decides WHICH disciplines
    // this project runs; `introduction` walks the author through running one. Naming only the second
    // is how a project ends up with 25 active processes nobody chose (D0054 - friction is the top risk).
    println!("  4. Run the `project-onboarding` skill — it asks what you are building and charters the");
    println!("     process set on that basis. `keel onboard` reports NOT CHARTERED until you do.");
    println!("  5. Then the `introduction` skill — capture your first need + run your first sprint.");
    println!("     Or: keel orient .   (where things stand)");
    println!();
    println!("The pre-commit gate at the REPOSITORY ROOT runs `keel gate --workspace` — validate + guard +");
    println!("declared rules, for every project the commit touches (Rust-only, no kernel).");
    println!("Engine design rationale is read-only reference in .engine/reference/decisions/;");
    println!("your project authors its OWN decisions fresh in .engine/decisions/.");
}

/// Refuse an `init` target INSIDE an existing project, reporting why (issue275). `true` = refused.
///
/// `keel init sub` under a project used to succeed, and the resulting nested project was invisible to
/// workspace discovery, so it rode out UNGATED. Discovery now finds it, but the layout still leaves
/// overlapping paths with two claimants and is not what any author means. Peers, not tenants.
fn refuse_nested_target(dir: &Path) -> bool {
    let Some(host) = enclosing_project(dir) else { return false };
    eprintln!(
        "error: {} is inside the keel project at {} — refusing to nest one project inside another.",
        dir.display(),
        host.display()
    );
    eprintln!("  A workspace holds projects as PEERS: create the directory beside the existing");
    eprintln!("  project, not under it. `keel projects` lists what this repository already holds.");
    // The host may BE the repository root, in which case there is no "beside" inside this repo at
    // all. Say so rather than leaving the author to discover it by trying: a workspace is peers in
    // subdirectories with no project at the root, which is the layout the request behind D0234
    // described — "a parent folder repo, then separate keel projects in subfolders".
    if host.join(".git").exists() {
        eprintln!();
        eprintln!("  {} is the REPOSITORY ROOT, so this repo has no room for a peer:", host.display());
        eprintln!("  a workspace is projects in subdirectories with none at the root. Either");
        eprintln!("    - give this project its own repository (usually the right answer), or");
        eprintln!("    - move the existing project into a subdirectory first, then init peers beside it.");
    }
    true
}

/// The nearest ANCESTOR of `dir` that is already a keel project, if any (issue275).
///
/// Strictly an ancestor: `dir` itself is handled by the `.engine/` refusal above, which reports the
/// overwrite case with its own message. Walks up rather than consulting `workspace::discover`,
/// because the target may not exist yet and may sit outside any git repository — neither of which
/// stops it from being inside a project.
fn enclosing_project(dir: &Path) -> Option<PathBuf> {
    // ABSOLUTISE FIRST. `init`'s target normally does not exist yet — that is the point of init — so
    // `canonicalize` fails and returns the argument unchanged. For a relative target like `sub`, its
    // parent is then the EMPTY path, and `is_project("")` silently tests the PROCESS's current
    // directory rather than the target's parent: the refusal printed the host project as an empty
    // string, and from another cwd it would have answered about a different directory entirely.
    //
    // `workspace::canon` rather than a bare canonicalize because the raw form carries the `\?\`
    // extended-length prefix on Windows, and this path is printed to the author.
    let abs = if dir.is_absolute() {
        dir.to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_default().join(dir)
    };
    let start = keel_cli::workspace::canon(&abs);
    let mut cur = start.parent();
    while let Some(p) = cur {
        if keel_cli::workspace::is_project(p) {
            return Some(p.to_path_buf());
        }
        cur = p.parent();
    }
    None
}

/// D0251 clause B: ship the wrapper into a fresh project. `keelw` resolves the project's pin;
/// `keel-wrapper.toml` is the committed checksum contract (entries come from the release page — the
/// starter names the duty rather than inventing hashes). The cache is SEEDED with the running
/// binary, so the fresh project's wrapper works immediately and offline: init itself just proved
/// this binary runs.
///
/// # Errors
/// The CLI exit code when either committed file cannot be written. A failed cache SEED is a note,
/// not an error — the wrapper still works via a checksum entry or a manual install.
fn init_wrapper(dir: &Path) -> Result<(), i32> {
    if let Err(e) = std::fs::write(dir.join("keelw"), include_str!("../../keelw")) {
        eprintln!("error writing keelw: {e}");
        return Err(1);
    }
    let wrapper_toml = format!(
        "# keel-wrapper — per-version, per-platform release-asset SHA-256s the keelw wrapper verifies\n# against (D0251 clause B). NEVER trust-on-first-use: a version with no entry here refuses to\n# download. Entries come from the release page's published checksums; the seeded cache entry in\n# .keel/bin/ covers THIS machine until then.\n[\"{}\"]\n",
        env!("CARGO_PKG_VERSION")
    );
    if let Err(e) = std::fs::write(dir.join("keel-wrapper.toml"), wrapper_toml) {
        eprintln!("error writing keel-wrapper.toml: {e}");
        return Err(1);
    }
    if let Ok(me) = std::env::current_exe() {
        let asset = if cfg!(windows) {
            "keel-windows-x86_64.exe"
        } else if cfg!(target_os = "macos") {
            "keel-macos-aarch64"
        } else {
            "keel-linux-x86_64"
        };
        let cache = dir.join(".keel").join("bin").join(env!("CARGO_PKG_VERSION"));
        let _ = std::fs::create_dir_all(&cache);
        if let Err(e) = std::fs::copy(&me, cache.join(asset)) {
            eprintln!("note: could not seed the wrapper cache ({e}) — keelw will need a checksum entry or a manual install");
        }
    }
    Ok(())
}

fn cmd_init(args: &[String]) -> i32 {
    const USAGE: &str = "keel init DIR [--profile strict|guided]";
    let target = match positional_arg(args, USAGE, "a directory") {
        Ok(a) => a,
        Err(code) => return code,
    };
    let dir = PathBuf::from(target);
    let engine_dst = dir.join(".engine");
    if engine_dst.exists() {
        eprintln!("error: {} already contains a .engine/ — refusing to overwrite", dir.display());
        return 2;
    }
    if refuse_nested_target(&dir) {
        return 2;
    }
    // Adoption profile: DECLARED, never inferred (D0174/P0.4 — the issue089/129 lockout class died
    // of inference). Default applies only to a PROVABLY EMPTY directory (strict: a fresh scaffold
    // starts green, init_smoke proves it); any directory with existing content requires the flag.
    let profile = match flag(args, "profile") {
        Some(p) if p == "strict" || p == "guided" => p,
        Some(p) => {
            eprintln!("error: --profile takes `strict` or `guided`, not `{p}`.");
            eprintln!("usage: {USAGE}");
            return 2;
        }
        None => {
            let has_content =
                std::fs::read_dir(&dir).is_ok_and(|rd| rd.flatten().any(|e| e.file_name() != ".git"));
            if has_content {
                eprintln!("error: {} has existing content — an adoption profile must be DECLARED, never inferred (D0174).", dir.display());
                eprintln!("  --profile strict   blocking in-loop gates from day one (fresh projects)");
                eprintln!("  --profile guided   advisory-first; promote to blocking later, citing measured evidence (D0180)");
                eprintln!("usage: {USAGE}");
                return 2;
            }
            "strict".to_string()
        }
    };
    let mut count = 0u32;
    if let Err(e) = scaffold_engine(&ENGINE_DIR, &engine_dst, &mut count) {
        eprintln!("error scaffolding engine: {e}");
        return 1;
    }
    // Empty .engine/decisions/ — where the NEW project authors its own decisions (the engine's ship
    // as read-only reference under .engine/reference/decisions/).
    if let Err(e) = std::fs::create_dir_all(engine_dst.join("decisions")) {
        eprintln!("error creating .engine/decisions: {e}");
        return 1;
    }
    if let Err(e) = std::fs::write(dir.join("CLAUDE.md"), CLAUDE_MD) {
        eprintln!("error writing CLAUDE.md: {e}");
        return 1;
    }
    // A .gitignore, because the MACHINE-LOCAL files must never be committed and nothing was
    // stopping them. Found by a two-clone test (sprint 300): both contributors' `.keel/actor`
    // bindings landed in git and then CONFLICTED on merge — each clone claiming to be the other's
    // identity. `actor.rs` has always said the binding is per-machine and "committing it would
    // re-create the shared-default defect it exists to remove"; nothing enforced it downstream,
    // because `keel init` scaffolded no ignore file at all.
    if let Err(e) = std::fs::write(dir.join(".gitignore"), GITIGNORE) {
        eprintln!("error writing .gitignore: {e}");
        return 1;
    }
    let tracking = dir.join(".tracking");
    if let Err(e) = std::fs::create_dir_all(&tracking) {
        eprintln!("error creating .tracking: {e}");
        return 1;
    }
    if let Err(e) = std::fs::write(tracking.join("README.md"), TRACKING_STARTER) {
        eprintln!("error writing .tracking/README.md: {e}");
        return 1;
    }
    // A starter actor registry so the newcomer's first recorded fact (createdBy/judgedBy) passes the
    // actors guard (D0037) — they edit it to their real identities.
    if let Err(e) = std::fs::write(tracking.join("actors.sysml"), STARTER_ACTORS) {
        eprintln!("error writing .tracking/actors.sysml: {e}");
        return 1;
    }
    // Scaffold a RUST-ONLY commit gate so the project has an automated gate from day one — no
    // conda/kernel (D0048).
    //
    // The hook belongs to the REPOSITORY, not the project (issue278). git allows one
    // `core.hooksPath` per repository, so a hook written inside a project directory can never be
    // invoked for a sibling — and `git init` followed by two `keel init`s left no repo-root hook and
    // no hooksPath at all, so the whole workspace was UNGATED and the subsequent commit ran with zero
    // pre-commit lines. Verified before the fix.
    let repo_root = keel_cli::gitx::git()
        .arg("-C")
        .arg(&dir)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| PathBuf::from(String::from_utf8_lossy(&o.stdout).trim()))
        .filter(|p| p.is_dir())
        .map_or_else(|| dir.clone(), |p| keel_cli::workspace::canon(&p));
    if let Err(code) = install_commit_gate(&repo_root, &dir) {
        return code;
    }
    if let Err(code) = init_enforcement_surface(&dir, &engine_dst, &profile) {
        return code;
    }
    // D0190: stamp the declared engine version - which binary's checks this engine is defined
    // against. Re-stamped by `keel migrate`; read only by the parity warning, never by migrate.
    let version_toml = format!(
        "# engine-version - the BINDING engine pin: the version whose writes and gates this project accepts\n# (D0190 stamped it; D0251 made it bite). Written by `keel init`, re-stamped by `keel migrate`; a\n# mismatched binary refuses writes and gates, warns on reads. keelw resolves this pin (D0251 B).\nengine = \"{}\"\n",
        env!("CARGO_PKG_VERSION")
    );
    if let Err(code) = init_wrapper(&dir) {
        return code;
    }
    if let Err(e) = std::fs::write(engine_dst.join("contracts").join("engine-version.toml"), version_toml) {
        eprintln!("error writing engine-version.toml: {e}");
        return 1;
    }
    print_init_next_steps(&dir, count, &profile);
    0
}

/// Header kept at the top of a generated `activation.toml` — the file must explain itself, because the
/// consequence of editing it wrongly (a control silently off) is not obvious from its contents.
const ACTIVATION_HEADER: &str = "\
# Process activation (D0138) — which processes THIS project has adopted.
#
# What activating a process does: turns on its whole unit (skill + declared rules + guards), as defined
# by the engine from each process's own `assert constraint` declarations. Deactivating one stops its guards running,
# and `keel guard` then REPORTS each as NOT ACTIVE rather than skipping it silently.
#
# DELETE THIS FILE to return to \"everything is active\", which is also the behaviour when no file
# exists — so an existing project that never declares one is unaffected.
#
# CORE guards (identity, provenance, vocabulary, rootedness, well-formedness) are in no unit and CANNOT
# be deactivated here. Activation exists to stop enforcing procedures you have not adopted; it is not a
# switch that makes truthfulness optional.
#
# Edit by hand, or use `keel activate <process>` / `keel deactivate <process>`.
";

/// Write `activation.toml` with both active sets stated exactly.
///
/// BOTH sections are always written, even when one is unchanged (D0164). Writing only the section being
/// edited would leave the other absent, and absent means EVERYTHING ACTIVE — so deactivating one
/// viewpoint would silently re-activate every process the project had turned off. A partial write of a
/// contract whose absence has meaning is a data-loss bug, not a convenience.
fn write_activation(root: &Path, processes: &[String], viewpoints: &[String]) -> std::io::Result<()> {
    // JOIN `.engine/contracts` HERE, as the original did. Taking a pre-joined directory instead was my
    // regression and it wrote the manifest to the repo root, where `Activation::load` never looks - so
    // `keel deactivate` reported success and changed nothing. Caught by round-tripping the command
    // against the surfaces it is supposed to affect rather than by reading its output, which said
    // "deactivated" either way.
    let dir = root.join(".engine/contracts");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(
        dir.join("activation.toml"),
        format!(
            "{ACTIVATION_HEADER}
[processes]
active = [{}]

[viewpoints]
active = [{}]
",
            processes.iter().map(|p| format!("\"{p}\"")).collect::<Vec<_>>().join(", "),
            viewpoints.iter().map(|v| format!("\"{v}\"")).collect::<Vec<_>>().join(", "),
        ),
    )
}

/// Activate or deactivate one VIEWPOINT, writing both manifest sections (D0164).
///
/// Split out of `cmd_activation` to keep that function within the line budget, and because the two
/// namespaces genuinely differ: a process switch turns guards on and off, a viewpoint switch turns a LENS
/// on and off. Bundling them into one branchy function hid that.
fn switch_viewpoint(
    root: &Path,
    act: &keel_cli::activation::Activation,
    mode: &str,
    target: &str,
    all: &[String],
    mut set: Vec<String>,
) -> i32 {
    let materialising = act.active_viewpoints.is_none();
    if mode == "activate" {
        if !set.iter().any(|v| v == target) {
            set.push(target.to_string());
        }
    } else {
        set.retain(|v| v != target);
    }
    set.sort();
    // The PROCESS section must be rewritten too, unchanged: absence means everything active, so omitting
    // it would silently re-activate every process the project had turned off.
    let procs: Vec<String> = act.unit_names().into_iter().filter(|p| act.is_process_active(p)).collect();
    if let Err(e) = write_activation(root, &procs, &set) {
        eprintln!("error writing .engine/contracts/activation.toml: {e}");
        return 1;
    }
    if materialising {
        println!("No viewpoint manifest existed (all were active), so one was written with the current");
        println!("effective state before applying this change.");
    }
    println!("{mode}d viewpoint `{target}`. Active viewpoints: {} of {}", set.len(), all.len());
    println!("Read it back: keel activation | keel guard");
    0
}

/// `keel activate <process>` / `keel deactivate <process>` / `keel activation` (D0138).
///
/// The subtle case is activating when NO manifest exists. Absence means "everything is active", so
/// writing a manifest containing only the named process would silently DEACTIVATE every other control —
/// the opposite of what the caller asked for. So a first write MATERIALISES the current effective state
/// (all declared units) and then applies the change, and says that it did.
fn cmd_activation(mode: &str, args: &[String]) -> i32 {
    // issue179: `activate`/`deactivate` take a process or viewpoint NAME; a flag would be looked up as
    // one and reported as unknown, which reads as "no such process" rather than "unrecognised flag".
    if let Some(f) = args.first().filter(|a| a.starts_with('-')) {
        eprintln!("error: `{f}` looks like a flag, not a process or viewpoint name (issue179).");
        return 2;
    }
    let (target, root_arg) = match mode {
        "activation" => (None, args.first()),
        _ => (args.first(), args.get(1)),
    };
    let Some(root) = resolve_guard_root(root_arg.map(String::from).as_ref()) else {
        eprintln!("error: no .engine/ directory found. usage: keel {mode} [<process>] [ROOT]");
        return 2;
    };
    let act = keel_cli::activation::Activation::load(&root);

    if mode == "activation" {
        println!("declared manifest: {}", if act.is_declared() { "yes" } else { "no — everything present is active" });
        // EVERY declared process, not only the switchable ones (issue149): listing 6 of 18 with no note
        // that the rest exist reads as "this project has 6 processes".
        for p in keel_cli::activation::declared_processes(&root) {
            match act.unit(&p) {
                Some(unit) => println!(
                    "  [{}] {p}  ({} guard(s))",
                    if act.is_process_active(&p) { "active  " } else { "INACTIVE" },
                    unit.guards.len()
                ),
                None => println!("  [always  ] {p}  (asserts no guard — nothing to switch off)"),
            }
        }
        // VIEWPOINTS (D0164), listed alongside processes because the human's direction was that they be
        // switchable "just like processes" - and a switch whose state nobody can see is not a switch.
        let vps = keel_cli::view::declared_viewpoints(&root).unwrap_or_default();
        println!("
viewpoints ({} declared):", vps.len());
        for vp in &vps {
            println!(
                "  [{}] {}",
                if act.is_viewpoint_active(&vp.name) { "active  " } else { "INACTIVE" },
                vp.name
            );
        }
        println!("\ncore guards (never deactivatable):");
        for g in keel_cli::guards::GUARD_NAMES {
            if act.guard_state(g) == keel_cli::activation::GuardState::Core {
                println!("  {g}");
            }
        }
        return 0;
    }

    let Some(target) = target else {
        eprintln!("usage: keel {mode} <process> [ROOT]");
        return 2;
    };
    // A name may be a process or a VIEWPOINT (D0164). Resolve in that order and refuse an ambiguous name
    // rather than guessing: switching off the wrong thing is worse than asking again.
    let vp_names: Vec<String> =
        keel_cli::view::declared_viewpoints(&root).unwrap_or_default().into_iter().map(|v| v.name).collect();
    let vp_active: Vec<String> = vp_names.iter().filter(|v| act.is_viewpoint_active(v)).cloned().collect();
    if vp_names.iter().any(|v| v == target) && act.unit(target).is_some() {
        eprintln!("error: `{target}` names both a process unit and a viewpoint - rename one; keel will not guess which to {mode}");
        return 2;
    }
    if vp_names.iter().any(|v| v == target) {
        return switch_viewpoint(&root, &act, mode, target, &vp_names, vp_active);
    }
    if act.unit(target).is_none() {
        // Say WHICH of the two cases this is (issue149). "Not a declared process unit" was true and
        // read as "no such process", which is a different and wrong answer.
        if keel_cli::activation::declared_processes(&root).iter().any(|p| p == target) {
            eprintln!(
                // issue242: this text used to end "or give it an `assert constraint` so it becomes
                // switchable" -- advising the exact edit that CAPTURES a core guard and disarms it.
                // Adding an assert is a process-definition change under the keystone and is now
                // caught by audit-adherence as a Core -> Active/Inactive weakening; the CLI must not
                // recommend it as a convenience.
                "error: `{target}` is a declared process but asserts no guard, so there is nothing for {mode} to switch. Activation governs GUARDS (D0138). To stop running this process, remove the facts it authors -- a process whose inputs are absent produces nothing. Do NOT add an `assert constraint` to make it switchable: claiming a guard converts it from CORE to that process's switchable property, which DISARMS it (issue242), and audit-adherence gates that transition."
            );
        } else {
            eprintln!(
                "error: `{target}` is not a declared process. Declared: {}",
                keel_cli::activation::declared_processes(&root).join(", ")
            );
        }
        return 2;
    }

    let materialising = !act.is_declared();
    let mut set: Vec<String> = act.unit_names().into_iter().filter(|p| act.is_process_active(p)).collect();
    match mode {
        "activate" => {
            if !set.iter().any(|p| p == target) {
                set.push(target.clone());
            }
        }
        _ => set.retain(|p| p != target),
    }
    set.sort();
    if let Err(e) = write_activation(&root, &set, &vp_active) {
        eprintln!("error writing .engine/contracts/activation.toml: {e}");
        return 1;
    }
    if materialising {
        println!(
            "No manifest existed (everything was active), so one was written with the current effective\nstate before applying the change — behaviour is otherwise unchanged."
        );
    }
    println!("{mode}d `{target}`. Active: {}", if set.is_empty() { "(none)".to_string() } else { set.join(", ") });
    println!("Read it back: keel activation | keel guard");
    0
}

/// `keel version` (also `--version` / `-V`) — report which build this is.
///
/// Exists because a downstream project could not answer "am I running the fix?": three versioned
/// releases shipped with no version identification in the artifact, so a project still on the blocked
/// version was INDISTINGUISHABLE from one that had upgraded. Reports the release version, the build
/// commit (baked by `build.rs`; `unknown` off-git, `+dirty` from a modified tree — never guessed), and
/// the CONTROL INVENTORY this binary carries, computed from `GUARD_NAMES` rather than restated, so it
/// cannot drift from the guards that actually run.
/// `keel migrate [ROOT] [--dry-run]` — bring a DOWNSTREAM project up to this binary's engine vintage.
///
/// Deliberately NOT defaulted to the discovered repo root: `find_repo_root` walks upward looking for
/// `.engine/`, which from inside a downstream project's subdirectory is right, but from anywhere in
/// the self-build repo would point this at the engine SOURCE. `migrate` refuses the self-build repo
/// anyway, but a command that rewrites authored facts should take its target explicitly.
fn cmd_migrate(args: &[String]) -> i32 {
    let dry_run = args.iter().any(|a| a == "--dry-run");
    let root = match root_arg(args, "keel migrate [--dry-run] [ROOT]", &["dry-run"], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    keel_cli::migrate::cmd(&root, &ENGINE_DIR, dry_run)
}

/// `keel record issue` — the sanctioned path for D0108 clause 5, which mandates that conflicting
/// conclusions be recorded as an Issue for human adjudication and had no implementation.
///
/// REFUSES on an unknown `--resolver` rather than authoring a dangling edge: the `issues` guard is
/// satisfied by the PRESENCE of a `#Resolves` edge, so an edge pointing at nothing would read as
/// triaged while resolving nothing — exactly the phantom issue109 found twice in this repo.
fn cmd_record_issue(args: &[String]) -> i32 {
    let root = flag(args, "root").map_or_else(|| find_repo_root().unwrap_or_else(|| PathBuf::from(".")), PathBuf::from);
    let req = |n: &str| flag(args, n);
    let (Some(title), Some(description), Some(severity), Some(resolver)) =
        (req("title"), req("description"), req("severity"), req("resolver"))
    else {
        eprintln!("error: --title --description --severity --resolver are all required.");
        eprintln!("  --resolver names the EXISTING item that resolves this issue. It is required because the");
        eprintln!("  `issues` guard fails on an untriaged Issue, so recording one without triage would hand you");
        eprintln!("  a red gate as the command's output (D0077).");
        return 2;
    };
    if !["Critical", "High", "Medium", "Low"].contains(&severity.as_str()) {
        eprintln!("error: --severity must be Critical | High | Medium | Low (got '{severity}')");
        return 2;
    }
    let Some(date) = flag(args, "date").filter(|d| !d.is_empty()) else {
        eprintln!("error: --date YYYY-MM-DD required (when it was found is its own irreducible fact)");
        return 2;
    };
    // NEVER default the actor (D0129/issue072): an unattributable fact looks like evidence.
    let author = match keel_cli::actor::resolve(&root, flag(args, "by").as_deref()) {
        Ok(a) => a,
        Err(msg) => { eprintln!("{msg}"); return 2; }
    };
    match keel_cli::view::item_exists(&root, &resolver) {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("error: resolver '{resolver}' is declared nowhere in the model.");
            eprintln!("  Authoring the edge anyway would make this Issue read as TRIAGED by something that does not");
            eprintln!("  exist (issue109). Declare the resolving action or Decision first, then re-run.");
            return 2;
        }
        Err(e) => { eprintln!("error: cannot read the model to check the resolver: {e}"); return 2; }
    }
    // Bound to locals so the borrows outlive the struct; the inline form silently collapsed both to
    // None (they type-checked and were wrong — a flag the caller passed would have been dropped).
    let related_task = flag(args, "related-task");
    let marker = flag(args, "marker");
    let n = keel_cli::write::NewIssue {
        title: &title,
        description: &description,
        severity: &severity,
        resolver: &resolver,
        related_task: related_task.as_deref(),
        date: &date,
        author: &author,
        marker: marker.as_deref(),
        in_field: args.iter().any(|a| a == "--in-field"),
    };
    match keel_cli::write::record_issue(&root, &n) {
        Ok((name, path)) => {
            println!("recorded {name} -> {path}");
            println!("  triaged on arrival: `#Resolves dependency from {resolver} to {name};`");
            println!("  run `keel validate . && keel guard .` to confirm; nothing was committed.");
            0
        }
        Err(e) => { eprintln!("error: {e}"); 1 }
    }
}

/// `keel accept <decision> --note "<the human's words>" --by <humanActor> --date YYYY-MM-DD`
///
/// THE SINGLE HUMAN GATE HAD NO CLI. `write::accept_decision` existed and was reachable only through
/// `keel serve`'s HTTP API, so recording the one attestation this engine treats as irreducibly human
/// required either running a web server or hand-editing the decision file. That is the same defect
/// `record issue` had (sprint 291): a mandated path with no implementation is the friction that
/// guarantees non-compliance (D0054), and here it applies to the gate the whole autonomous loop
/// pauses for.
///
/// # This command records a human's word; it cannot create one
///
/// `--note` must carry what the human actually said, and `--by` must name a `Person`. The
/// `confirmation-authenticity` guard independently checks that the acceptance result is judged by a
/// Person-typed actor, so an AI accepting its own proposal fails the gate rather than passing it —
/// this command makes the honest path easy without making the dishonest one possible.
fn cmd_accept(args: &[String]) -> i32 {
    // Channel layer (D0178/P1.3, best-effort by recorded design): in a session bearing
    // agent-environment markers, `keel accept` requires a TTY-interactive human or the console
    // approve queue - the actor binding alone is agent-mutable state. The write layer (AI-kind
    // refusal) and the tree-derived audit are the real controls; this is the friction layer.
    {
        use std::io::IsTerminal as _;
        let agent_marked = ["CLAUDECODE", "CLAUDE_CODE_ENTRYPOINT", "CLAUDE_CODE_SESSION_ID", "CLAUDE_CODE_BRIDGE_SESSION_ID"]
            .iter()
            .any(|k| std::env::var(k).is_ok_and(|v| !v.is_empty()));
        if agent_marked && !std::io::stdin().is_terminal() {
            eprintln!("keel accept: this session carries agent-environment markers and no interactive terminal (D0178/K6).");
            eprintln!("  Acceptance is the human's own act: run `keel accept` from YOUR terminal, or accept from the console approve queue / the deck.");
            return 1;
        }
    }
    let root = find_repo_root().unwrap_or_else(|| PathBuf::from("."));
    let Some(decision) = args.first().filter(|a| !a.starts_with('-')) else {
        eprintln!("usage: keel accept <decision> --note \"<what the human said>\" --by <humanActor> --date YYYY-MM-DD");
        eprintln!();
        eprintln!("Records a HUMAN's acceptance of a proposed Decision (D0106). The note must be what they");
        eprintln!("actually said — it IS the attestation, and `confirmation-authenticity` independently checks");
        eprintln!("that `--by` names a Person, so this cannot be used to self-accept an AI's own proposal.");
        return 2;
    };
    let (Some(note), Some(date)) = (flag(args, "note"), flag(args, "date")) else {
        eprintln!("error: --note and --date are both required. The note is the attestation; the date is when it was given.");
        return 2;
    };
    let judged_by = match keel_cli::actor::resolve(&root, flag(args, "by").as_deref()) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("{msg}");
            return 2;
        }
    };
    // Find the decision's file rather than making the caller supply it: a path argument here is a
    // chance to accept the wrong file, and the name is unambiguous.
    let mut found = None;
    for p in keel_cli::collect_sysml(&root.join(".engine").join("decisions")) {
        if std::fs::read_to_string(&p).is_ok_and(|t| t.contains(&format!("part {decision} : Decision"))) {
            found = Some(p);
            break;
        }
    }
    let Some(path) = found else {
        eprintln!("error: no Decision '{decision}' under .engine/decisions/.");
        return 2;
    };
    let sha = keel_cli::gitx::git()
        .arg("-C")
        .arg(&root)
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_owned())
        .unwrap_or_default();
    match keel_cli::write::accept_decision(&path, decision, &sha, &date, &judged_by, &note) {
        Ok(_) => {
            println!("accepted {decision} (judged by {judged_by} at {date}, against {sha})");
            println!("  -> {}", path.strip_prefix(&root).unwrap_or(&path).display().to_string().replace('\\', "/"));
            println!("  run `keel validate . && keel guard .` — confirmation-authenticity checks that {judged_by} is a Person.");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `keel enroll` — the trailing ROOT is only honoured when it actually looks like a keel project,
/// so a stray argument cannot silently redirect an enrollment into the wrong tree.
fn cmd_enroll(rest: &[String]) -> i32 {
    let root = rest
        .iter()
        .rev()
        .find(|a| !a.starts_with("--"))
        .filter(|a| Path::new(a.as_str()).join(".tracking").is_dir())
        .map_or_else(|| find_repo_root().unwrap_or_else(|| PathBuf::from(".")), PathBuf::from);
    keel_cli::enroll::cmd(rest, &root)
}

/// The subcommand catalogue, printed when no arm matches.
///
/// Extracted from `main` because a dispatcher whose fallback is thirty lines of `eprintln!` makes
/// the DISPATCH unreadable — the lint that forced this out is doing the right job.
/// The subcommand catalogue: ONE table, printed here and asserted COMPLETE by
/// `hardening::tests::every_dispatched_subcommand_is_documented`.
///
/// It used to be a wall of `eprintln!` beside a hand-maintained `match`, and the two drifted in
/// one direction only: 35 of 75 subcommands were dispatched and never described (issue172). A
/// table plus a test is not full derivation — clap would be — but it makes the drift impossible
/// to land rather than merely unlikely.
const CATALOGUE: &[&str] = &[
    "  version | --version [--json] which build is this — release version + build commit + guard inventory",
    "  init DIR                     scaffold the engine into a NEW project (D0093 cold start)",
    "  sync [ROOT]                  fetch, report divergence, integrate by MERGE, gate the result (D0129)",
    "  land [ROOT]                  push; on rejection integrate and retry, bounded. Never rewrites history",
    "  claim <item> | --list | --mine   take/inspect a work claim; liveness is COMPUTED (D0147)",
    "  verification [--pending]         EXAMINED vs EXERCISED, split — never one number",
    "  audit-history [--since REF] [--max N]  re-derive the gate verdict per commit (issue116)",
    "  arch <elements|criticality|coupling|drift|stpa-inputs|coverage>",
    "                                   computed views over an AUTHORED CodeElement registry (D0148)",
    "  enroll --actor I --name N --kind human|ai   enroll a contributor: register, bind, verify the gate (D0129)",
    "  migrate [ROOT] [--dry-run]   bring an EXISTING project's .engine/.tracking up to this binary's vintage",
    "  process list|search|show|export|import   the process catalogue: a process is a movable UNIT (D0128)",
    "  onboard [ROOT] [--json]      has this project chosen its processes, and on what basis? each process's declared APPLIES-WHEN + whether the set is chartered (D0225)",
    "  adoption-check [ROOT] [--unit N] [--keep]   gate a FOREIGN tree: every unit must land clean in a project that lacks it, AND that project must gate clean WITHOUT it (issue264)",
    "  attestation [ROOT] [--json]  is a `pass` a receipt or a testimony? results by judge kind, how many EXERCISED claims record what produced them, and the fail rate (D0232)",
    "  recall --prompt -             seed recall from a PROMPT on stdin and print a budgeted, content-bearing brief; zero model calls (D0242/D0243)",
    "  library init|sync|list       the machine-local cache of your portable-content repository - fast-forward-only, stated staleness, availability is never activation (D0250)",
    "  record statement|story        intake's write path: a human's words VERBATIM, then the story that translates them with its #DerivedFrom edge authored alongside (D0236)",
    "  projects [ROOT] [--json]     every keel project in this git repository, and which one you are in - a workspace (D0234)",
    "  activation [ROOT]            which processes this project has ADOPTED, and which guards are core (D0138)",
    "  activate|deactivate PROCESS  adopt/drop a process as a UNIT — skill + rules + guards in one step",
    "  serve [--port N] [ROOT]      the interactive console — localhost read dashboard (D0094 m1)",
    "  validate [ROOT]              semantic-validate all .tracking/ files",
    "  check FILE...                parse-check one or more .sysml files",
    "  check --spec-version         report the baked grammar version vs upstream (--no-fetch to skip the live check)",
    "  ls [ROOT]                    list .tracking/ .sysml files",
    "  orient [ROOT] [--html]       orient state as JSON, or --html = the human dashboard #View (D0093)",
    "  whats-next [ROOT]            print ready task names (one per line)",
    "  github-gesture               parse a channel comment (COMMENT_BODY/ISSUE_BODY/COMMENT_ID/COMMENT_BODIES by ENV, never argv) -> JSON verdict/option/reason/decision; exit 1 if unparsed (D0221)",
    "  github-decider [<login>]     who may decide on the GitHub decision channel; no arg lists them (D0219). An unmapped login is refused, never defaulted",
    "  github-decision-id <id>      split a channel decision id into project<TAB>name - `alpha/d0001` in a workspace, `d0001` alone (D0234)",
    "  advance <sprint> [--to G]    process cursor: the sprint's current ceremony step; --to is refused until earlier steps' verify-Tests pass (D0209 clause 3)",
    "  actor-trace <actor> [ROOT]   everything an actor authored / judged / owns — computed from provenance (issue106)",
    "  assumptions [ROOT]           accepted-but-unverified items something DEPENDS on — computed, never authored (issue105)",
    "  marker-census [ROOT]         per-marker EDGE count (the migration control total) vs prose mentions (issue099)",
    "  diagram [ROOT]               whole-model interactive graph HTML (D0085; redirect to .html)",
    "  render <view> [--mode graph|table|review]  render any declared view as HTML (D0086)",
    "  apply-review --batch F [--sha S] [--judged-by A] [--judged-at D]  write a review batch back as linked critiques (D0086)",
    "  append-result --file F --task T --sha S [--verdict pass|fail] [--judged-by A] [--judged-at D]",
    "  append-gate-result --file F --gate G --sha S [--verdict pass|fail] [--judged-by A] [--judged-at D]",
    "  accept <decision> --note \"<what the human said>\" --by <humanActor> --date YYYY-MM-DD   record a HUMAN acceptance (D0106)",
    "  add-task --file F --def D --task T --dod TEXT [--method test|inspect|confirmation|demo|analysis]",
    "assured [ROOT]               composite READY/NOT-READY assurance verdict + per-check detail (D0079)",
    "audit [ROOT]                 retrospective adherence: charter, ceremony, estimation, sitting review",
    "audit-history [--since REF] [--max N]  re-derive the gate verdict per commit (issue116)",
    "audit-adherence [--since REF]  re-derive guard-set/severity monotonicity per commit - a control cannot be disarmed unsigned (D0209)",
    "hardening [ROOT]             the critique process's own questions, computed (issue171/D0169)",
    "deck [ROOT] [--out FILE]    the mobile obligation deck - served at /deck by keel serve, saving via this API (issue192)",
    "  mint [N]                     engine-minted v4 UUIDs, one per line - identity is never hand-authored (us019)",
    "  new sprint <N> <slug> --charter <dNNNN> [--points P]   scaffold the ceremony record - ids minted, placeholders the fast gate rejects",
    "  sync-claude [ROOT] [--check]   regenerate the keel-owned .claude/ enforcement surface in place; --check reports drift (D0174)",
    "  override <path> --reason R   arm a single-use, path-bound write unlock; consumption records an obligation (D0176)",
    "  enforcement-report [ROOT]    fires/blocks/overrides/red-yields from the fire-ledger - promotions cite this (D0180)",
    "  decision-follow-through [ROOT] [--table]   every accepted Decision's downstream tracked items + evidence, and the gaps (us020)",
    "check-engine [ROOT]          .engine instance reference resolution, kernel-free (D0112 phase 2)",
    "hook post-edit|stop|pre-bash|pre-write|subagent-stop|user-prompt   the in-loop gates, in the binary (D0134/D0174)",
    "reverify [--all-drift|--task N] [--by A]  re-run the declared gate at HEAD; stamp fresh results (D0101)",
    "suspect [ROOT]               done work whose evidence has DRIFTED from the tree it was judged against",
    "outstanding [ROOT]           every not-done item, flat — the burndown without the ranking",
    "orphans [ROOT]               items nothing references: tasks with no DoD, issues with no resolver",
    "recent [ROOT]                git-derived activity timeline: commits touching .tracking/.engine (sr15)",
    "intake [ROOT]                statements -> user stories -> routing: unparsed, unrouted, unsourced (D0166)",
    "dispositions [ROOT]          findings by verdict — act / acceptRisk / dismiss / undispositioned (D0165)",
    "authority-queue [ROOT]       what awaits a HUMAN's authority, and what may not be self-attested",
    "attestation-coverage [ROOT]  accepted Decisions lacking a passing acceptance result (D0066)",
    "open-issues [ROOT]           every OPEN issue + its resolvers + whether each resolver is complete (D0077)",
    "indicators [ROOT]            MONITORED with no enforced threshold — never gated (invariant 7)",
    "snapshot-indicators [ROOT]   stamp the current indicator values as a dated series point",
    "record-measurement --indicator I --value V [--at DATE]  add one measurement to an indicator series",
    "rootedness [ROOT]            charter-source burndown: need-rooted vs decision-chartered vs orphan (D0099)",
    "tier-satisfaction [ROOT]     per tier, the fraction cleanly satisfied downstream (Needs -> SRs -> tests)",
    "concern-coverage [ROOT]      declared viewpoints vs stakeholder concerns — which concerns nothing serves",
    "critique-coverage [ROOT]     per-element required-lens matrix + the gap set (D0097)",
    "critique-policy [ROOT]       which antagonistic lenses each assurance-element type REQUIRES (D0097)",
    "sitting-coverage [ROOT]      per-sitting human review currency, grandfathered at D0155's line",
    "governing-version <item> [ROOT]  which process version governs this item",
    "decisions [ROOT]             load-bearing decisions, ranked by how much depends on them",
    "business [ROOT]              the what/why layer: Brief -> Personas -> Needs -> UseCases (D0107)",
    "launchables [ROOT]           the console's launchable set, computed from declared skills + processes",
    "workflows [ROOT]             the six workflows and their phases",
    "contentions [ROOT]           recorded disagreements between contributors awaiting adjudication (D0108)",
    "controls [ROOT]              the two-way hazard/control diff: uncovered failure conditions + unanchored controls (D0195)",
    "decision-card [NAME] [--proposed]  a decision's deciding context as JSON - the githubChannel issue body source (D0205)",
    "why <term> [ROOT]            answer from the model as a graph: seed on names/aliases, traverse, cite provenance + failing critiques (D0161)",
    "knowledge question-coverage [ROOT]  per declared Question: does seeding find an entity and traversal reach an answer (D0161)",
    "trace <item> [ROOT]          every typed edge reaching an item, both directions",
    "trace-need <need> [ROOT]     one Need's full satisfaction chain down to test results",
    "boundary <element> [ROOT]    one element's interface surface — takes an ELEMENT, not a root",
    "boundary-sweep [ROOT]        tier-satisfaction white-box sweep, per Need-slice",
    "reprocess-candidates [ROOT]  items whose governing process version has moved on since they were judged",
];

/// The subcommand catalogue, printed when no arm matches.
fn print_usage() -> i32 {
    eprintln!("keel <subcommand> [args]");
    for line in CATALOGUE {
        eprintln!("  {line}");
    }
    2
}
/// A read-only view subcommand: run `f` against the repo root and print its JSON, or the error as
/// JSON so a consumer parsing stdout gets a parseable answer either way.
/// `keel decision-card [NAME] [--proposed]` (D0205): machine-readable deciding context.
fn cmd_decision_card(rest: &[String]) -> i32 {
    let name = rest.first().filter(|a| !a.starts_with('-')).map(String::as_str);
    let proposed = rest.iter().any(|a| a == "--proposed");
    let root = find_repo_root().unwrap_or_else(|| PathBuf::from("."));
    match keel_cli::deck::decision_cards(&root, name, proposed) {
        Ok(s) => {
            println!("{s}");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `keel why <term> [ROOT]` (D0161): seed on names/titles/aliases, traverse, answer with provenance.
/// Hard latency cap for prompt-path recall. Measured at 641-863ms on this repo's 13.6k-item corpus, so
/// the cap leaves headroom while bounding the worst case: a prompt-path cost that grows with the corpus
/// is a tax that compounds silently, and the reader is TOLD when it is hit rather than left to wonder.
const RECALL_CAP_MS: u128 = 2500;

/// Default character budget for a recalled payload. Bounded on purpose: injected context costs the
/// human tokens on every turn, so the cost has to be predictable rather than proportional to how
/// connected the term happens to be (D0242 part 1).
const RECALL_BUDGET: usize = 4000;

fn budget_arg(args: &[String]) -> usize {
    flag(args, "budget").and_then(|v| v.parse().ok()).unwrap_or(RECALL_BUDGET)
}

/// `keel recall --prompt - [--budget N] [ROOT]` — seed from a PROMPT and print a budgeted brief.
///
/// The prompt arrives on STDIN, never as an argument: it is free-form text that may contain quotes,
/// newlines and backticks, and the one thing this path must never do is let that text reach a shell.
fn cmd_recall(rest: &[String]) -> i32 {
    let usage = "keel recall --prompt - [--budget N] [ROOT]   (the prompt arrives on STDIN)";
    if flag(rest, "prompt").as_deref() != Some("-") {
        eprintln!("usage: {usage}");
        eprintln!("  `--prompt -` is required and means: read the prompt from stdin.");
        return 2;
    }
    let positional = without_flag_values(rest, &["prompt", "budget"]);
    let root = match root_arg(&positional, usage, &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    let mut prompt = String::new();
    if std::io::Read::read_to_string(&mut std::io::stdin(), &mut prompt).is_err() {
        eprintln!("recall: could not read the prompt from stdin");
        return 2;
    }
    match keel_cli::view::recall_for_prompt(&root, &prompt, budget_arg(rest)) {
        Ok(s) => {
            print!("{s}");
            0
        }
        Err(e) => {
            eprintln!("recall error: {e}");
            1
        }
    }
}

fn cmd_why(rest: &[String]) -> i32 {
    // `--budget N` consumes N, so the positionals have to be taken from the STRIPPED list or the
    // number becomes the root — measured: `keel why keystone --brief --budget 1500` recalled 0 seeds
    // because it built a model at `./1500`.
    let positional = without_flag_values(rest, &["budget"]);
    let bare: Vec<&String> = positional.iter().filter(|a| !a.starts_with("--")).collect();
    let Some(term) = bare.first() else {
        eprintln!("usage: keel why <term> [ROOT] [--brief] [--budget N]");
        return 2;
    };
    let Some(root) = bare.get(1).map(|p| PathBuf::from(p.as_str())).or_else(find_repo_root) else {
        eprintln!("usage: keel why <term> [ROOT] [--brief] [--budget N]");
        return 2;
    };
    // `--brief` is the INJECTABLE form: budgeted, content-bearing text rather than JSON whose seeds
    // are bare identifiers. The JSON form is unchanged for every existing caller.
    if rest.iter().any(|a| a == "--brief") {
        return match keel_cli::view::why_brief(&root, term, budget_arg(rest)) {
            Ok(s) => {
                print!("{s}");
                0
            }
            Err(e) => {
                eprintln!("why error: {e}");
                1
            }
        };
    }
    match keel_cli::view::why(&root, term) {
        Ok(s) => {
            println!("{s}");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `keel knowledge question-coverage [ROOT]` (D0161): per declared Question, does seeding find an
/// entity and traversal reach an answer. Computed, never authored.
fn cmd_knowledge(rest: &[String]) -> i32 {
    if rest.first().map(String::as_str) != Some("question-coverage") {
        eprintln!("usage: keel knowledge question-coverage [ROOT]");
        return 2;
    }
    cmd_view0(rest.get(1..).unwrap_or(&[]), "knowledge question-coverage", keel_cli::view::question_coverage)
}

fn cmd_view0(
    rest: &[String],
    name: &'static str,
    f: fn(&Path) -> Result<String, keel_cli::view::ViewError>,
) -> i32 {
    // `cmd_query0` takes a fn POINTER, and a closure capturing `f` is not one — so the root is
    // resolved here and the view invoked directly, rather than threading a capture through it.
    let Some(root) = rest.first().map(PathBuf::from).or_else(find_repo_root) else {
        eprintln!("usage: keel {name} [ROOT]");
        return 2;
    };
    // issue281: a zero-argument VIEW must not answer over nothing either. `cmd_view0` resolves
    // its own root rather than going through `root_arg`, so the shared precondition missed it -
    // found by sweeping every command at a workspace root, where `controls` still exited 0.
    if let Err(code) = keel_cli::workspace::require_project(&root, &format!("keel {name} [ROOT]")) {
        return code;
    }
    println!("{}", f(&root).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}")));
    0
}

/// The repo a git-touching subcommand acts on: the first non-flag argument, else the discovered root.
fn repo_arg(rest: &[String]) -> PathBuf {
    rest.iter()
        .find(|a| !a.starts_with("--"))
        .map_or_else(|| find_repo_root().unwrap_or_else(|| PathBuf::from(".")), PathBuf::from)
}

fn cmd_version(args: &[String]) -> i32 {
    let hard = keel_cli::guards::GUARD_NAMES.len() - WARNING_ONLY_GUARDS.len();
    if args.iter().any(|a| a == "--json") {
        println!(
            "{{\"version\":\"{}\",\"buildCommit\":\"{}\",\"guards\":{},\"guardsHardBlocking\":{},\"guardsWarningOnly\":{}}}",
            env!("CARGO_PKG_VERSION"),
            env!("KEEL_BUILD_COMMIT"),
            keel_cli::guards::GUARD_NAMES.len(),
            hard,
            WARNING_ONLY_GUARDS.len(),
        );
        return 0;
    }
    println!("keel {}", env!("CARGO_PKG_VERSION"));
    println!("build commit: {}", env!("KEEL_BUILD_COMMIT"));
    // D0190: the binary version is the ONE declared semver; the others are derived facts reported
    // beside it (a breaking API change is recorded in the release Decision, not versioned apart).
    println!("api contract: {} (derived; breaking changes recorded in release Decisions)", keel_cli::serve::KEEL_API_VERSION);
    println!("claude surface: {} (generated from this binary)", keel_cli::claude_surface::SURFACE_VERSION);
    match find_repo_root().map(|r| engine_version_skew(&r)) {
        Some(Some(w)) => println!("engine declared: SKEW - {w}"),
        Some(None) => println!("engine declared: matches (engine-version.toml or pre-D0190 absent)"),
        None => {}
    }
    println!(
        "guards: {} ({hard} hard-blocking, {} warning-only)",
        keel_cli::guards::GUARD_NAMES.len(),
        WARNING_ONLY_GUARDS.len(),
    );
    0
}

/// The warning-only members of `GUARD_NAMES` — they RUN on every commit and are visible, but never
/// block (the D0102 promote-once-low-noise pattern). Named here so `keel version` can report the
/// hard-vs-warning split without a hand-maintained count.
const WARNING_ONLY_GUARDS: [&str; 9] =
    ["decision-requirement-link", "verification-trace", "priority-inversion", "retro-backlog", "doc-sync", "hook-config-integrity", "sequence-multiplicity", "parser-coverage", "base-first-justification"];

#[allow(clippy::too_many_lines)] // one dispatch table = one place a subcommand can be reached from;
// splitting it by arbitrary length would hide half the surface from anyone reading for what exists
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rest: &[String] = args.get(2..).unwrap_or(&[]);
    let code = match args.get(1).map(String::as_str) {
        // Version must answer BEFORE any root resolution — a caller establishing which binary they
        // have may be standing anywhere, including outside a keel project.
        Some("version" | "--version" | "-V") => cmd_version(rest),
        Some("init") => cmd_init(rest),
        Some("sync") => keel_cli::sync::cmd_sync(&repo_arg(rest)),
        Some("land") => keel_cli::sync::cmd_land(&repo_arg(rest), 3),
        Some("migrate") => cmd_migrate(rest),
        // D0138: what has this project ADOPTED — declared, not inferred from file presence.
        Some("process") => keel_cli::process_cmd::cmd(rest, &find_repo_root().unwrap_or_else(|| PathBuf::from("."))),
        Some("onboard") => keel_cli::onboard::cmd(rest),
        Some("adoption-check") => keel_cli::adoption_check::cmd(rest),
        Some("attestation") => keel_cli::attestation::cmd(rest),
        Some("projects") => keel_cli::workspace::cmd(rest),
        Some(v @ ("activation" | "activate" | "deactivate")) => cmd_activation(v, rest),
        Some("serve") => cmd_serve(rest),
        Some("validate") => cmd_validate(rest),
        Some("hook") => cmd_hook(rest), // D0134: in-loop gates in the BINARY, no python runtime
        Some("gate") => cmd_gate(rest), // D0128 Tier-2: the fast per-edit in-loop gate
        Some("check-engine") => cmd_check_engine(rest),
        Some("check") => cmd_check(rest),
        Some("rules") => cmd_rules(rest),
        Some("business") => cmd_business(rest),
        Some("launchables") => cmd_launchables(rest),
        Some("ls") => cmd_ls(rest),
        Some("library") => keel_cli::library::run(rest),
        Some("orient") => cmd_orient(rest),
        Some("whats-next") => cmd_whats_next(rest),
        Some("view") => cmd_view(rest),
        Some("attestation-coverage") => cmd_attestation_coverage(rest),
        Some("orphans") => cmd_orphans(rest),
        Some("audit") => cmd_audit(rest),
        Some("hardening") => cmd_hardening(rest),
        Some("deck") => cmd_deck(rest),
        Some("mint") => cmd_mint(rest),
        Some("new") => cmd_new(rest),
        Some("sync-claude") => cmd_sync_claude(rest),
        Some("override") => cmd_override(rest),
        Some("enforcement-report") => cmd_enforcement_report(rest),
        Some("decision-follow-through") => cmd_decision_follow_through(rest),
        Some("guard") => cmd_guard(rest),
        Some("governing-version") => cmd_governing_version(rest),
        Some("reprocess-candidates") => cmd_reprocess_candidates(rest),
        Some("suspect") => cmd_suspect(rest),
        Some("open-issues") => cmd_open_issues(rest),
        Some("intake") => cmd_intake(rest),
        Some("dispositions") => cmd_dispositions(rest),
        Some("sitting-coverage") => cmd_sitting_coverage(rest),
        Some("concern-coverage") => cmd_concern_coverage(rest),
        Some("coverage") => cmd_coverage(rest),
        Some("critique-coverage") => cmd_critique_coverage(rest),
        Some("critique-policy") => cmd_critique_policy(rest),
        Some("actor-trace") => cmd_query1(rest, "actor-trace", |r, a| keel_cli::view::actor_trace(r, a).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("assumptions") => cmd_view0(rest, "assumptions", keel_cli::view::assumptions),
        Some("authority-queue") => cmd_view0(rest, "authority-queue", keel_cli::view::authority_queue),
        Some("contentions") => cmd_view0(rest, "contentions", keel_cli::view::contentions),
        Some("controls") => cmd_view0(rest, "controls", keel_cli::view::controls),
        Some("decision-card") => cmd_decision_card(rest),
        Some("why") => cmd_why(rest),
        Some("recall") => cmd_recall(rest),
        Some("knowledge") => cmd_knowledge(rest),
        Some("marker-census") => cmd_view0(rest, "marker-census", keel_cli::view::marker_census),
        Some("rootedness") => cmd_query0(rest, "keel rootedness [ROOT]", |r| keel_cli::view::rootedness(r).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("tier-satisfaction") => cmd_query0(rest, "keel tier-satisfaction [ROOT]", |r| keel_cli::view::tier_satisfaction(r).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("recent") => cmd_query0(rest, "keel recent [ROOT]", |r| keel_cli::view::recent(r).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("boundary") => cmd_query1(rest, "boundary", |r, need| keel_cli::view::boundary_json(r, need).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("boundary-sweep") => cmd_query0(rest, "keel boundary-sweep [ROOT]", |r| keel_cli::view::boundary_sweep_json(r).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("reverify") => cmd_reverify(rest),
        // D0129/issue072: inspect or bind this machine's acting identity (never defaulted).
        Some("actor") => keel_cli::actor::cmd(rest, &find_repo_root().unwrap_or_else(|| PathBuf::from("."))),
        Some("claim") => keel_cli::claim::cmd(rest, &find_repo_root().unwrap_or_else(|| PathBuf::from("."))),
        // `repo_arg(rest)` would take the SUBCOMMAND as the path — `arch elements .` resolved the
        // root to `./elements`, whose empty model then printed "no CodeElement instances authored".
        Some("arch") => keel_cli::arch::cmd(rest, &repo_arg(rest.get(1..).unwrap_or(&[]))),
        // issue281: `verification` reads a MODEL, so it must not answer over nothing — but it takes
        // its root via `repo_arg`, which is the repository-scoped resolver `sync`/`land` use and which
        // therefore carries no project precondition. Found by sweeping every command at a workspace
        // root rather than by trusting that one chokepoint covered them all: nine refused, this one
        // still exited 0.
        Some("verification") => {
            let root = repo_arg(rest);
            keel_cli::workspace::require_project(&root, "keel verification [ROOT] [--pending]")
                .map_or_else(|code| code, |()| keel_cli::verification::cmd(rest, &root))
        }
        Some("audit-history") => keel_cli::history::cmd(rest, &find_repo_root().unwrap_or_else(|| PathBuf::from("."))),
        Some("audit-adherence") => keel_cli::adherence::cmd(rest, &find_repo_root().unwrap_or_else(|| PathBuf::from("."))),
        Some("github-gesture") => keel_cli::github::gesture_cmd(),
        Some("github-decision-id") => keel_cli::github::decision_id_cmd(rest),
        Some("github-decider") => keel_cli::github::decider_cmd(rest, &find_repo_root().unwrap_or_else(|| PathBuf::from("."))),
        Some("advance") => keel_cli::cursor::advance_cmd(rest, &find_repo_root().unwrap_or_else(|| PathBuf::from("."))),
        Some("enroll") => cmd_enroll(rest),
        Some("assured") => cmd_assured(rest),
        Some("decisions") => cmd_decisions(rest),
        Some("diagram") => cmd_diagram(rest),
        Some("render") => cmd_render(rest),
        Some("report") => cmd_report(rest),
        Some("indicators") => cmd_indicators(rest),
        Some("record-measurement") => cmd_record_measurement(rest),
        Some("snapshot-indicators") => cmd_snapshot_indicators(rest),
        Some("apply-review") => cmd_apply_review(rest),
        Some("outstanding") => cmd_query0(rest, "outstanding", keel_cli::queries::outstanding),
        Some("workflows") => cmd_query0(rest, "workflows", keel_cli::queries::workflows),
        Some("item") => cmd_query1(rest, "item", keel_cli::queries::item),
        Some("trace") => cmd_query1(rest, "trace", keel_cli::queries::trace),
        Some("trace-need") => cmd_query1(rest, "trace-need", keel_cli::queries::trace_need),
        Some("append-result") => cmd_append_result(rest),
        Some("append-gate-result") => cmd_append_gate_result(rest),
        Some("add-task") => cmd_add_task(rest),
        Some("accept") => cmd_accept(rest),
        Some("record") => cmd_record(rest),
        _ => print_usage(),
    };
    // STDERR, always, and after the command's own output: a computed view's stdout is JSON that
    // automation parses, so a perf line on stdout would corrupt every caller that asked for numbers.
    if let Some(r) = keel_cli::perf::report() {
        eprintln!("{r}");
    }
    process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::{bash_classify, bash_tokens, classify_guard_args, remap_engine_content, remap_engine_path, root_arg, BashVerdict, Path};

    /// D0176/P1.2: argv-level matching — operators as tokens, never substrings of prose. A commit
    /// message DESCRIBING --no-verify does not match; the real flag does; the keel carve-out
    /// exempts ordinary keel commands but NEVER the human-judgment set.
    #[test]
    #[allow(clippy::expect_used)] // test setup: a failed mkdir should abort the test loudly
    fn bash_matcher_is_argv_level_and_carves_out_human_judgment() {
        let root = std::env::temp_dir().join("keel-bash-classify");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
        std::fs::write(
            root.join(".tracking").join("actors.sysml"),
            "package A {\n    part hum : Person { :>> name = \"H\"; }\n}\n",
        )
        .expect("actors");
        assert!(matches!(bash_classify(&root, "git commit --no-verify -m x"), BashVerdict::Block(_)));
        assert!(
            matches!(bash_classify(&root, "git commit -m \"never use --no-verify\""), BashVerdict::Clean),
            "prose inside quotes is ONE token and must not match"
        );
        assert!(matches!(bash_classify(&root, "echo x > .tracking/issues.sysml"), BashVerdict::Block(_)));
        assert!(matches!(bash_classify(&root, "sed -i s/a/b/ .tracking/backlog.sysml"), BashVerdict::Block(_)));
        assert!(matches!(bash_classify(&root, "git config core.hooksPath /dev/null"), BashVerdict::Block(_)));
        assert!(matches!(bash_classify(&root, "SKIP_VALIDATE=1 git commit -m x"), BashVerdict::Block(_)));
        assert!(matches!(bash_classify(&root, "keel validate ."), BashVerdict::Clean), "ordinary keel is exempt");
        assert!(matches!(bash_classify(&root, "keel accept d1 --by hum"), BashVerdict::Ask(_)), "accept is never exempt");
        assert!(matches!(bash_classify(&root, "keel actor set hum"), BashVerdict::Ask(_)), "actor mutation is never exempt");
        assert!(
            matches!(bash_classify(&root, "KEEL_ACTOR=hum keel append-result --file f --task t --sha s"), BashVerdict::Ask(_)),
            "writing AS a Person routes to the human channel"
        );
        assert!(
            matches!(bash_classify(&root, "KEEL_ACTOR=someAi keel append-result --file f --task t --sha s"), BashVerdict::Clean),
            "writing as a non-Person is the normal agent case"
        );
        assert_eq!(bash_tokens("a \"b c\" d"), vec!["a", "b c", "d"], "quoted text glues to one token");
    }

    /// issue150: the oversight advisory must be ADVISORY. A turn boundary that blocked because a server
    /// was not running would be the over-strict gate that trains its actor to disable it, taking the
    /// honest-state checks in the same hook down with it (issue076/issue081, and D0132 in the large).
    #[test]
    fn the_oversight_advisory_never_blocks() {
        const MAIN_RS: &str = include_str!("main.rs");
        let at = MAIN_RS.find("fn hook_stop(").unwrap_or(0);
        assert!(at > 0, "hook_stop must exist");
        let hook = &MAIN_RS[at..];
        // The property, not the layout: the advisory ALONE must resolve to a systemMessage. It now also
        // appends to a blocking reason when the model is separately dishonest (issue150) - that block is
        // caused by `problems`, never by the console being down - so the earlier version of this test,
        // which scanned a region between two markers, asserted a structure rather than the property and
        // failed the moment the advisory was hoisted out of the green branch.
        let solo = hook.find("oversight.map_or(0,").unwrap_or(0);
        assert!(solo > 0, "the advisory alone must resolve via map_or, returning 0 when absent");
        // No char escape: the first line of the slice is the statement we care about.
        let line_end = solo + hook[solo..].lines().next().map_or(0, str::len);
        assert!(
            hook[solo..line_end].contains("systemMessage"),
            "the advisory's own emit must be a systemMessage -- the human's console being down is not              dishonest state and must never block a turn"
        );
        // And a block must still be gated on `problems`, so nothing can reach it via the advisory.
        let block = hook.find("\"decision\": \"block\"").unwrap_or(0);
        assert!(block > 0, "the block emit must exist");
        assert!(
            hook[..block].contains("if problems.is_empty() {"),
            "the block path must sit behind the problems check"
        );
    }

    /// The advisory's count must come from the console's own obligation computation, not a second one.
    #[test]
    fn the_advisory_counts_what_the_console_shows() {
        let total = keel_cli::serve::obligations_total(std::path::Path::new(".."));
        assert!(total.is_some(), "obligations_total must be computable for this repository");
        assert!(
            total.unwrap_or(0) > 0,
            "this repository has outstanding human obligations, so the reused count must be non-zero --              a zero here would mean the hook silently advises nothing while work waits"
        );
    }

    #[test]
    fn an_unknown_flag_is_a_mistake_and_never_a_root() {
        // issue133: the whole class. A mistyped flag used to BECOME the root path (or, in the
        // skip-flags variant, be silently ignored), so a typo produced a confident wrong answer
        // somewhere downstream instead of an error where the mistake was made.
        let a = |v: &[&str]| v.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
        assert_eq!(root_arg(&a(&["--explan"]), "u", &[], 0), Err(2));
        assert_eq!(root_arg(&a(&["--explan"]), "u", &["explain"], 0), Err(2), "a NEAR-MISS of a known flag is still unknown");
        // A REAL project path, because `root_arg` now VALIDATES as well as parses (issue281): it
        // refuses a root that is not a keel project, so a synthetic `/r` no longer reaches the caller.
        // The assertions below still test what they always did — that the positional is FOUND around a
        // declared flag — they just use a root that a caller could really pass.
        let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let r = repo.to_string_lossy().to_string();
        let found = |v: &[&str], pos: usize| {
            root_arg(&a(v), "u", &["explain"], pos).ok().map(|p| p.to_string_lossy().to_string())
        };
        assert_eq!(found(&["--explain", &r], 0), Some(r.clone()));
        assert_eq!(found(&[&r, "--explain"], 0), Some(r.clone()));
        // `positionals` skips the subcommand's own leading argument (`keel view <name> [ROOT]`)
        assert_eq!(found(&["decisions", &r], 1), Some(r.clone()));
        // and a leading positional alone leaves ROOT to repo discovery, not to the positional
        assert_ne!(root_arg(&a(&["decisions"]), "u", &[], 1).map(|p| p.to_string_lossy().to_string()), Ok("decisions".to_string()));
        // The new half of the contract: a path that exists but is NOT a project is refused, never
        // answered over. This is the false green issue281 closed.
        let tmp = std::env::temp_dir();
        assert_eq!(root_arg(&a(&[&tmp.to_string_lossy()]), "u", &[], 0), Err(2), "a non-project root is refused");
    }

    #[test]
    fn guard_args_distinguish_name_from_root() {
        // Regression (v0.1.0 release smoke): `keel guard <ROOT>` must run all guards on ROOT, not read
        // ROOT as a guard name. A known name runs that one guard; "all"/no-arg/a path runs all.
        let s = |v: &[&str]| v.iter().map(|x| (*x).to_string()).collect::<Vec<_>>();
        assert_eq!(classify_guard_args(&s(&[])), (None, None)); // run all, default root
        assert_eq!(classify_guard_args(&s(&["all"])), (None, None)); // run all
        assert_eq!(classify_guard_args(&s(&["myproj"])), (None, Some("myproj"))); // bare ROOT -> run all on it
        assert_eq!(classify_guard_args(&s(&["."])), (None, Some("."))); // "." is a ROOT, not a guard
        assert_eq!(classify_guard_args(&s(&["ceremony"])), (Some("ceremony"), None)); // a known guard name
        assert_eq!(classify_guard_args(&s(&["ceremony", "myproj"])), (Some("ceremony"), Some("myproj"))); // name + root
        assert_eq!(classify_guard_args(&s(&["all", "myproj"])), (None, Some("myproj"))); // all on root
    }

    #[test]
    fn engine_path_remap_isolates_decisions() {
        // D0093 boundary: decisions ship as read-only reference, never as the new project's instance.
        assert_eq!(remap_engine_path(Path::new("decisions/0001-x.sysml")), Path::new("reference/decisions/0001-x.sysml"));

        // Everything else is scaffolded unchanged.
        assert_eq!(remap_engine_path(Path::new("schema/core/element.sysml")), Path::new("schema/core/element.sysml"));
        assert_eq!(remap_engine_path(Path::new("processes/introduction.sysml")), Path::new("processes/introduction.sysml"));
    }

    /// issue291: the reference copy's package must be renamed, or the project's own first recorded
    /// decision collides with it. Exercises the real corpus shape - a `// D0001` prose comment above
    /// the declaration, and a `procedureText` mentioning `d0001` - because the rename must touch the
    /// declaration ONLY (189 of 514 `dNNNN` occurrences downstream sit inside prose strings).
    #[test]
    fn reference_decision_package_is_renamed_declaration_only() {
        let src = concat!(
            "// D0001 - text files are truth\n",
            "package Decision0001 {\n",
            "    part d0001 : Decision {\n",
            "        :>> procedureText = \"ww confirmed d0001 on the call\";\n",
            "    }\n",
            "}\n",
        );
        // `unwrap_or_default` rather than `expect`: the fail-loud lints deny panic/expect/unwrap
        // even here, and an empty string fails every assert below with the content in the message.
        let out = remap_engine_content(Path::new("decisions/0001-text-files-are-truth.sysml"), src)
            .unwrap_or_default();
        assert!(out.contains("package ReferenceDecision0001 {"), "package renamed: {out}");
        assert!(out.contains("part d0001 : Decision"), "part name untouched (global resolution): {out}");
        assert!(out.contains("confirmed d0001 on the call"), "prose untouched: {out}");
        assert!(out.contains("// D0001 - text files are truth"), "comment untouched: {out}");
        // Idempotent: the transform's own output declares no `package DecisionNNNN` to rename, which
        // is what keeps `step_engine_resync` from planning an edit every run.
        assert!(remap_engine_content(Path::new("decisions/0001-x.sysml"), &out).is_none());
    }

    /// Only files remapped INTO `reference/decisions/` are transformed - a schema or process file
    /// that happens to mention a decision is copied byte-for-byte.
    #[test]
    fn non_decision_engine_files_are_never_content_transformed() {
        let src = concat!(
            "package EngineRules {\n",
            "    #JustifiedBy dependency from r to d0099;\n",
            "}\n",
        );
        assert!(remap_engine_content(Path::new("rules/rules.sysml"), src).is_none());
        assert!(remap_engine_content(Path::new("schema/core/element.sysml"), src).is_none());
    }

    #[test]
    fn version_guard_split_cannot_drift_from_the_guards_that_run() {
        // `keel version` reports the hard-vs-warning split by SUBTRACTING the warning list from
        // GUARD_NAMES. If a name in the warning list is not an actual enforced guard the reported hard
        // count silently overstates enforcement — and a longer warning list would underflow the
        // subtraction outright. Both are the same defect class as the version gap this command fixes:
        // a number a reader would trust that nothing checks.
        for w in super::WARNING_ONLY_GUARDS {
            assert!(
                keel_cli::guards::GUARD_NAMES.contains(&w),
                "warning-only guard `{w}` is not in GUARD_NAMES — the reported hard count would be wrong"
            );
        }
        assert!(
            super::WARNING_ONLY_GUARDS.len() < keel_cli::guards::GUARD_NAMES.len(),
            "warning list must be a strict subset — otherwise the hard count underflows"
        );
    }

    #[test]
    fn init_ships_downstream_claude_md_not_self_build() {
        // issue057 (field defect): `keel init` must ship a DOWNSTREAM "tracked by keel" CLAUDE.md,
        // NEVER the self-build's ("This repo is a work-tracking engine"). D0047 permanent control.
        assert!(super::CLAUDE_MD.contains("tracked by keel"), "init CLAUDE.md must frame the project as tracked BY keel");
        assert!(!super::CLAUDE_MD.contains("is a work-tracking engine"), "init must NOT ship the self-build CLAUDE.md");
        assert!(super::CLAUDE_MD.contains("Parsed:"), "downstream CLAUDE.md must carry the D0106 parse-first discipline");
    }

    /// THE CONTROL for issue243: `keel init` must not ship THIS project's instance data as if it were
    /// engine definition. A fresh tree inherited a byte-identical activation.toml, so it reported
    /// `declared manifest: yes` for an adoption declaration it never made - falsifying the guarantee
    /// that an absent manifest means everything is active - and inherited the self-build's EXCHANGE
    /// IDENTITIES for units it never exported.
    #[test]
    fn init_resets_instance_contracts_and_keeps_engine_definition() {
        use std::path::Path;
        for p in ["contracts/activation.toml", "contracts/unit-ids.toml", "contracts/installed-units.toml",
                  "contracts/github-actors.toml"] {
            assert!(super::is_instance_contract(Path::new(p)), "{p} is instance data and must be reset");
        }
        // Engine DEFINITION must still be copied verbatim.
        for p in ["contracts/process-enforcement.toml", "processes/intake.sysml", "rules/rules.sysml"] {
            assert!(!super::is_instance_contract(Path::new(p)), "{p} is engine definition and must be shipped as-is");
        }
        // The activation starter must leave the honest default IN FORCE, i.e. no live [processes]
        // section - a commented template is guidance, an uncommented one is a declaration.
        let starter = super::starter_for(Path::new("contracts/activation.toml"));
        for line in starter.lines() {
            let l = line.trim();
            assert!(
                l.is_empty() || l.starts_with('#'),
                "the activation starter must declare NOTHING; found a live line: {l}"
            );
        }
        // The id registries start genuinely empty: no `name = "uuid"` entry may be inherited.
        let ids = super::starter_for(Path::new("contracts/unit-ids.toml"));
        assert!(!ids.lines().any(|l| !l.trim_start().starts_with('#') && l.contains('=')),
            "an inherited unit id claims a lineage this project does not have");
        // D0219: the decider table must start EMPTY - an inherited decider could record
        // acceptances in another project's tree under someone else's name.
        let gh = super::starter_for(Path::new("contracts/github-actors.toml"));
        assert!(gh.contains("[logins]"), "the starter must still show the section shape");
        assert!(!gh.lines().any(|l| !l.trim_start().starts_with('#') && l.contains('=')),
            "no login may be inherited: who may decide is per-project");
    }
}
