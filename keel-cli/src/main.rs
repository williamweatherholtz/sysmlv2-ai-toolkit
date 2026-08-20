//! `keel` — CLI entry point.
//!
//! Subcommands:
//!   `validate [ROOT]`         — semantic-validate all `.tracking/` files
//!   `check FILE...`           — parse-check one or more `.sysml` files
//!   `orient [ROOT]`           — print orient state (cursor + ready/done/outstanding) as JSON
//!   `whats-next [ROOT]`       — print ready task names, one per line
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
/// Degrades to a skip (never blocks) if `keel` isn't on PATH. POSIX sh.
const PRECOMMIT_HOOK: &str = "#!/bin/sh\n# keel pre-commit gate (Rust-only; no JVM kernel) — scaffolded by `keel init` (D0048/D0093).\n# Enable: git config core.hooksPath .githooks   |   bypass once: SKIP_KEEL=1 git commit ...\n[ \"$SKIP_KEEL\" = \"1\" ] && { echo 'pre-commit: SKIP_KEEL=1 — keel gate skipped'; exit 0; }\nKEEL=\"${KEEL:-keel}\"\ncommand -v \"$KEEL\" >/dev/null 2>&1 || { echo \"pre-commit: '$KEEL' not on PATH — keel gate skipped (install keel to enforce)\"; exit 0; }\necho 'pre-commit: keel validate .'\n\"$KEEL\" validate . || { echo 'pre-commit: keel validate FAILED — commit aborted'; exit 1; }\necho 'pre-commit: keel guard'\n\"$KEEL\" guard || { echo 'pre-commit: keel guard FAILED — commit aborted'; exit 1; }\n";

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
        if !dir.pop() {
            return None;
        }
    }
}

/// Resolve a subcommand's optional `[ROOT]` positional, REFUSING an unrecognised flag (issue133).
///
/// `positionals` is how many leading positional arguments the subcommand takes before ROOT (`keel view
/// <name> [ROOT]` passes 1). `known` lists the flag names the subcommand accepts, without `--`.
///
/// THE DEFECT THIS EXISTS TO END: every parser used to take its first argument as ROOT, so `keel audit
/// --explan` made the root the literal string `--explan`, and the command then failed somewhere
/// downstream about a missing directory — or worse, succeeded against the wrong tree. The other half of
/// the class SKIPPED anything starting with `--`, so an unknown flag was silently ignored and the command
/// ran with the wrong behaviour and said nothing. Both turn a typo into a confident wrong answer instead
/// of an error at the point of the mistake, which is the shape this whole class keeps taking.
///
/// `Err(2)` after printing the usage line; the caller returns that code unchanged.
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
        return Ok(PathBuf::from(p.as_str()));
    }
    find_repo_root().ok_or_else(|| {
        eprintln!("error: no .engine/ directory found from the current directory upward.");
        eprintln!("usage: {usage}");
        2
    })
}

// ── subcommands ───────────────────────────────────────────────────────────────

fn cmd_validate(args: &[String]) -> i32 {
    let root = match root_arg(args, "keel validate [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };

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
    // waiting must look exactly like a hook with nothing to say.
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(HOOK_DEADLINE_SECS));
        std::process::exit(0);
    });
    let mut raw = String::new();
    let _ = std::io::stdin().read_to_string(&mut raw);
    let payload: serde_json::Value = serde_json::from_str(&raw).unwrap_or(serde_json::Value::Null);
    let root = find_repo_root().unwrap_or_else(|| PathBuf::from("."));
    if !root.join(".tracking").is_dir() {
        return 0; // not a keel project -> silent no-op, correctly
    }

    match event {
        "post-edit" => hook_post_edit(&payload, &root),
        "stop" => hook_stop(&payload, &root),
        "user-prompt" => hook_user_prompt(&root),
        "pre-bash" => hook_pre_bash(&payload),
        other => {
            eprintln!("unknown hook event '{other}' (expected stop|post-edit|pre-bash|user-prompt)");
            2
        }
    }
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
fn hook_pre_bash(payload: &serde_json::Value) -> i32 {
    let cmd = payload.pointer("/tool_input/command").and_then(serde_json::Value::as_str).unwrap_or_default();
    let advisories = keel_cli::shellcheck::inspect(cmd);
    if advisories.is_empty() {
        return 0;
    }
    println!("[shell-adaptation -- CLAUDE.md sec 6, the #1 avoidable-friction class (issue094)]");
    for a in &advisories {
        println!("  {}", a.what);
        println!("    fix: {}", a.fix);
    }
    println!("  Advisory only -- nothing is blocked. If the command errors or hangs, SWITCH TOOLS rather than re-issuing the same form.");
    0
}

