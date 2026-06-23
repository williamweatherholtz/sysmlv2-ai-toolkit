//! `serve` (D0094 m1) — the Keel interactive console: a LOCALHOST tokio+axum server over the engine.
//!
//! The human's ACTING surface (D0094). m1 = the live READ console; m2 = DETERMINISTIC ACTIONS: record
//! a disposition (POST -> the write API) and open a full HTML report/diagram. Every read computes from
//! the existing view authority; every write goes through the write API + guards (ONE truth, no second
//! store). A request-logging middleware makes the server observable. Localhost-only; the agent-bridge
//! (m3) builds on this. Tiers degrade gracefully.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::extract::{Path as AxPath, Request, State};
use axum::http::StatusCode;
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;

use crate::json::Json;

/// The embedded single-page console frontend (self-contained, no CDN — the cytoscape precedent).
const CONSOLE_HTML: &str = include_str!("../assets/console.html");

#[derive(Clone)]
struct AppState {
    root: Arc<PathBuf>,
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
    let state = AppState { root: Arc::new(root) };
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
    println!("Keel console (D0094 m1+m2) on http://{addr}  \u{2014} Ctrl-C to stop");
    println!("  read: / · /api/{{orient,decisions,dispositions,processes,report/<name>,history}}");
    println!("  act:  POST /api/disposition · /view/report/<name> · /view/diagram");
    println!("  (requests logged below)");
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
    eprintln!("[keel serve] {method} {path} -> {} ({}ms)", resp.status().as_u16(), start.elapsed().as_millis());
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

/// Wrap a raw JSON string body in a 200 response with the right content type.
fn ok_json(body: String) -> Response {
    ([("content-type", "application/json")], body).into_response()
}

/// Wrap a `ViewError`-fallible JSON computation into a response (500 + message on error).
fn view_json(r: Result<String, crate::view::ViewError>) -> Response {
    match r {
        Ok(body) => ok_json(body),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

async fn api_orient(State(s): State<AppState>) -> Response {
    ok_json(crate::orient::compute(&s.root).to_json())
}

async fn api_decisions(State(s): State<AppState>) -> Response {
    view_json(crate::view::decisions_report(&s.root))
}

async fn api_dispositions(State(s): State<AppState>) -> Response {
    view_json(crate::view::dispositions(&s.root))
}

async fn api_processes(State(s): State<AppState>) -> Response {
    ok_json(processes_json(&s.root))
}

async fn api_report(State(s): State<AppState>, AxPath(name): AxPath<String>) -> Response {
    view_json(crate::view::report(&s.root, &name, false))
}

async fn api_history(State(s): State<AppState>) -> Response {
    ok_json(interaction_history(&s.root))
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
        rows.push(Json::Obj(vec![
            ("file".to_string(), Json::s(name)),
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
