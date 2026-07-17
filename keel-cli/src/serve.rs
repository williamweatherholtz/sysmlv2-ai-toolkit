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

/// The committed keel read-API version (`viewerKeelApi`, D0114 shape B).
///
/// `SemVer`: a breaking change to any `/api/*` read contract bumps the major version. A separate viewer
/// app pins this; `GET /api/version` reports it.
pub const KEEL_API_VERSION: &str = "1.15.0";

/// The stable, committed read endpoints a viewer may depend on (the versioned contract surface).
const KEEL_API_READ_ENDPOINTS: &[&str] = &[
    "/api/version", "/api/schema", "/api/review-queue", "/api/orient", "/api/business", "/api/decisions",
    "/api/dispositions", "/api/processes", "/api/launchables", "/api/report/:name", "/api/history", "/api/recent",
    "/api/item/:name", "/api/section", "/api/slice", "/api/change-impact", "/api/snapshot", "/api/baseline-compare",
    "/api/critique-plan", "/api/boundary", "/api/boundary-sweep", "/api/events", "/api/check", "/api/fingerprint", "/api/index", "/api/relations",
];

/// The committed WRITE endpoints a viewer may drive to change the model THROUGH keel processes + the
/// guarded write API (N-16 `viewerInProgramEdit` / D0117 generative UI — the write half of the surface,
/// advertised in `/api/version` so a viewer discovers actions rather than hardcoding them). Every write
/// goes through the write API + guards; none auto-commits (the human commits). `/api/decision` scaffolds
/// a `status=proposed` Decision — acceptance stays a separate explicit human gate (D0106).
const KEEL_API_WRITE_ENDPOINTS: &[&str] = &[
    "/api/decision", "/api/decision/accept", "/api/decision/reject", "/api/gate-result", "/api/disposition", "/api/testresult", "/api/resolver", "/api/edge", "/api/item",
];

/// The `SysML` declaration keyword for a created item, by its type's meta-kind (D0126, `/api/item`). Keeps
/// CREATE generative + correct: a `requirement`/`use case`/`verification`-def type must be instantiated
/// with the matching keyword (a bare `part` would not conform). Anything else defaults to `part`.
fn item_keyword(type_name: &str) -> &'static str {
    match type_name {
        "Need" | "SystemRequirement" | "SubsystemRequirement" | "ComponentRequirement" | "Requirement" => "requirement",
        "UseCase" => "use case",
        "Test" | "TestPlan" => "verification",
        _ => "part",
    }
}

/// The edge kinds the in-program write surface (`/api/edge`, N-16 + `viewerCreateLinkage` D0126) is
/// permitted to author — the closed algebra: native `satisfy`/`allocate` + the governance markers. The
/// viewer authors typed traceability THROUGH the process, never arbitrary text. Extend by adding a kind.
const AUTHORABLE_EDGE_KINDS: &[&str] = &["satisfy", "allocate", "Supersede", "DependsOn", "DerivedFrom", "Covers", "Resolves", "Dispositions"];

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
        // viewerKeelApi (D0114 shape B / N-6): the COMMITTED, VERSIONED read API contract. A separate
        // viewer app consumes keel through it; breaking changes bump KEEL_API_VERSION.
        .route("/api/version", get(api_version))
        // viewerSchemaApi (N-17/D0117) — declared types + attributes, the generative-UI substrate
        .route("/api/schema", get(api_schema))
        // review queue (D0121) — user-gated items awaiting human judgment (read side of the loop)
        .route("/api/review-queue", get(api_review_queue))
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
        .route("/api/agent/plan", get(api_agent_plan))
        .route("/api/agent/stream", get(api_agent_stream))
        // serveLiveCache — event-driven change push (SSE)
        .route("/api/events", get(api_events))
        // serveItemIntrospect — generic any-item detail
        .route("/api/item/:name", get(api_item))
        // sr18 — bounded section render (a declared view, or an element + its 1-hop neighbourhood)
        .route("/api/section", get(api_section))
        // viewerConfigurableSlice (N-2/N-4/N-10) — seed + configurable depth/edges/direction
        .route("/api/slice", get(api_slice))
        .route("/api/index", get(api_index))
        .route("/api/relations", get(api_relations))
        // viewerChangeImpact (N-10) — blast radius from a focus, grouped by distance
        .route("/api/change-impact", get(api_change_impact))
        // viewerExportShare (N-12) — a viewpoint snapshot stamped with commit + as-of + scope
        .route("/api/snapshot", get(api_snapshot))
        // viewerBaselineCompare (N-13) — diff the viewpoint between two commits
        .route("/api/baseline-compare", get(api_baseline_compare))
        // viewerIterativeCritique (N-15) — deterministic iteration plan over a slice (axis + context + lens)
        .route("/api/critique-plan", get(api_critique_plan))
        // sr19 — Need-slice boundary (white-box internals + black-box interfaces) + the tier sweep
        .route("/api/boundary", get(api_boundary))
        .route("/api/boundary-sweep", get(api_boundary_sweep))
        // serveItemActions — append a downstream TestResult to a task
        .route("/api/testresult", post(api_testresult))
        // sr16 — on ACT, attach a tracked #Resolves resolver task to a finding
        .route("/api/resolver", post(api_resolver))
        // viewerInProgramEdit (N-16/D0117) — scaffold a PROPOSED Decision via the keel record process
        .route("/api/decision", post(api_decision))
        // review queue (D0121) — record human acceptance/rejection as fact: accept/reject a Decision, accept/reject a gate
        .route("/api/decision/accept", post(api_decision_accept))
        .route("/api/decision/reject", post(api_decision_reject))
        .route("/api/gate-result", post(api_gate_result))
        .route("/api/edge", post(api_edge))
        .route("/api/item", post(api_create_item))
        .route("/api/check", get(api_check))
        .route("/api/fingerprint", get(api_fingerprint))
        .layer(middleware::from_fn(log_request))
        // viewerKeelApi (D0114 shape B): let a separate local viewer app consume /api/* cross-port
        .layer(middleware::from_fn(cors_localhost))
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
    println!("  api:   /api/version (committed read API v{KEEL_API_VERSION}, viewerKeelApi/D0114 shape B)");
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

