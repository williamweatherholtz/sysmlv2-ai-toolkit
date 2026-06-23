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
        .route("/api/decisions", get(api_decisions))
        .route("/api/dispositions", get(api_dispositions))
        .route("/api/processes", get(api_processes))
        .route("/api/report/:name", get(api_report))
        .route("/api/history", get(api_history))
        // m2 — deterministic actions
        .route("/api/disposition", post(api_disposition))
        .route("/view/report/:name", get(view_report))
        .route("/view/diagram", get(view_diagram))
        // m3 — agent-bridge (headless claude -> SSE)
        .route("/api/agent/stream", get(api_agent_stream))
        // serveLiveCache — event-driven change push (SSE)
        .route("/api/events", get(api_events))
        // serveItemIntrospect — generic any-item detail
        .route("/api/item/:name", get(api_item))
        // serveItemActions — append a downstream TestResult to a task
        .route("/api/testresult", post(api_testresult))
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
    println!("  agent: /api/agent/stream?action=<critique|investigate|report>&target=<x> (SSE; local `claude` CLI)");
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

#[derive(serde::Deserialize)]
struct AgentReq {
    action: String,
    target: String,
}

/// Build the agent prompt for a console action. The agent inherits CLAUDE.md discipline from the cwd;
/// every prompt forbids committing (commits/acceptance stay the human's gate, D0016).
fn build_agent_prompt(action: &str, target: &str) -> String {
    match action {
        "critique" => format!("Use the element-critique skill to adversarially critique `{target}` through its Core-3 lenses as an INDEPENDENT critic; record each finding as a severity-carrying Issue per the issue-resolution process. Do NOT git commit; the human commits."),
        "investigate" => format!("Investigate the merits of `{target}`: read its context and linked items, judge whether it is sound or low-value, and explain your reasoning plus any recommendation. Read-only analysis: do not modify the model and do NOT git commit."),
        "report" => format!("Run `keel report {target}` and summarize its health and opportunities in a few sentences. Read-only; do NOT git commit."),
        other => format!("{other}: {target}. Follow CLAUDE.md discipline; do NOT git commit."),
    }
}

/// GET /api/agent/stream?action=&target= (D0094 m3) — spawn a headless `claude` agent in the repo and
/// stream its `stream-json` events to the browser over SSE (not polling). Degrades gracefully if the
/// `claude` CLI is absent; rejects past the concurrency cap; never sets `ANTHROPIC_API_KEY`.
async fn api_agent_stream(State(s): State<AppState>, Query(q): Query<AgentReq>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let root = (*s.root).clone();
    let prompt = build_agent_prompt(&q.action, &q.target);
    let counter = Arc::clone(&s.agents);
    let prev = counter.fetch_add(1, Ordering::SeqCst);
    let over_cap = prev >= AGENT_MAX_CONCURRENT;
    // Hold the slot for the stream's lifetime; on over-cap we release immediately below.
    let slot = AgentSlot(Arc::clone(&counter));

    let stream = async_stream::stream! {
        let _slot = slot; // dropped (counter--) when the stream finishes or the client disconnects
        if over_cap {
            yield Ok(Event::default().event("error").data(format!("busy: {AGENT_MAX_CONCURRENT} agent runs already in flight \u{2014} try again shortly")));
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
        yield Ok(Event::default().event("done").data(format!("agent finished (exit {code:?})")));
    };
    Sse::new(stream)
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

/// GET /api/item/:name (D0094 serveItemIntrospect) — any item's detail (attrs + edges + neighbors).
async fn api_item(State(s): State<AppState>, AxPath(name): AxPath<String>) -> Response {
    cached(&s, &format!("item:{name}"), |r| crate::view::item_detail(r, &name))
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
    files.sort_by(|a, b| b.1.cmp(&a.1));
    let sessions: Vec<Json> = files
        .iter()
        .map(|(p, mtime)| {
            let id = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
            let turns = std::fs::read_to_string(p).map(|t| t.lines().count()).unwrap_or(0);
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