/// `UserPromptSubmit`: inject the route-first checklist, plus a warning about out-of-band writes.
///
/// ASCII-only on purpose: this text is injected into the model's context via stdout, and a non-UTF-8
/// Windows console turns non-ASCII into mojibake.
fn hook_user_prompt(root: &Path) -> i32 {
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
        std::process::Command::new("git")
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
        return 0; // clean -> silent, so a passing gate costs nothing
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
    let mut failing: Vec<String> = Vec::new();
    for name in keel_cli::guards::GUARD_NAMES {
        if let Some(r) = keel_cli::guards::run_one(name, root) {
            for v in r.violations.iter().take(5) {
                failing.push(format!("  [{name}] {v}"));
            }
        }
    }
    if !failing.is_empty() {
        problems.push(format!("keel guard:\n{}", failing.join("\n")));
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
                "[oversight] {total} item(s) are waiting on the HUMAN, and this session is BRIDGED (CLAUDE_CODE_BRIDGE_SESSION_ID is set), so a localhost console may not be reachable for them. Publish the review deck instead of starting `keel serve`: run the `obligation-review` skill (`python .engine/tools/obligation_canvas.py . -o <path>` then publish it), and hand them the URL."
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
        // Second consecutive red: allow the stop with a loud warning rather than trapping the agent.
        return hook_emit(&serde_json::json!({
            "systemMessage": "[in-loop gate] Still red after a correction pass — allowing the stop to avoid a loop. Do NOT commit until keel validate + guard are green."
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
    let fast = args.iter().any(|a| a == "--fast");
    let root = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(PathBuf::from)
        .or_else(find_repo_root)
        .unwrap_or_else(|| PathBuf::from("."));
    if !fast {
        eprintln!("usage: keel gate --fast [ROOT]   (the per-edit in-loop gate: validate + duplicate-identity + marker-vocabulary)");
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
    // The two EXACT guards — set membership and duplicate detection, no heuristics.
    for name in ["duplicate-identity", "marker-vocabulary"] {
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
                eprintln!("usage: keel serve [--port N] [ROOT]");
                return 2;
            }
        }
    };
    keel_cli::serve::run(root, port)
}

fn cmd_orient(args: &[String]) -> i32 {
    let html = args.iter().any(|a| a == "--html");
    let root = match root_arg(args, "keel orient [ROOT] [--html]", &["html"], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
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
    let Some(name) = name else {
        let reports = keel_cli::guards::run_all(&root);
        let mut all_ok = true;
        for r in &reports {
            r.print();
            all_ok &= r.ok();
        }
        println!("[guard] {}", if all_ok { "ALL PASS" } else { "FAILED" });
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
    let root = match root_arg(args, "keel rules [ROOT]", &[], 0) {
        Ok(r) => r,
        Err(code) => return code,
    };
    match keel_cli::view::check(&root) {
        Ok(json) => {
            println!("{json}");
            0
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
    for task in orient::compute(&root).ready {
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
    let judged_at = flag(args, "judged-at").unwrap_or_else(|| "2026-01-01".to_owned());

    match w::append_result(&file, &task, &sha, &verdict, &judged_at, &judged_by) {
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
    let judged_at = flag(args, "judged-at").unwrap_or_else(|| "2026-01-01".to_owned());

    let notes = flag(args, "notes");
    match w::append_gate_result(&file, &gate, &sha, &verdict, &judged_at, &judged_by, notes.as_deref()) {
        Ok(uuid) => { println!("{uuid}"); 0 }
        Err(e) => { eprintln!("error: {e}"); 1 }
    }
}

/// `keel record <type> ...` — the closed RMWX `record` verb (D0105/D0106; issue054 C1). Currently
/// records a Decision: `keel record decision --slug S --title T --context C --decision D --rationale R
/// --consequences Q --date YYYY-MM-DD --author A [--root ROOT]` → writes a proposed Decision file
/// (auto NNNN + UUID), killing point-of-decision friction (D0054). Acceptance stays a separate human gate.
fn cmd_record(args: &[String]) -> i32 {
    if args.first().map(String::as_str) == Some("issue") {
        return cmd_record_issue(args);
    }
    if args.first().map(String::as_str) != Some("decision") {
        eprintln!("usage: keel record decision --slug S --title T --context C --decision D --rationale R --consequences Q --date YYYY-MM-DD --author A [--root ROOT]");
        eprintln!("       keel record issue --title T --description D --severity Critical|High|Medium|Low --resolver R --date YYYY-MM-DD [--related-task T] [--marker M] [--in-field] [--by A] [--root ROOT]");
        return 2;
    }
    let root = flag(args, "root").map_or_else(
        || find_repo_root().unwrap_or_else(|| PathBuf::from(".")),
        PathBuf::from,
    );
    let req = |name: &str| flag(args, name);
    let (Some(slug), Some(title), Some(context), Some(decision), Some(rationale), Some(consequences)) =
        (req("slug"), req("title"), req("context"), req("decision"), req("rationale"), req("consequences"))
    else {
        eprintln!("error: --slug --title --context --decision --rationale --consequences are all required (a substantive why — D0103)");
        return 2;
    };
    let date = flag(args, "date").unwrap_or_default();
    // NEVER default to a named human (D0129/issue072): that silently forges a human attestation.
    let author = match keel_cli::actor::resolve(&root, flag(args, "author").as_deref()) {
        Ok(a) => a,
        Err(msg) => { eprintln!("{msg}"); return 2; }
    };
    if date.is_empty() {
        eprintln!("error: --date YYYY-MM-DD required (the attestation time is its own irreducible fact)");
        return 2;
    }
    match w::record_decision(&root, &slug, &title, &date, &author, &context, &decision, &rationale, &consequences) {
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
    let at = flag(args, "at").unwrap_or_else(|| "2026-01-01".to_owned());
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
    let at = flag(args, "at").unwrap_or_else(|| "2026-01-01".to_owned());
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
    let judged_at = flag(args, "judged-at").unwrap_or_else(|| "2026-01-01".to_owned());
    let critiques = root.join(".tracking").join("critiques.sysml");

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
use keel_cli::migrate::{is_engine_dev_only, remap_engine_path};

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

fn cmd_init(args: &[String]) -> i32 {
    let target = match positional_arg(args, "keel init DIR", "a directory") {
        Ok(a) => a,
        Err(code) => return code,
    };
    let dir = PathBuf::from(target);
    let engine_dst = dir.join(".engine");
    if engine_dst.exists() {
        eprintln!("error: {} already contains a .engine/ — refusing to overwrite", dir.display());
        return 2;
    }
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
    // Scaffold a RUST-ONLY commit gate (.githooks/pre-commit) so the project has an automated
    // keel validate/guard gate from day one — no conda/kernel (D0048). The user enables it with
    // `git config core.hooksPath .githooks` (printed below).
    let hooks = dir.join(".githooks");
    if let Err(e) = std::fs::create_dir_all(&hooks) {
        eprintln!("error creating .githooks: {e}");
        return 1;
    }
    let hook_path = hooks.join("pre-commit");
    if let Err(e) = std::fs::write(&hook_path, PRECOMMIT_HOOK) {
        eprintln!("error writing .githooks/pre-commit: {e}");
        return 1;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let _ = std::fs::set_permissions(&hook_path, std::fs::Permissions::from_mode(0o755));
    }
    println!("Scaffolded the engine into {} ({count} engine file(s)).", dir.display());
    println!();
    println!("Next:");
    println!("  1. cd {}", dir.display());
    println!("  2. git init && git config core.hooksPath .githooks   (enable the keel pre-commit gate)");
    println!("  3. Read CLAUDE.md — how to work here (text is truth; the AI drives the CLI, you supervise).");
    println!("  4. Run the `introduction` skill (guided onboarding) — capture your first need + run your first sprint.");
    println!("     Or: keel orient .   (where things stand)");
    println!();
    println!("The .githooks/pre-commit gate runs `keel validate` + `keel guard` (Rust-only, no kernel).");
    println!("Engine design rationale is read-only reference in .engine/reference/decisions/;");
    println!("your project authors its OWN decisions fresh in .engine/decisions/.");
    0
}

/// Header kept at the top of a generated `activation.toml` — the file must explain itself, because the
/// consequence of editing it wrongly (a control silently off) is not obvious from its contents.
const ACTIVATION_HEADER: &str = "\
# Process activation (D0138) — which processes THIS project has adopted.
#
# What activating a process does: turns on its whole unit (skill + declared rules + guards), as defined
# by the engine in `.engine/contracts/process-units.toml`. Deactivating one stops its guards running,
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
                "error: `{target}` is a declared process but asserts no guard, so there is nothing for {mode} to switch. Activation governs GUARDS (D0138). To stop running this process, remove the facts it authors -- a process whose inputs are absent produces nothing -- or give it an `assert constraint` so it becomes switchable."
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
    let sha = std::process::Command::new("git")
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
    "  activation [ROOT]            which processes this project has ADOPTED, and which guards are core (D0138)",
    "  activate|deactivate PROCESS  adopt/drop a process as a UNIT — skill + rules + guards in one step",
    "  serve [--port N] [ROOT]      the interactive console — localhost read dashboard (D0094 m1)",
    "  validate [ROOT]              semantic-validate all .tracking/ files",
    "  check FILE...                parse-check one or more .sysml files",
    "  check --spec-version         report the baked grammar version vs upstream (--no-fetch to skip the live check)",
    "  ls [ROOT]                    list .tracking/ .sysml files",
    "  orient [ROOT] [--html]       orient state as JSON, or --html = the human dashboard #View (D0093)",
    "  whats-next [ROOT]            print ready task names (one per line)",
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
    "hardening [ROOT]             the critique process's own questions, computed (issue171/D0169)",
    "check-engine [ROOT]          .engine instance reference resolution, kernel-free (D0112 phase 2)",
    "hook post-edit|stop|pre-bash the in-loop gates, in the binary — no python runtime (D0134)",
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
        Some("orient") => cmd_orient(rest),
        Some("whats-next") => cmd_whats_next(rest),
        Some("view") => cmd_view(rest),
        Some("attestation-coverage") => cmd_attestation_coverage(rest),
        Some("orphans") => cmd_orphans(rest),
        Some("audit") => cmd_audit(rest),
        Some("hardening") => cmd_hardening(rest),
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
        Some("verification") => keel_cli::verification::cmd(rest, &repo_arg(rest)),
        Some("audit-history") => keel_cli::history::cmd(rest, &find_repo_root().unwrap_or_else(|| PathBuf::from("."))),
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
    use super::{classify_guard_args, remap_engine_path, root_arg, Path};

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
        // a declared flag passes through, and the positional is still found around it
        assert_eq!(root_arg(&a(&["--explain", "/r"]), "u", &["explain"], 0).ok().map(|p| p.to_string_lossy().to_string()), Some("/r".to_string()));
        assert_eq!(root_arg(&a(&["/r", "--explain"]), "u", &["explain"], 0).ok().map(|p| p.to_string_lossy().to_string()), Some("/r".to_string()));
        // `positionals` skips the subcommand's own leading argument (`keel view <name> [ROOT]`)
        assert_eq!(root_arg(&a(&["decisions", "/r"]), "u", &[], 1).ok().map(|p| p.to_string_lossy().to_string()), Some("/r".to_string()));
        // and a leading positional alone leaves ROOT to repo discovery, not to the positional
        assert_ne!(root_arg(&a(&["decisions"]), "u", &[], 1).map(|p| p.to_string_lossy().to_string()), Ok("decisions".to_string()));
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
}
