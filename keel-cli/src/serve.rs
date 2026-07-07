//! `serve` (D0094 m1) — the Keel interactive console: a LOCALHOST tokio+axum server over the engine.
//!
//! The human's ACTING surface (D0094). m1 = the live READ console; m2 = DETERMINISTIC ACTIONS: record
//! a disposition (POST -> the write API) and open a full HTML report/diagram. Every read computes from
//! the existing view authority; every write goes through the write API + guards (ONE truth, no second
//! store). A request-logging middleware makes the server observable. Localhost-only; the agent-bridge
//! (m3) builds on this. Tiers degrade gracefully.

use std::collections::HashMap;
use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::{Path as AxPath, Query, Request, State};
use axum::http::StatusCode;
use axum::middleware::{self, Next};
use axum::response::sse::{Event, Sse};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use tokio::io::AsyncBufReadExt;
use tokio_stream::Stream;

use crate::json::Json;

/// The embedded single-page console frontend (self-contained, no CDN — the cytoscape precedent).
const CONSOLE_HTML: &str = include_str!("../assets/console.html");

/// Per-action turn cap (the agent-bridge cost guardrail, D0094) + max concurrent agent runs.
const AGENT_MAX_TURNS: &str = "30";
const AGENT_MAX_CONCURRENT: usize = 2;

#[derive(Clone)]
struct AppState {
    root: Arc<PathBuf>,
    /// In-flight agent-bridge runs (concurrency guardrail, D0094).
    agents: Arc<AtomicUsize>,
    /// Per-view JSON cache keyed `view -> (fingerprint, json)` (D0094 serveLiveCache): recompute a view
    /// only when the model's content fingerprint changes; a materialized #View cache (regenerable, §2.1).
    cache: Arc<Mutex<HashMap<String, (u64, String)>>>,
}

/// Run the console server on `127.0.0.1:port` over `root`. Blocks until interrupted.
///
/// # Errors
/// Returns a non-zero exit code if the runtime fails to build or the port cannot be bound.
#[must_use]
pub fn run(root: PathBuf, port: u16) -> i32 {
    let rt = match tokio::runtime::Builder::new_multi_thread().enable_all().build() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("serve: cannot start runtime: {e}");
            return 1;
        }
    };
    rt.block_on(async move { serve_async(root, port).await })
}

async fn serve_async(root: PathBuf, port: u16) -> i32 {
    let state = AppState { root: Arc::new(root), agents: Arc::new(AtomicUsize::new(0)), cache: Arc::new(Mutex::new(HashMap::new())) };
    let app = Router::new()
        .route("/", get(index))
        .route("/api/orient", get(api_orient))
        .route("/api/recent", get(api_recent))
        .route("/api/decisions", get(api_decisions))
        .route("/api/business", get(api_business))
        .route("/api/launchables", get(api_launchables))
        .route("/api/dispositions", get(api_dispositions))
        .route("/api/processes", get(api_processes))
        .route("/api/report/:name", get(api_report))
        .route("/api/history", get(api_history))
        // m2 — deterministic actions
        .route("/api/disposition", post(api_disposition))
        .route("/view/report/:name", get(view_report))
        .route("/view/diagram", get(view_diagram))
        // persistent serve settings (e.g. the agent-bridge toggle — claude -p billing control)
        .route("/api/settings", get(api_settings_get).post(api_settings_post))
        // m3 — agent-bridge (headless claude -> SSE)
        .route("/api/agent/stream", get(api_agent_stream))
        // serveLiveCache — event-driven change push (SSE)
        .route("/api/events", get(api_events))
        // serveItemIntrospect — generic any-item detail
        .route("/api/item/:name", get(api_item))
        // sr18 — bounded section render (a declared view, or an element + its 1-hop neighbourhood)
        .route("/api/section", get(api_section))
        // sr19 — Need-slice boundary (white-box internals + black-box interfaces) + the tier sweep
        .route("/api/boundary", get(api_boundary))
        .route("/api/boundary-sweep", get(api_boundary_sweep))
        // serveItemActions — append a downstream TestResult to a task
        .route("/api/testresult", post(api_testresult))
        // sr16 — on ACT, attach a tracked #Resolves resolver task to a finding
        .route("/api/resolver", post(api_resolver))
        .layer(middleware::from_fn(log_request))
        .with_state(state);
    let addr = format!("127.0.0.1:{port}");
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("serve: cannot bind {addr}: {e}");
            return 1;
        }
    };
    println!("Keel console (D0094 m1+m2+m3) on http://{addr}  \u{2014} Ctrl-C to stop");
    println!("  read:  / · /api/{{orient,decisions,dispositions,processes,report/<name>,history}}");
    println!("  act:   POST /api/disposition · /view/report/<name> · /view/diagram");
    println!("  agent: /api/agent/stream?action=critique&target=<x> (SSE; local `claude` CLI; directed-only — sr17)");
    println!("  live:  /api/events (SSE change-push; views cached per content fingerprint)");
    println!("  (requests logged to the terminal + keel-serve.log)");
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("serve: {e}");
        return 1;
    }
    0
}

async fn index() -> Html<&'static str> {
    Html(CONSOLE_HTML)
}