/// True for a browser `Origin` that is localhost/127.0.0.1 on any port (or none).
fn is_localhost_origin(o: &str) -> bool {
    matches!(o, "http://localhost" | "http://127.0.0.1")
        || o.starts_with("http://localhost:")
        || o.starts_with("http://127.0.0.1:")
}

/// Set the localhost-CORS response headers (reflect the caller's origin; allow the API's verbs + JSON).
fn add_cors_headers(h: &mut axum::http::HeaderMap, origin: &str) {
    use axum::http::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, VARY};
    use axum::http::HeaderValue;
    if let Ok(v) = HeaderValue::from_str(origin) {
        h.insert(ACCESS_CONTROL_ALLOW_ORIGIN, v);
    }
    h.insert(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, POST, OPTIONS"));
    h.insert(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("content-type"));
    h.insert(VARY, HeaderValue::from_static("Origin"));
}

/// Localhost-only CORS middleware (`viewerKeelApi` / D0114 shape B): a SEPARATE viewer app served from
/// another local port must be able to `fetch` `/api/*`. Reflects a localhost/127.0.0.1 `Origin` (any
/// port), advertises the API's verbs, and short-circuits the `OPTIONS` preflight with 204. Non-local
/// origins get no CORS headers — and the server is already `127.0.0.1`-bound, so this only enables the
/// intended local-cross-port case (shape B), not remote access.
async fn cors_localhost(req: Request, next: Next) -> Response {
    let local_origin = req
        .headers()
        .get(axum::http::header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string)
        .filter(|o| is_localhost_origin(o));
    if req.method() == axum::http::Method::OPTIONS {
        let mut resp = StatusCode::NO_CONTENT.into_response();
        if let Some(o) = &local_origin {
            add_cors_headers(resp.headers_mut(), o);
        }
        return resp;
    }
    let mut resp = next.run(req).await;
    if let Some(o) = &local_origin {
        add_cors_headers(resp.headers_mut(), o);
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

/// ISO-8601 commit date of HEAD (the snapshot's `as-of`); `"unknown"` if git fails.
fn git_head_date(root: &Path) -> String {
    std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["log", "-1", "--format=%cs", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map_or_else(|| "unknown".to_string(), |o| String::from_utf8_lossy(&o.stdout).trim().to_string())
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

#[derive(serde::Deserialize)]
struct DecisionReq {
    slug: String,
    title: String,
    context: String,
    decision: String,
    rationale: String,
    consequences: String,
    date: String,
    author: Option<String>,
}

/// POST /api/decision (viewerInProgramEdit, N-16/D0117) — scaffold a PROPOSED Decision through the keel
/// `record decision` process (D0105 RMWX axis). Reuses `write::record_decision`: auto NNNN + UUID,
/// `status=proposed`. Acceptance is a SEPARATE explicit human gate (D0106) — this never fabricates the
/// acceptance event, and never auto-commits (the human reviews + commits). The generated UI proposes
/// changes THROUGH the process, not by editing facts directly ("not going rogue").
async fn api_decision(State(s): State<AppState>, axum::Json(b): axum::Json<DecisionReq>) -> Response {
    let author = b.author.filter(|a| !a.is_empty()).unwrap_or_else(|| "wweatherholtz".to_string());
    if b.slug.is_empty() || b.title.is_empty() || b.decision.is_empty() {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"slug, title, and decision are required\"}".to_string()).into_response();
    }
    match crate::write::record_decision(&s.root, &b.slug, &b.title, &b.date, &author, &b.context, &b.decision, &b.rationale, &b.consequences) {
        Ok((nnnn, path)) => ok_json(format!("{{\"ok\":true,\"decision\":\"D{nnnn}\",\"path\":\"{path}\",\"status\":\"proposed\"}}")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

#[derive(serde::Deserialize)]
struct DecisionAcceptReq {
    decision: String,
    file: String,
    note: String,
    judged_at: String,
    judged_by: Option<String>,
}

/// POST /api/decision/accept (D0121 review queue) — ACCEPT a proposed Decision: flip status + append
/// the `{decision}Accept` event via `write::accept_decision`. The human's note IS the attestation
/// (D0106 — `judged_by` is a Person, never AI-fabricated); never auto-commits.
async fn api_decision_accept(State(s): State<AppState>, axum::Json(b): axum::Json<DecisionAcceptReq>) -> Response {
    let Some(path) = safe_repo_path(&s.root, &b.file) else {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"file must be a repo-relative .sysml path\"}".to_string()).into_response();
    };
    let judged_by = b.judged_by.filter(|a| !a.is_empty()).unwrap_or_else(|| "wweatherholtz".to_string());
    let sha = git_head(&s.root);
    match crate::write::accept_decision(&path, &b.decision, &sha, &b.judged_at, &judged_by, &b.note) {
        Ok(_) => ok_json(format!("{{\"ok\":true,\"decision\":\"{}\",\"status\":\"accepted\"}}", b.decision)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

#[derive(serde::Deserialize)]
struct DecisionRejectReq {
    decision: String,
    file: String,
    rationale: String,
    judged_at: String,
    judged_by: Option<String>,
}

/// POST /api/decision/reject (D0121/D0122 review queue) — REJECT a proposed Decision: flip status to
/// `rejected` + append the `{decision}Reject` judgment (rationale) via `write::reject_decision`. The
/// human's rationale IS the attestation (D0106); never auto-commits.
async fn api_decision_reject(State(s): State<AppState>, axum::Json(b): axum::Json<DecisionRejectReq>) -> Response {
    let Some(path) = safe_repo_path(&s.root, &b.file) else {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"file must be a repo-relative .sysml path\"}".to_string()).into_response();
    };
    if b.rationale.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"a rejection rationale is required\"}".to_string()).into_response();
    }
    let judged_by = b.judged_by.filter(|a| !a.is_empty()).unwrap_or_else(|| "wweatherholtz".to_string());
    let sha = git_head(&s.root);
    match crate::write::reject_decision(&path, &b.decision, &sha, &b.judged_at, &judged_by, &b.rationale) {
        Ok(_) => ok_json(format!("{{\"ok\":true,\"decision\":\"{}\",\"status\":\"rejected\"}}", b.decision)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

#[derive(serde::Deserialize)]
struct GateResultReq {
    gate: String,
    file: String,
    judged_at: String,
    verdict: Option<String>,
    note: Option<String>,
    judged_by: Option<String>,
}

/// POST /api/gate-result (D0121 review queue) — ACCEPT a pending confirmation gate: append a passing
/// `{gate}R{n}` `TestResult` via `write::append_gate_result` (the human's action = the sign-off, D0106;
/// optional note recorded as `notes`). Never auto-commits.
async fn api_gate_result(State(s): State<AppState>, axum::Json(b): axum::Json<GateResultReq>) -> Response {
    let Some(path) = safe_repo_path(&s.root, &b.file) else {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"file must be a repo-relative .sysml path\"}".to_string()).into_response();
    };
    let judged_by = b.judged_by.filter(|a| !a.is_empty()).unwrap_or_else(|| "wweatherholtz".to_string());
    let sha = git_head(&s.root);
    let note = b.note.as_deref().filter(|t| !t.is_empty());
    let verdict = match b.verdict.as_deref() {
        Some("fail") => "fail",
        _ => "pass",
    };
    match crate::write::append_gate_result(&path, &b.gate, &sha, verdict, &b.judged_at, &judged_by, note) {
        Ok(_) => ok_json(format!("{{\"ok\":true,\"gate\":\"{}\",\"outcome\":\"{verdict}\"}}", b.gate)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

#[derive(serde::Deserialize)]
struct EdgeReq {
    /// Target file (repo-relative .sysml); defaults to `.tracking/authored.sysml` when omitted.
    file: Option<String>,
    /// The edge kind (closed algebra): `satisfy`/`allocate` or a governance marker. `marker` is a legacy alias.
    kind: Option<String>,
    marker: Option<String>,
    from: String,
    to: String,
}

/// POST /api/edge (viewerInProgramEdit N-16 + viewerCreateLinkage D0126) — author a typed traceability
/// edge THROUGH the guarded write API (append-only, idempotent): native `satisfy`/`allocate` or a
/// governance marker (`#Kind dependency from…to…`). `kind` is whitelisted (`AUTHORABLE_EDGE_KINDS`),
/// endpoints are identifier-shaped, and `file` defaults to `.tracking/authored.sysml` (created if absent)
/// — the viewer changes facts through the process, never by free text. Never auto-commits; run `/api/check`.
async fn api_edge(State(s): State<AppState>, axum::Json(b): axum::Json<EdgeReq>) -> Response {
    let kind = b.kind.or(b.marker).unwrap_or_default();
    if !AUTHORABLE_EDGE_KINDS.contains(&kind.as_str()) {
        return (StatusCode::BAD_REQUEST, format!("{{\"error\":\"edge kind '{}' not permitted; allowed: {}\"}}", kind.replace('"', "'"), AUTHORABLE_EDGE_KINDS.join(", "))).into_response();
    }
    let ident = |x: &str| !x.is_empty() && x.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if !ident(&b.from) || !ident(&b.to) {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"from and to must be bare SysML identifiers\"}".to_string()).into_response();
    }
    let file_rel = b.file.filter(|f| !f.is_empty()).unwrap_or_else(|| ".tracking/authored.sysml".to_string());
    if safe_repo_path(&s.root, &file_rel).is_none() {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"file must be a repo-relative .sysml path\"}".to_string()).into_response();
    }
    match crate::write::author_edge(&s.root, &file_rel, &kind, &b.from, &b.to) {
        Ok(()) => ok_json(format!("{{\"ok\":true,\"edge\":\"{kind} {} -> {}\"}}", b.from, b.to)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

#[derive(serde::Deserialize)]
struct StrAttr { name: String, value: String }
#[derive(serde::Deserialize)]
struct EnumAttr { name: String, #[serde(rename = "enumType")] enum_type: String, value: String }
#[derive(serde::Deserialize)]
struct CreateItemReq {
    #[serde(rename = "type")]
    type_name: String,
    name: Option<String>,
    #[serde(default)]
    string_attrs: Vec<StrAttr>,
    #[serde(default)]
    enum_attrs: Vec<EnumAttr>,
    author: Option<String>,
    date: String,
}

/// POST /api/item (viewerAuthoringEndpoints, D0126) — create a new item of a declared type through the
/// guarded write path (`write::create_item`): generated UUID + provenance, string + enum attrs, into
/// `.tracking/authored.sysml`. Additive only, never auto-commits; run `/api/check` after to surface any
/// guard obligation (e.g. an untriaged Issue) inline. The keyword is derived from the type's meta-kind.
async fn api_create_item(State(s): State<AppState>, axum::Json(b): axum::Json<CreateItemReq>) -> Response {
    let ty = b.type_name.trim();
    if ty.is_empty() || !ty.chars().all(|c| c.is_ascii_alphanumeric()) {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"type must be a declared type name\"}".to_string()).into_response();
    }
    let author = b.author.filter(|a| !a.is_empty()).unwrap_or_else(|| "wweatherholtz".to_string());
    let strs: Vec<(String, String)> = b.string_attrs.into_iter().map(|a| (a.name, a.value)).collect();
    let enums: Vec<(String, String, String)> = b.enum_attrs.into_iter().map(|a| (a.name, a.enum_type, a.value)).collect();
    let new_item = crate::write::NewItem {
        keyword: item_keyword(ty), type_name: ty, name_hint: b.name.as_deref().unwrap_or(""),
        string_attrs: &strs, enum_attrs: &enums, author: &author, created_at: &b.date,
    };
    match crate::write::create_item(&s.root, &new_item) {
        Ok((name, path)) => ok_json(format!("{{\"ok\":true,\"name\":\"{name}\",\"type\":\"{ty}\",\"path\":\"{path}\"}}")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// GET /api/check (viewerInProgramEdit, N-16) — run the honest-state gates (parse-validate + all
/// `GUARD_NAMES`) against the working tree and return the verdict, so an in-program write that would be
/// REJECTED is surfaced inline (D0098 honest state — the same gates the pre-commit hook enforces). Returns
/// `{ok, blocking:[{guard,violations[]}], warnings:[{guard,warnings[]}], parseErrors:[…]}`. Not cached
/// (the point is to reflect the just-written working tree); `block_in_place` so guards' git shell-outs
/// don't starve the runtime.
async fn api_check(State(s): State<AppState>) -> Response {
    let json = tokio::task::block_in_place(|| {
        let root = s.root.as_path();
        let report = crate::validate_root(root);
        let mut parse_errors: Vec<Json> = report.errors.iter()
            .map(|e| Json::s(format!("{}: {}", e.file.display(), e.message)))
            .collect();
        parse_errors.extend(report.diagnostics.iter().map(|(p, d)| Json::s(format!("{}: {}", p.display(), d.message))));
        let mut blocking: Vec<Json> = Vec::new();
        let mut warnings: Vec<Json> = Vec::new();
        for name in crate::guards::GUARD_NAMES {
            if let Some(rep) = crate::guards::run_one(name, root) {
                if !rep.violations.is_empty() {
                    blocking.push(Json::Obj(vec![
                        ("guard".to_string(), Json::s(name.to_string())),
                        ("violations".to_string(), Json::Arr(rep.violations.iter().map(|v| Json::s(v.clone())).collect())),
                    ]));
                }
                if !rep.warnings.is_empty() {
                    warnings.push(Json::Obj(vec![
                        ("guard".to_string(), Json::s(name.to_string())),
                        ("warnings".to_string(), Json::Arr(rep.warnings.iter().map(|v| Json::s(v.clone())).collect())),
                    ]));
                }
            }
        }
        let ok = parse_errors.is_empty() && blocking.is_empty();
        Json::Obj(vec![
            ("ok".to_string(), Json::Bool(ok)),
            ("parseErrors".to_string(), Json::Arr(parse_errors)),
            ("blocking".to_string(), Json::Arr(blocking)),
            ("warnings".to_string(), Json::Arr(warnings)),
        ]).dump()
    });
    ok_json(json)
}

/// GET /api/fingerprint (viewerInProgramEdit, N-16 / D0108) — the model's current content fingerprint.
/// A viewer captures it when a write form opens and re-checks at submit: a changed fingerprint means the
/// model moved underneath (a possible concurrent edit, D0108) — the viewer flags a conflict rather than
/// silently overwriting. Cheap (stat-only), never cached.
async fn api_fingerprint(State(s): State<AppState>) -> Response {
    ok_json(format!("{{\"fingerprint\":\"{}\"}}", fingerprint(&s.root)))
}

/// Resolve a repo-relative `.sysml` path safely (no absolute paths, no `..` traversal, stays under root).
fn safe_repo_path(root: &Path, rel: &str) -> Option<PathBuf> {
    if rel.is_empty() || rel.contains("..") {
        return None;
    }
    let p = std::path::Path::new(rel);
    if p.is_absolute() || p.extension().is_none_or(|e| !e.eq_ignore_ascii_case("sysml")) {
        return None;
    }
    Some(root.join(p))
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
    /// srServeApproveGate (Tier 2b) — the execute stream refuses to run unless this is true; the console
    /// sets it only after the human reviews the /api/agent/plan output and clicks approve.
    #[serde(default)]
    approved: bool,
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
/// Launch prompt (srServeLauncherDefinedOnly, Tier 2a): execute a DECLARED process/skill by name. The
/// agent reads the launchable's definition from `.engine`; this prompt just directs it to follow that
/// definition + record tracked facts, never commit. Only reached for an `is_launchable` target.
fn build_launch_prompt(target: &str) -> String {
    format!(
        "Deploy/execute the DECLARED keel process or skill `{target}` exactly per its definition in `.engine` \
         (its steps / purpose / write-policy). Produce its declared artifacts as tracked facts (append via the \
         write API where applicable); stay strictly within that process — do not freelance beyond it. Do NOT git commit; the human commits."
    )
}

/// The computed PLAN for an agent request (srServeApproveGate, Tier 2b): what WOULD run, computed without
/// spawning — so the human can review + approve before execution. `action_ok`/`launch_undefined` are the
/// same validity checks the stream enforces; `prompt` is exactly what the agent would receive.
fn request_plan(root: &Path, q: &AgentReq) -> (bool, bool, String) {
    let action_ok = matches!(q.action.as_str(), "critique" | "launch");
    let launch_undefined = q.action == "launch" && !crate::view::is_launchable(root, &q.target).unwrap_or(false);
    let prompt = if q.action == "launch" {
        build_launch_prompt(&q.target)
    } else if let Some(need) = q.boundary.as_deref() {
        let interfaces = crate::view::boundary_interfaces(root, need).unwrap_or_default();
        build_blackbox_prompt(need, &interfaces)
    } else {
        let section_members = q.section.as_deref().and_then(|seed| {
            let (view, element) = parse_section_seed(seed);
            crate::view::section_member_names(root, view.as_deref(), element.as_deref()).ok()
        });
        build_agent_prompt(&q.target, section_members.as_deref())
    };
    (action_ok, launch_undefined, prompt)
}

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
/// GET /api/agent/plan?action=&target=... (srServeApproveGate, Tier 2b) — compute what the agent WOULD
/// run (parsed action, target, validity, exact prompt) WITHOUT spawning, so the human can review before
/// approving. The console shows this, then calls /api/agent/stream with approved=1 on approval.
async fn api_agent_plan(State(s): State<AppState>, Query(q): Query<AgentReq>) -> Response {
    let (action_ok, launch_undefined, prompt) = request_plan(&s.root, &q);
    let json = crate::json::Json::Obj(vec![
        ("plan".to_string(), crate::json::Json::s("agent-request plan (srServeApproveGate) — review, then execute with approved=1")),
        ("action".to_string(), crate::json::Json::s(q.action)),
        ("target".to_string(), crate::json::Json::s(q.target)),
        ("action_ok".to_string(), crate::json::Json::Bool(action_ok)),
        ("launch_undefined".to_string(), crate::json::Json::Bool(launch_undefined)),
        ("executable".to_string(), crate::json::Json::Bool(action_ok && !launch_undefined)),
        ("prompt".to_string(), crate::json::Json::s(prompt)),
        ("requires_approval".to_string(), crate::json::Json::Bool(true)),
    ])
    .dump();
    ok_json(json)
}

async fn api_agent_stream(State(s): State<AppState>, Query(q): Query<AgentReq>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let root = (*s.root).clone();
    // sr17 critique + D0109 launch (model-driven launcher; non-freeform: a launch target must be is_launchable).
    let (action_ok, launch_undefined, prompt) = request_plan(&root, &q);
    // srServeApproveGate (Tier 2b): the execute path refuses to run without an EXPLICIT approval — the human
    // must GET /api/agent/plan, review the route/prompt, and only then re-invoke with approved=1. This makes
    // approve-before-execute structural (closes D0106's conversational residual, issue059).
    let unapproved = !q.approved;
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
            // Directed-only (sr17/D0098 + D0109): the bridge serves `critique` + `launch` of a DECLARED target — no free-form AI surface.
            yield Ok(Event::default().event("error").data("only `critique` and `launch` are supported \u{2014} the console has no free-form AI surface (sr17/D0098/D0109): every AI action is directed at a named element (critique) or a DECLARED process/skill (launch). For free-form AI, open a terminal."));
            return;
        }
        if launch_undefined {
            // srServeLauncherDefinedOnly (Tier 2a): reject a launch of a non-declared target — no freeform launch.
            yield Ok(Event::default().event("error").data(format!("`{}` is not a declared launchable (srServeLauncherDefinedOnly/D0109): only declared processes/skills may be launched \u{2014} see `keel launchables`. There is no freeform launch path.", q.target)));
            return;
        }
        if unapproved {
            // srServeApproveGate (Tier 2b): no execution without explicit approval of the reviewed plan.
            yield Ok(Event::default().event("error").data("approval required (srServeApproveGate/D0109): GET /api/agent/plan to review the parsed route + exact prompt, then re-invoke this stream with approved=1. The agent never runs on an unreviewed/unapproved route (closes issue059)."));
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
    // issue063: the compute can shell out to git (orient suspect/drift) for up to a second-plus on a cold
    // hit; run it via block_in_place so it never STARVES the multi-thread runtime's worker — other
    // requests + the SSE change-push keep flowing while this view computes. (No async refactor needed:
    // block_in_place offloads the current worker's other tasks; the serve runtime is new_multi_thread.)
    match tokio::task::block_in_place(|| compute(&state.root)) {
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

/// GET /api/version (viewerKeelApi / D0114 shape B) — the committed API version + the stable read-endpoint
/// contract a viewer app pins to. Static (no model read); the one endpoint a client hits first.
async fn api_version() -> Response {
    let eps = KEEL_API_READ_ENDPOINTS.iter().map(|e| Json::s((*e).to_string())).collect();
    let weps = KEEL_API_WRITE_ENDPOINTS.iter().map(|e| Json::s((*e).to_string())).collect();
    ok_json(Json::Obj(vec![
        ("apiVersion".to_string(), Json::s(KEEL_API_VERSION.to_string())),
        ("viewerKeelApi".to_string(), Json::s("committed read+write API for a viewpoint explorer (D0114 shape B); breaking read-contract changes bump the major version".to_string())),
        ("readEndpoints".to_string(), Json::Arr(eps)),
        ("writeEndpoints".to_string(), Json::Arr(weps)),
    ]).dump())
}

/// GET /api/schema (viewerSchemaApi, N-17/D0117) — the declared item types + attribute fields, so a
/// generative UI builds forms from the model (paired with /api/launchables for actions). Cached per
/// content fingerprint. New types/attributes appear automatically — nothing hardcoded.
async fn api_schema(State(s): State<AppState>) -> Response {
    cached(&s, "schema", crate::view::schema_json)
}

/// GET /api/review-queue (D0121) — the human review queue: user-gated items awaiting judgment
/// (proposed Decisions + pending confirmation gates). The read side of the human-oversight loop;
/// the "Review" console tab renders it and records acceptance via the write endpoints.
async fn api_review_queue(State(s): State<AppState>) -> Response {
    cached(&s, "review-queue", crate::view::review_queue_json)
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

/// A configurable-slice request (viewerConfigurableSlice, N-2/N-4/N-10).
#[derive(serde::Deserialize)]
struct SliceReq {
    seed: String,
    depth: Option<usize>,
    edges: Option<String>,
    dir: Option<String>,
    dateattr: Option<String>,
    since: Option<String>,
    until: Option<String>,
}

/// GET /api/slice?seed=NAME&depth=N&edges=a,b&dir=down|up|both[&dateattr=judgedAt&since=D&until=D]
/// (viewerConfigurableSlice N-2/N-4/N-10 + N-5 time-filter) — a configurable slice from a seed as JSON
/// (`{seed, kind, count, items[], edges[]}`). `depth` default 1; `edges` empty = all; `dir` default
/// `both` (`up` = change-impact). TIME FILTER (N-5): if `dateattr` is set (e.g. `judgedAt`), keep only
/// members whose that-attribute is in the ISO date range `[since, until]` (either bound optional).
async fn api_slice(State(s): State<AppState>, Query(q): Query<SliceReq>) -> Response {
    let depth = q.depth.unwrap_or(1);
    let edges: std::collections::HashSet<String> = q
        .edges
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(|e| e.trim().to_lowercase())
        .filter(|e| !e.is_empty())
        .collect();
    let dir = crate::view::SliceDir::parse(q.dir.as_deref().unwrap_or("both"));
    let df = crate::view::DateFilter {
        attr: q.dateattr.as_deref().filter(|a| !a.is_empty()),
        since: q.since.as_deref().filter(|d| !d.is_empty()),
        until: q.until.as_deref().filter(|d| !d.is_empty()),
    };
    match crate::view::slice_json(&s.root, &q.seed, depth, &edges, dir, df) {
        Ok(json) => ok_json(json),
        Err(e) => (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// GET /api/index (D0126 — browse-first discovery) — the browsable register of substantive items with
/// computed `displayLabel`, type, date, and edge degree; the viewer lists + filters it so a user finds
/// elements without knowing an identifier. Cached per fingerprint like the other views.
async fn api_index(State(s): State<AppState>) -> Response {
    cached(&s, "index", crate::view::index_json)
}

#[derive(serde::Deserialize)]
struct RelationsReq { focus: String, kind: Option<String> }

/// GET /api/relations?focus=NAME&kind=siblings|children|ancestry (srViewerSystemBoundViews, D0126) —
/// a system-bound analysis slice with a shifting focus: children (downstream), ancestry (upstream), or
/// siblings (same type, shared parent). Pure computed view, cached per fingerprint.
async fn api_relations(State(s): State<AppState>, Query(q): Query<RelationsReq>) -> Response {
    let kind = match q.kind.as_deref() {
        Some("ancestry") => "ancestry",
        Some("siblings") => "siblings",
        _ => "children",
    };
    let focus = q.focus;
    cached(&s, &format!("relations:{kind}:{focus}"), move |r| crate::view::relations_json(r, &focus, kind))
}

#[derive(serde::Deserialize)]
struct ChangeImpactReq {
    seed: String,
    edges: Option<String>,
    dir: Option<String>,
}

/// GET /api/change-impact?seed=NAME[&edges=a,b&dir=up|down|both] (viewerChangeImpact / N-10) — the
/// elements reachable from the focus GROUPED BY DISTANCE (blast radius); cycles counted once. `dir=up`
/// (default) = dependents (edges pointing at the focus); `edges` empty = all.
async fn api_change_impact(State(s): State<AppState>, Query(q): Query<ChangeImpactReq>) -> Response {
    let edges: std::collections::HashSet<String> = q.edges.as_deref().unwrap_or("").split(',').map(|e| e.trim().to_lowercase()).filter(|e| !e.is_empty()).collect();
    let dir = crate::view::SliceDir::parse(q.dir.as_deref().unwrap_or("up"));
    match crate::view::change_impact_json(&s.root, &q.seed, &edges, dir) {
        Ok(json) => ok_json(json),
        Err(e) => (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// GET /api/snapshot?seed=NAME&depth=N&edges=a,b&dir=... (viewerExportShare / N-12) — the slice STAMPED
/// with provenance (source commit + as-of date + scope) so it round-trips; oversized → capped subset + note.
async fn api_snapshot(State(s): State<AppState>, Query(q): Query<SliceReq>) -> Response {
    let depth = q.depth.unwrap_or(2);
    let edges: std::collections::HashSet<String> = q.edges.as_deref().unwrap_or("").split(',').map(|e| e.trim().to_lowercase()).filter(|e| !e.is_empty()).collect();
    let dir = crate::view::SliceDir::parse(q.dir.as_deref().unwrap_or("both"));
    let commit = git_head(&s.root);
    let as_of = git_head_date(&s.root);
    match crate::view::snapshot_json(&s.root, &q.seed, depth, &edges, dir, &commit, &as_of) {
        Ok(json) => ok_json(json),
        Err(e) => (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

#[derive(serde::Deserialize)]
struct BaselineReq {
    seed: String,
    from: String,
    to: String,
    depth: Option<usize>,
    edges: Option<String>,
    dir: Option<String>,
}

/// GET /api/baseline-compare?seed=NAME&from=COMMIT&to=COMMIT[&depth=&edges=&dir=] (viewerBaselineCompare
/// / N-13) — diff the viewpoint between two commits: added / removed / changed / reverified / unchanged
/// (via git worktrees); no differences → "no drift".
async fn api_baseline_compare(State(s): State<AppState>, Query(q): Query<BaselineReq>) -> Response {
    let depth = q.depth.unwrap_or(2);
    let edges: std::collections::HashSet<String> = q.edges.as_deref().unwrap_or("").split(',').map(|e| e.trim().to_lowercase()).filter(|e| !e.is_empty()).collect();
    let dir = crate::view::SliceDir::parse(q.dir.as_deref().unwrap_or("both"));
    match crate::view::baseline_compare_json(&s.root, &q.seed, &q.from, &q.to, depth, &edges, dir) {
        Ok(json) => ok_json(json),
        Err(e) => (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// A critique-plan request (viewerIterativeCritique, N-15): the slice to iterate + the lens.
#[derive(serde::Deserialize)]
struct CritiquePlanReq {
    seed: String,
    depth: Option<usize>,
    edges: Option<String>,
    dir: Option<String>,
    lens: Option<String>,
}

/// GET /api/critique-plan?seed=NAME&depth=N&edges=a,b&dir=&lens=L (viewerIterativeCritique, N-15) — the
/// deterministic iteration plan (axis + per-element context + lens) the viewer drives the agent bridge
/// over. Same seed semantics as /api/slice; `lens` default `best-practice`.
async fn api_critique_plan(State(s): State<AppState>, Query(q): Query<CritiquePlanReq>) -> Response {
    let depth = q.depth.unwrap_or(1);
    let edges: std::collections::HashSet<String> = q
        .edges
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(|e| e.trim().to_lowercase())
        .filter(|e| !e.is_empty())
        .collect();
    let dir = crate::view::SliceDir::parse(q.dir.as_deref().unwrap_or("both"));
    let lens = q.lens.as_deref().unwrap_or("best-practice");
    match crate::view::critique_plan_json(&s.root, &q.seed, depth, &edges, dir, lens) {
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
    use super::{build_launch_prompt, claude_in_dirs, is_localhost_origin, KEEL_API_READ_ENDPOINTS, KEEL_API_VERSION, KEEL_API_WRITE_ENDPOINTS};

    #[test]
    fn cors_reflects_localhost_origins_only() {
        // viewerKeelApi (D0114 shape B): a separate local viewer (any port) is allowed; remote is not.
        assert!(is_localhost_origin("http://localhost:5173"));
        assert!(is_localhost_origin("http://127.0.0.1:8080"));
        assert!(is_localhost_origin("http://localhost"));
        assert!(!is_localhost_origin("http://evil.example.com"));
        assert!(!is_localhost_origin("https://localhost.evil.com"));
        assert!(!is_localhost_origin("http://10.0.0.5:8080"));
    }
    use std::ffi::OsString;

    #[test]
    fn api_version_contract_is_self_consistent() {
        // viewerKeelApi (D0114): the version is SemVer-shaped, and the committed contract advertises
        // itself + the core read endpoints a viewer depends on.
        assert_eq!(KEEL_API_VERSION.split('.').count(), 3, "SemVer major.minor.patch: {KEEL_API_VERSION}");
        assert!(KEEL_API_READ_ENDPOINTS.contains(&"/api/version"), "contract must advertise itself");
        assert!(KEEL_API_READ_ENDPOINTS.contains(&"/api/orient"), "contract must include orient");
        assert!(KEEL_API_READ_ENDPOINTS.contains(&"/api/item/:name"), "contract must include item detail");
        assert!(KEEL_API_READ_ENDPOINTS.contains(&"/api/slice"), "contract must include the configurable slice");
        assert!(KEEL_API_READ_ENDPOINTS.contains(&"/api/critique-plan"), "contract must include the critique plan");
        assert!(KEEL_API_READ_ENDPOINTS.contains(&"/api/schema"), "contract must include the declared-model schema");
        // viewerInProgramEdit (N-16/D0117): the write half is advertised so a viewer discovers actions.
        assert!(KEEL_API_WRITE_ENDPOINTS.contains(&"/api/decision"), "write contract must include record-Decision");
        assert!(KEEL_API_WRITE_ENDPOINTS.contains(&"/api/disposition"), "write contract must include disposition");
    }

    #[test]
    fn launch_prompt_directs_a_declared_target_no_commit() {
        // srServeLauncherDefinedOnly (Tier 2a): the launch prompt names the declared target, directs
        // execution per its definition, and forbids committing (the human commits). Freeform rejection
        // is enforced upstream by is_launchable (tested in view::tests).
        let p = build_launch_prompt("doc-sync");
        assert!(p.contains("`doc-sync`"));
        assert!(p.contains("DECLARED"));
        assert!(p.contains("do not freelance") || p.contains("strictly within"));
        assert!(p.contains("Do NOT git commit"));
    }

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