/// Request-logging middleware (D0094 m2 observability): logs method, path, status, and elapsed ms to
/// the terminal so the server is debuggable.
async fn log_request(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start = std::time::Instant::now();
    let resp = next.run(req).await;
    let line = format!("[keel serve] {method} {path} -> {} ({}ms)", resp.status().as_u16(), start.elapsed().as_millis());
    eprintln!("{line}");
    // Also append to keel-serve.log (best-effort; gitignored via *.log) so slow loads are inspectable.
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("keel-serve.log") {
        use std::io::Write as _;
        let _ = writeln!(f, "{line}");
    }
    resp
}

/// Current short HEAD of `root` (for a disposition's `judgedAgainst`); `"uncommitted"` if git fails.
fn git_head(root: &Path) -> String {
    std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map_or_else(|| "uncommitted".to_string(), |o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

/// A disposition request from the console (the human's explicit verdict on a >= Medium finding).
#[derive(serde::Deserialize)]
struct DispReq {
    finding: String,
    verdict: String,
    rationale: String,
    judged_at: String,
    judged_by: Option<String>,
}

/// POST /api/disposition (D0094 m2) — record a finding disposition via the write API (D0092). The
/// human clicking + entering rationale IS their explicit attestation (D0016); the agent does not infer
/// it. Writes a #Dispositions confirmation; never auto-commits.
async fn api_disposition(State(s): State<AppState>, axum::Json(body): axum::Json<DispReq>) -> Response {
    let verdict = match body.verdict.as_str() {
        "act" => "act",
        "accept-risk" | "acceptRisk" => "acceptRisk",
        "dismiss" => "dismiss",
        other => return (StatusCode::BAD_REQUEST, format!("{{\"error\":\"unknown verdict '{other}'\"}}")).into_response(),
    };
    let sha = git_head(&s.root);
    let judged_by = body.judged_by.filter(|b| !b.is_empty()).unwrap_or_else(|| "wweatherholtz".to_string());
    let critiques = s.root.join(".tracking").join("critiques.sysml");
    let d = crate::write::Disposition { finding: &body.finding, verdict, rationale: &body.rationale, sha: &sha, judged_at: &body.judged_at, judged_by: &judged_by };
    match crate::write::append_disposition(&critiques, &d) {
        Ok(name) => ok_json(format!("{{\"ok\":true,\"name\":\"{name}\",\"verdict\":\"{verdict}\"}}")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// Wrap a `ViewError`-fallible HTML computation into a response (500 + message on error).
fn view_html(r: Result<String, crate::view::ViewError>) -> Response {
    match r {
        Ok(html) => Html(html).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("render error: {e}")).into_response(),
    }
}

/// GET /view/report/:name (D0094 m2) — the full computed HTML report (instantiate/render action).
async fn view_report(State(s): State<AppState>, AxPath(name): AxPath<String>) -> Response {
    view_html(crate::view::report_html(&s.root, &name, false))
}

/// GET /view/diagram (D0094 m2) — the whole-model interactive diagram HTML (render action).
async fn view_diagram(State(s): State<AppState>) -> Response {
    view_html(crate::view::diagram_html(&s.root))
}

// ── m3 agent-bridge — drive the LOCALLY-AUTHENTICATED `claude` CLI, stream its work over SSE ───────
// The UI launches a headless Claude Code agent in the repo (so it loads CLAUDE.md + the skills) and
// streams events to the browser. Auth = the user's `claude` CLI subscription/ENTERPRISE session — the
// server NEVER sets ANTHROPIC_API_KEY (that would force pricier API-rate billing, D0094). The agent
// runs under the engine's EXISTING discipline; the prompt forbids auto-commit (the human commits).

/// Concurrency guard: decrements the in-flight agent counter when the stream ends or the client drops.
struct AgentSlot(Arc<AtomicUsize>);
impl Drop for AgentSlot {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Best-effort process-TREE killer for a spawned agent (serveAgentCancel, D0094). On Windows the
/// agent is `cmd /C claude` → `claude.exe`, and `kill_on_drop` reaps only the direct `cmd` child,
/// orphaning the `claude.exe` grandchild; this guard `taskkill /T`-kills the whole tree when the
/// SSE stream is dropped (client disconnect / Stop button / normal end). It is DISARMED once the
/// child exits normally, so it never targets a reaped (and possibly recycled) PID. On Unix the agent
/// is spawned as `claude` directly, so `kill_on_drop` already reaps it and this guard is a no-op.
struct TreeKiller(Option<u32>);
impl TreeKiller {
    const fn disarm(&mut self) {
        self.0 = None;
    }
}
impl Drop for TreeKiller {
    fn drop(&mut self) {
        let Some(pid) = self.0 else { return };
        if cfg!(windows) {
            let _ = std::process::Command::new("taskkill")
                .args(["/T", "/F", "/PID", &pid.to_string()])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        }
    }
}

#[derive(serde::Deserialize)]
struct AgentReq {
    action: String,
    target: String,
    /// sr18 — optional section seed (`view:NAME` or `element:NAME`) scoping the critique to a bounded
    /// neighbourhood: the prompt names the section's members so the AI critiques `target` IN CONTEXT.
    #[serde(default)]
    section: Option<String>,
    /// sr19 — optional Need name for a BLACK-BOX interface critique: critique the Need-slice's cut edges
    /// (interfaces) rather than an element's internals. When set, `target` is ignored.
    #[serde(default)]
    boundary: Option<String>,
}

/// sr19 black-box critique prompt: critique the INTERFACES (cut edges) of a Need-slice boundary for
/// necessity, minimality, and completeness — recording each finding as an Issue REFERENCING the cut edge
/// (endpoints + kind; D0100 — no port, the edge is the interface). Names the interfaces so the critique is
/// concrete. The agent inherits CLAUDE.md discipline; the prompt forbids committing (the human's gate).
fn build_blackbox_prompt(need: &str, interfaces: &[String]) -> String {
    let list = if interfaces.is_empty() { "(none — the boundary is fully self-contained)".to_string() } else { interfaces.join("; ") };
    format!("Black-box (integration) critique of the Need-slice boundary `{need}`. Use the element-critique skill as an INDEPENDENT critic, but critique the INTERFACES — the cut edges crossing this boundary: {list}. For each interface assess necessity (is this cross-boundary edge needed?), minimality (is the boundary leaky — too many interfaces?), and completeness (is an expected interface missing?). Record each finding as a severity-carrying Issue that NAMES the interface (its endpoints + edge kind) per the issue-resolution process. Do NOT git commit; the human commits.")
}

/// Parse a section seed string (`view:NAME` / `element:NAME`) into the `(view, element)` pair
/// [`crate::view::section_json`] expects. A bare string with no prefix is treated as an element seed.
fn parse_section_seed(seed: &str) -> (Option<String>, Option<String>) {
    seed.strip_prefix("view:").map_or_else(
        || (None, Some(seed.strip_prefix("element:").unwrap_or(seed).to_string())),
        |v| (Some(v.to_string()), None),
    )
}

/// The only AI bridge action (sr17/D0098 directed-only): an antagonistic, RECORDING critique of a
/// named element. There is deliberately no free-form / investigate / chat action — every AI action is
/// directed at a named target and produces a recorded artifact (Issues). The agent inherits CLAUDE.md
/// discipline from the cwd; the prompt forbids committing (commits/acceptance stay the human's gate, D0016).
///
/// sr18 — when `section_members` is supplied, the critique is SECTION-SCOPED: the prompt names the
/// bounded local neighbourhood so the AI judges `target` in its context (whole-model views are too
/// coarse for local "does X make sense here" critique), still recording findings against the elements.
fn build_agent_prompt(target: &str, section_members: Option<&[String]>) -> String {
    use std::fmt::Write as _;
    let mut prompt = format!("Use the element-critique skill to adversarially critique `{target}` through its Core-3 lenses as an INDEPENDENT critic; record each finding as a severity-carrying Issue per the issue-resolution process.");
    if let Some(members) = section_members {
        if !members.is_empty() {
            let _ = write!(prompt, " Scope the critique to this bounded SECTION (its local neighbourhood): {}. Judge whether `{target}` is coherent, necessary, and well-formed WITHIN that local context; record findings against the section's elements.", members.join(", "));
        }
    }
    prompt.push_str(" Do NOT git commit; the human commits.");
    prompt
}

/// Persistent serve settings file (`<root>/.keel-serve.json`, gitignored). Project-local runtime
/// preferences for `keel serve`; absent file => defaults.
fn settings_path(root: &Path) -> PathBuf {
    root.join(".keel-serve.json")
}

/// Whether the AI agent bridge (`claude -p`) is enabled (serveSettings).
///
/// DEFAULT ON ("fine for now") — a persistent toggle so a user wary of `claude -p` billing (the D0094
/// caveat) can turn it OFF; when off, the console is pure read/oversight (no `claude -p` ever spawned).
fn agent_bridge_enabled(root: &Path) -> bool {
    std::fs::read_to_string(settings_path(root))
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .and_then(|v| v.get("agentBridge").and_then(serde_json::Value::as_bool))
        .unwrap_or(true)
}

/// GET /api/settings — the persisted serve settings (defaults applied).
async fn api_settings_get(State(s): State<AppState>) -> Response {
    ok_json(format!("{{\"agentBridge\":{}}}", agent_bridge_enabled(&s.root)))
}

#[derive(serde::Deserialize)]
struct SettingsReq {
    #[serde(rename = "agentBridge")]
    agent_bridge: bool,
}

/// POST /api/settings — persist serve settings (currently the agent-bridge toggle) to `.keel-serve.json`.
async fn api_settings_post(State(s): State<AppState>, axum::Json(b): axum::Json<SettingsReq>) -> Response {
    let body = format!("{{\"agentBridge\": {}}}\n", b.agent_bridge);
    match std::fs::write(settings_path(&s.root), body) {
        Ok(()) => ok_json(format!("{{\"ok\":true,\"agentBridge\":{}}}", b.agent_bridge)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// Probe `PATH` for a `claude` executable WITHOUT spawning it (serveDownstreamDegrade). The agent
/// bridge is optional: a downstream consumer who never installs Claude Code must get a CLEAR message,
/// not a cryptic exit code. On Windows the CLI is an npm shim resolved via `cmd /C` — which always
/// succeeds even when `claude` is absent (cmd exists), so spawn-failure detection misses it. Probing
/// PATH first makes the "not installed" path uniform across platforms. Honors `PATHEXT` on Windows.
fn claude_on_path() -> bool {
    let Some(path) = std::env::var_os("PATH") else { return false };
    let pathext = std::env::var_os("PATHEXT");
    claude_in_dirs(&path, pathext.as_deref())
}

/// Pure core of [`claude_on_path`] (testable without mutating process env). Scans `path` (an
/// `OsStr` in `PATH` syntax) for a `claude` executable. On Windows a bare name resolves via
/// `pathext` (`.CMD`/`.EXE`/`.BAT`/...); on Unix the literal name. `pathext` is honored only on
/// Windows (`cfg!(windows)`); falls back to the default extension set when absent.
fn claude_in_dirs(path: &std::ffi::OsStr, pathext: Option<&std::ffi::OsStr>) -> bool {
    let candidates: Vec<String> = if cfg!(windows) {
        let exts = pathext
            .and_then(|p| p.to_str())
            .filter(|s| !s.is_empty())
            .map_or_else(|| ".COM;.EXE;.BAT;.CMD".to_string(), str::to_string);
        std::iter::once("claude".to_string())
            .chain(exts.split(';').filter(|e| !e.is_empty()).map(|e| format!("claude{}", e.to_ascii_lowercase())))
            .collect()
    } else {
        vec!["claude".to_string()]
    };
    std::env::split_paths(path).any(|dir| candidates.iter().any(|c| dir.join(c).is_file()))
}

/// GET /api/agent/stream?action=critique&target= (D0094 m3; sr17 directed-only) — spawn a headless
/// `claude` agent in the repo and stream its `stream-json` events to the browser over SSE. The ONLY
/// accepted action is `critique` (the directed, RECORDING AI action — D0098/sr17: no free-form/chat/
/// investigate); any other action is rejected. Degrades gracefully if `claude` is absent; rejects past
/// the concurrency cap; never sets `ANTHROPIC_API_KEY`.
async fn api_agent_stream(State(s): State<AppState>, Query(q): Query<AgentReq>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let root = (*s.root).clone();
    let action_ok = q.action == "critique";
    // sr19 black-box: if a boundary (Need) is given, critique its interfaces (cut edges); else (sr18)
    // a section-scoped or plain white-box element critique.
    let prompt = if let Some(need) = q.boundary.as_deref() {
        let interfaces = crate::view::boundary_interfaces(&root, need).unwrap_or_default();
        build_blackbox_prompt(need, &interfaces)
    } else {
        let section_members = q.section.as_deref().and_then(|seed| {
            let (view, element) = parse_section_seed(seed);
            crate::view::section_member_names(&root, view.as_deref(), element.as_deref()).ok()
        });
        build_agent_prompt(&q.target, section_members.as_deref())
    };
    let counter = Arc::clone(&s.agents);
    let prev = counter.fetch_add(1, Ordering::SeqCst);
    let over_cap = prev >= AGENT_MAX_CONCURRENT;
    // Hold the slot for the stream's lifetime; on over-cap we release immediately below.
    let slot = AgentSlot(Arc::clone(&counter));

    let stream = async_stream::stream! {
        let _slot = slot; // dropped (counter--) when the stream finishes or the client disconnects
        if !agent_bridge_enabled(&root) {
            // serveSettings: the user disabled the `claude -p` bridge (billing control, D0094) — the
            // console is read/oversight-only; AI critique runs in the user's own CLI/Claude Code session.
            yield Ok(Event::default().event("error").data("the AI agent bridge is OFF in settings (it drives `claude -p`, whose billing is in flux \u{2014} D0094). Enable it in Settings, or run the critique in your own Claude Code session. The read console, views, and reports are unaffected."));
            return;
        }
        if !action_ok {
            // sr17 directed-only: the bridge serves exactly one recording action — critique.
            yield Ok(Event::default().event("error").data("only the `critique` action is supported \u{2014} the console has no free-form AI surface (sr17/D0098): every AI action is a directed, recorded critique of a named element. For free-form AI, open a terminal."));
            return;
        }
        if over_cap {
            yield Ok(Event::default().event("error").data(format!("busy: {AGENT_MAX_CONCURRENT} agent runs already in flight \u{2014} try again shortly")));
            return;
        }
        // serveDownstreamDegrade: clear, uniform message when the optional agent bridge isn't installed
        // (on Windows a missing `claude` would otherwise spawn `cmd` fine and exit 1 \u{2014} cryptic).
        if !claude_on_path() {
            yield Ok(Event::default().event("error").data("the `claude` CLI is not on PATH \u{2014} the agent bridge is optional. Install Claude Code and log in to your Claude subscription/enterprise to enable in-console actions (do NOT set ANTHROPIC_API_KEY \u{2014} that forces API-rate billing). The read console, views, and reports work without it."));
            return;
        }
        yield Ok(Event::default().event("status").data(format!("launching `claude` (turn cap {AGENT_MAX_TURNS}): {prompt}")));
        // Windows: `claude` is a `.cmd` npm shim that CreateProcess cannot spawn directly, so route via
        // `cmd /C` (which resolves claude.cmd on PATH). Unix: spawn `claude` directly. Either way the
        // CLI uses the user's own subscription/enterprise auth (we never set ANTHROPIC_API_KEY).
        let mut command = if cfg!(windows) {
            let mut c = tokio::process::Command::new("cmd");
            c.arg("/C").arg("claude");
            c
        } else {
            tokio::process::Command::new("claude")
        };
        let spawned = command
            .args(["-p", &prompt, "--output-format", "stream-json", "--include-partial-messages", "--verbose", "--max-turns", AGENT_MAX_TURNS])
            .current_dir(&root)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true) // client disconnect / stream end -> kill the agent (cancellation)
            .spawn();
        let mut child = match spawned {
            Ok(c) => c,
            Err(e) => {
                yield Ok(Event::default().event("error").data(format!("cannot launch the `claude` CLI ({e}). Install Claude Code + ensure it is on PATH and logged in to your Claude subscription/enterprise (do NOT set ANTHROPIC_API_KEY — that forces API-rate billing).")));
                return;
            }
        };
        // Arm a process-TREE killer (serveAgentCancel): if the client disconnects / hits Stop, the
        // SSE stream is dropped, dropping `killer` BEFORE `child` (reverse decl order) so the whole
        // `cmd`+`claude.exe` tree is reaped, not just the direct child. Disarmed on normal exit below.
        let mut killer = TreeKiller(child.id());
        if let Some(out) = child.stdout.take() {
            let mut lines = tokio::io::BufReader::new(out).lines();
            loop {
                match lines.next_line().await {
                    Ok(Some(line)) => yield Ok(Event::default().event("agent").data(line)),
                    Ok(None) => break,
                    Err(e) => {
                        yield Ok(Event::default().event("error").data(format!("read error: {e}")));
                        break;
                    }
                }
            }
        }
        let code = child.wait().await.ok().and_then(|st| st.code());
        killer.disarm(); // normal exit — the child is already reaped; don't taskkill a dead/recycled PID
        yield Ok(Event::default().event("done").data(format!("agent finished (exit {code:?})")));
    };
    Sse::new(stream)
}

/// A request to attach a resolver to a finding (sr16): on ACT, create a tracked `#Resolves` task.
#[derive(serde::Deserialize)]
struct ResolverReq {
    finding: String,
    title: String,
}

/// POST /api/resolver (sr16) — the tracked-resolver half of the critique loop. Creates a resolver
/// action in the backlog (`NextWork`) + a `#Resolves` edge from it to the finding, via the write API.
/// Idempotent on re-click (existing task / edge are no-ops). The actual fix is then done by the
/// process-aware agent / human; re-verify = re-run Critique on the element. Never auto-commits.
async fn api_resolver(State(s): State<AppState>, axum::Json(b): axum::Json<ResolverReq>) -> Response {
    // Resolver name = <finding-as-identifier>Fix (findings are SysML identifiers, e.g. issue046).
    let base: String = b.finding.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
    if base.is_empty() {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"empty finding\"}".to_string()).into_response();
    }
    let resolver = format!("{base}Fix");
    let title = b.title.replace('\\', "/").replace('"', "'").replace(['\n', '\r', '\t'], " ");
    let backlog = s.root.join(".tracking").join("backlog.sysml");
    let issues = s.root.join(".tracking").join("issues.sysml");
    match crate::write::add_task(&backlog, "NextWork", &resolver, &title, "inspect") {
        // Ok = created; TaskAlreadyExists = re-click (resolver exists) — both proceed to ensure the edge.
        Ok(_) | Err(crate::write::WriteError::TaskAlreadyExists(_)) => {}
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
    match crate::write::append_resolves_edge(&issues, &resolver, &b.finding) {
        Ok(()) => ok_json(format!("{{\"ok\":true,\"resolver\":\"{resolver}\"}}")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// Wrap a raw JSON string body in a 200 response with the right content type.
fn ok_json(body: String) -> Response {
    ([("content-type", "application/json")], body).into_response()
}

/// Cheap content fingerprint of the model files (.tracking + .engine `.sysml`): folds (path, len,
/// mtime). Drives BOTH the view cache and the change-detection SSE (D0094 serveLiveCache) — "if the
/// fingerprint is unchanged, the files didn't change". Catches out-of-purview edits (e.g. git checkout
/// rewrites mtime). ~stat-only, fast enough to poll server-side.
fn fingerprint(root: &Path) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for base in [".tracking", ".engine"] {
        for f in crate::collect_sysml(&root.join(base)) {
            if let Ok(m) = std::fs::metadata(&f) {
                f.to_string_lossy().hash(&mut h);
                m.len().hash(&mut h);
                if let Ok(t) = m.modified() {
                    if let Ok(d) = t.duration_since(std::time::UNIX_EPOCH) {
                        d.as_nanos().hash(&mut h);
                    }
                }
            }
        }
    }
    h.finish()
}

/// Serve a view's JSON from the per-fingerprint cache, recomputing ONLY when the model changed
/// (D0094 serveLiveCache) — this is what kills the per-request 2s recompute on unchanged data.
fn cached(state: &AppState, key: &str, compute: impl FnOnce(&Path) -> Result<String, crate::view::ViewError>) -> Response {
    let fp = fingerprint(&state.root);
    let hit = {
        let guard = state.cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        guard.get(key).and_then(|(cfp, json)| (*cfp == fp).then(|| json.clone()))
    };
    if let Some(json) = hit {
        return ok_json(json);
    }
    match compute(&state.root) {
        Ok(json) => {
            {
                let mut guard = state.cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                guard.insert(key.to_string(), (fp, json.clone()));
            }
            ok_json(json)
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

async fn api_orient(State(s): State<AppState>) -> Response {
    cached(&s, "orient", |r| Ok(crate::orient::compute(r).to_json()))
}

async fn api_decisions(State(s): State<AppState>) -> Response {
    cached(&s, "decisions", crate::view::decisions_report)
}

// serveBusinessNeedsView: the Business layer (Brief/Personas/Needs/UseCases) — the "what/why".
async fn api_business(State(s): State<AppState>) -> Response {
    cached(&s, "business", crate::view::business)
}

// srServeModelDrivenRegistry (Tier 1a): the model-declared launchable set (process-launcher foundation).
async fn api_launchables(State(s): State<AppState>) -> Response {
    cached(&s, "launchables", crate::view::launchables)
}

async fn api_dispositions(State(s): State<AppState>) -> Response {
    cached(&s, "dispositions", crate::view::dispositions)
}

async fn api_processes(State(s): State<AppState>) -> Response {
    cached(&s, "processes", |r| Ok(processes_json(r)))
}

async fn api_report(State(s): State<AppState>, AxPath(name): AxPath<String>) -> Response {
    cached(&s, &format!("report:{name}"), |r| crate::view::report(r, &name, false))
}

/// History reads ~/.claude transcripts (outside the fingerprint), so it is computed fresh (uncached).
async fn api_history(State(s): State<AppState>) -> Response {
    ok_json(interaction_history(&s.root))
}

/// GET /api/recent (sr15) — the git-derived recent-activity timeline. Reads git history (outside the
/// model fingerprint), so it is computed fresh (uncached); a git failure yields an empty timeline.
async fn api_recent(State(s): State<AppState>) -> Response {
    match crate::view::recent(&s.root) {
        Ok(json) => ok_json(json),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("recent error: {e}")).into_response(),
    }
}

/// GET /api/item/:name (D0094 serveItemIntrospect) — any item's detail (attrs + edges + neighbors).
async fn api_item(State(s): State<AppState>, AxPath(name): AxPath<String>) -> Response {
    cached(&s, &format!("item:{name}"), |r| crate::view::item_detail(r, &name))
}

/// A bounded-section request (sr18ServeSectionCritique): exactly one of `view` (a declared view name)
/// or `element` (an element + its 1-hop typed-edge neighbourhood).
#[derive(serde::Deserialize)]
struct SectionReq {
    view: Option<String>,
    element: Option<String>,
}

/// GET /api/section?view=NAME | ?element=NAME (sr18) — render a bounded section as JSON
/// (`{seed, kind, count, items[], edges[]}`) for local, section-scoped critique. A computed `#View`.
async fn api_section(State(s): State<AppState>, Query(q): Query<SectionReq>) -> Response {
    match crate::view::section_json(&s.root, q.view.as_deref(), q.element.as_deref()) {
        Ok(json) => ok_json(json),
        Err(e) => (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// A Need-slice boundary request (sr19): the Need whose slice (internals + interfaces) to compute.
#[derive(serde::Deserialize)]
struct BoundaryReq {
    need: String,
}

/// GET /api/boundary?need=NAME (sr19) — a Need-slice boundary: white-box internal elements + black-box
/// interface cut edges + coupling count, as JSON. A computed `#View`.
async fn api_boundary(State(s): State<AppState>, Query(q): Query<BoundaryReq>) -> Response {
    match crate::view::boundary_json(&s.root, &q.need) {
        Ok(json) => ok_json(json),
        Err(e) => (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// GET /api/boundary-sweep (sr19) — the tier-satisfaction white-box sweep: per Need, slice size, coupling,
/// SR count, decomposed/verified status. A computed `#View`.
async fn api_boundary_sweep(State(s): State<AppState>) -> Response {
    cached(&s, "boundary-sweep", crate::view::boundary_sweep_json)
}

/// The .tracking file declaring `action <task>;` (so a downstream `TestResult` can be appended to it).
fn find_task_file(root: &Path, task: &str) -> Option<PathBuf> {
    let needle = format!("action {task};");
    crate::collect_sysml(&root.join(".tracking")).into_iter().find(|f| std::fs::read_to_string(f).is_ok_and(|t| t.contains(&needle)))
}

/// A request to append a downstream `TestResult` to an action task (D0094 serveItemActions).
#[derive(serde::Deserialize)]
struct TrReq {
    task: String,
    verdict: Option<String>,
    judged_at: String,
    judged_by: Option<String>,
}

/// POST /api/testresult (D0094 serveItemActions) — append a `TestResult` downstream of an action task via
/// the write API (`append_result`). `judgedAgainst` = git HEAD; never auto-commits.
async fn api_testresult(State(s): State<AppState>, axum::Json(b): axum::Json<TrReq>) -> Response {
    let verdict = b.verdict.unwrap_or_else(|| "pass".to_string());
    if verdict != "pass" && verdict != "fail" {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"verdict must be pass or fail\"}".to_string()).into_response();
    }
    let Some(file) = find_task_file(&s.root, &b.task) else {
        return (StatusCode::NOT_FOUND, format!("{{\"error\":\"no `action {}` found in .tracking\"}}", b.task.replace('"', "'"))).into_response();
    };
    let by = b.judged_by.filter(|x| !x.is_empty()).unwrap_or_else(|| "wweatherholtz".to_string());
    let sha = git_head(&s.root);
    match crate::write::append_result(&file, &b.task, &sha, &verdict, &b.judged_at, &by) {
        Ok(name) => ok_json(format!("{{\"ok\":true,\"name\":\"{name}\",\"verdict\":\"{verdict}\"}}")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// GET /api/events (D0094 serveLiveCache) — SSE change-push: poll the content fingerprint server-side
/// (~1.5s) and emit a `changed` event only when it flips, so the UI refetches event-driven (not blind
/// polling). `ping` keepalives in between; `hello` carries the initial fingerprint.
async fn api_events(State(s): State<AppState>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let root = (*s.root).clone();
    let stream = async_stream::stream! {
        let mut last = fingerprint(&root);
        yield Ok(Event::default().event("hello").data(last.to_string()));
        loop {
            tokio::time::sleep(Duration::from_millis(1500)).await;
            let now = fingerprint(&root);
            if now == last {
                yield Ok(Event::default().event("ping").data(""));
            } else {
                last = now;
                yield Ok(Event::default().event("changed").data(now.to_string()));
            }
        }
    };
    Sse::new(stream)
}

/// The engine's processes-in-use: each `.engine/processes/*.sysml` + its `Process` title/purpose.
fn processes_json(root: &Path) -> String {
    let dir = root.join(".engine").join("processes");
    let mut rows: Vec<Json> = Vec::new();
    for f in crate::collect_sysml(&dir) {
        let name = f.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        let attr = |key: &str| -> String {
            let needle = format!(":>> {key} = \"");
            text.split(needle.as_str()).nth(1).and_then(|s| s.split('"').next()).unwrap_or("").to_string()
        };
        // The `part <name> : Process` item name (so the console can introspect it via /api/item).
        let item = text
            .lines()
            .find_map(|l| l.trim_start().strip_prefix("part ").filter(|r| r.contains(": Process")).map(|r| r.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect::<String>()))
            .unwrap_or_default();
        rows.push(Json::Obj(vec![
            ("file".to_string(), Json::s(name)),
            ("name".to_string(), Json::s(item)),
            ("title".to_string(), Json::s(attr("title"))),
            ("purpose".to_string(), Json::s(attr("purpose"))),
        ]));
    }
    Json::Obj(vec![("processes".to_string(), Json::Arr(rows))]).dump()
}

// ── interaction history (D0094 m1) — a read-only lens over the Claude Code session transcripts ──
// Renders the AI<->user conversation from ~/.claude/projects/<encoded-cwd>/*.jsonl. NEVER copied into
// the model (compute-don't-store, §2.1) — the transcript is Claude Code's artifact; this is a view.

/// Claude Code encodes the launch cwd into the projects-dir name by mapping every non-alphanumeric
/// character to `-` (e.g. `C:\Users\...\keel-ai-toolkit` -> `C--Users-...-keel-ai-toolkit`).
fn encoded_project_dir(root: &Path) -> Option<PathBuf> {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))?;
    let abs = std::fs::canonicalize(root).ok()?;
    let raw = abs.to_string_lossy();
    let stripped = raw.strip_prefix(r"\\?\").unwrap_or(&raw);
    let enc: String = stripped.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    Some(Path::new(&home).join(".claude").join("projects").join(enc))
}

/// Best-effort text of a transcript line's message content (string, or concatenated text blocks).
fn message_text(v: &serde_json::Value) -> String {
    let Some(content) = v.get("message").and_then(|m| m.get("content")) else { return String::new() };
    if let Some(s) = content.as_str() {
        return s.to_string();
    }
    if let Some(arr) = content.as_array() {
        let mut out = String::new();
        for block in arr {
            if block.get("type").and_then(serde_json::Value::as_str) == Some("text") {
                if let Some(t) = block.get("text").and_then(serde_json::Value::as_str) {
                    out.push_str(t);
                    out.push('\n');
                }
            }
        }
        return out.trim().to_string();
    }
    String::new()
}

/// Truncate to `n` chars (char-safe), appending an ellipsis when cut.
fn clip(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        return s.to_string();
    }
    let head: String = s.chars().take(n).collect();
    format!("{head}\u{2026}")
}

/// One session's user/assistant turns (text only), oldest first; non-conversation lines skipped.
fn session_entries(path: &Path) -> Vec<Json> {
    let Ok(text) = std::fs::read_to_string(path) else { return Vec::new() };
    let mut entries = Vec::new();
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else { continue };
        let role = v.get("type").and_then(serde_json::Value::as_str).unwrap_or("");
        if role != "user" && role != "assistant" {
            continue;
        }
        let body = message_text(&v);
        if body.is_empty() {
            continue;
        }
        entries.push(Json::Obj(vec![
            ("role".to_string(), Json::s(role.to_string())),
            ("text".to_string(), Json::s(clip(&body, 4000))),
        ]));
    }
    entries
}

/// The AI<->user interaction history as JSON: the session list (newest first) + the latest session's
/// turns. A read-only lens; nothing is stored.
#[must_use]
pub fn interaction_history(root: &Path) -> String {
    let Some(dir) = encoded_project_dir(root) else {
        return Json::Obj(vec![("available".to_string(), Json::Bool(false)), ("note".to_string(), Json::s("no home dir resolvable".to_string()))]).dump();
    };
    let Ok(rd) = std::fs::read_dir(&dir) else {
        return Json::Obj(vec![
            ("available".to_string(), Json::Bool(false)),
            ("note".to_string(), Json::s(format!("no transcripts at {}", dir.display()))),
        ])
        .dump();
    };
    // (path, modified-seconds) for each .jsonl, newest first.
    let mut files: Vec<(PathBuf, u64)> = Vec::new();
    for entry in rd.flatten() {
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()) != Some("jsonl") {
            continue;
        }
        let mtime = entry
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map_or(0, |d| d.as_secs());
        files.push((p, mtime));
    }
    files.sort_by_key(|f| std::cmp::Reverse(f.1));
    let sessions: Vec<Json> = files
        .iter()
        .map(|(p, mtime)| {
            let id = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
            let turns = std::fs::read_to_string(p).map_or(0, |t| t.lines().count());
            Json::Obj(vec![
                ("id".to_string(), Json::s(id)),
                ("modified".to_string(), Json::Int(i64::try_from(*mtime).unwrap_or(i64::MAX))),
                ("lines".to_string(), Json::Int(i64::try_from(turns).unwrap_or(i64::MAX))),
            ])
        })
        .collect();
    let current = files.first().map_or_else(|| Json::Arr(Vec::new()), |(p, _)| Json::Arr(session_entries(p)));
    let current_id = files.first().map_or_else(String::new, |(p, _)| p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string());
    Json::Obj(vec![
        ("available".to_string(), Json::Bool(true)),
        ("dir".to_string(), Json::s(dir.display().to_string())),
        ("current_id".to_string(), Json::s(current_id)),
        ("sessions".to_string(), Json::Arr(sessions)),
        ("current".to_string(), current),
    ])
    .dump()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::claude_in_dirs;
    use std::ffi::OsString;

    /// serveDownstreamDegrade: the agent bridge is OPTIONAL. `claude_in_dirs` must report absent
    /// when no `claude` executable sits on PATH — so the console can emit a clear "not installed"
    /// message instead of a cryptic exit code. Empty dir -> false; dropping the right-named file
    /// in -> true. The executable basename differs by platform (`claude.cmd` on Windows shims).
    #[test]
    fn detects_claude_presence_on_path() {
        let dir = std::env::temp_dir().join(format!("keel_claude_probe_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path: OsString = dir.clone().into_os_string();

        // No claude anywhere on this one-entry PATH.
        assert!(!claude_in_dirs(&path, None), "should report absent in an empty dir");

        // Drop a claude executable named the way the platform resolves it.
        let name = if cfg!(windows) { "claude.cmd" } else { "claude" };
        std::fs::write(dir.join(name), b"").unwrap();
        assert!(claude_in_dirs(&path, None), "should detect {name} on PATH");

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// sr18 — a section seed parses to the `(view, element)` pair `section_json` expects: an explicit
    /// `view:`/`element:` prefix routes accordingly; a bare name defaults to an element seed.
    #[test]
    fn section_seed_parses_view_element_and_bare() {
        use super::parse_section_seed;
        assert_eq!(parse_section_seed("view:orphans"), (Some("orphans".to_string()), None));
        assert_eq!(parse_section_seed("element:sr18ServeSectionCritique"), (None, Some("sr18ServeSectionCritique".to_string())));
        assert_eq!(parse_section_seed("d0098"), (None, Some("d0098".to_string())));
    }

    /// sr18 — without a section the critique prompt is the plain element critique; WITH section members
    /// it becomes section-scoped (names the bounded neighbourhood + asks for in-context judgment). Both
    /// forms forbid committing (the human's gate, D0016).
    #[test]
    fn agent_prompt_is_section_scoped_only_when_members_given() {
        use super::build_agent_prompt;
        let plain = build_agent_prompt("sr18", None);
        assert!(plain.contains("critique `sr18`"));
        assert!(!plain.contains("bounded SECTION"));
        assert!(plain.contains("Do NOT git commit"));

        let scoped = build_agent_prompt("sr18", Some(&["sr18".to_string(), "n17ServeGranularWhitebox".to_string()]));
        assert!(scoped.contains("bounded SECTION"));
        assert!(scoped.contains("n17ServeGranularWhitebox"));
        assert!(scoped.contains("Do NOT git commit"));

        // An empty member set must not fabricate a section clause.
        let empty = build_agent_prompt("sr18", Some(&[]));
        assert!(!empty.contains("bounded SECTION"));
    }

    /// sr19 — the black-box prompt critiques the boundary's INTERFACES (cut edges), names them, asks the
    /// integration concerns (necessity/minimality/completeness), and forbids committing. An empty
    /// interface set is stated as self-contained, not fabricated.
    #[test]
    fn blackbox_prompt_critiques_named_interfaces() {
        use super::build_blackbox_prompt;
        let p = build_blackbox_prompt("n17", &["satisfy n17 -> sr99".to_string(), "allocate sr1 -> compX".to_string()]);
        assert!(p.contains("Black-box"));
        assert!(p.contains("n17"));
        assert!(p.contains("satisfy n17 -> sr99"));
        assert!(p.contains("necessity"));
        assert!(p.contains("Do NOT git commit"));

        let none = build_blackbox_prompt("n17", &[]);
        assert!(none.contains("self-contained"));
    }
}
