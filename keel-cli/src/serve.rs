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

/// A short id for the console BUILD — a hash of the page's own bytes (issue153).
///
/// It changes exactly when the page changes, and is substituted into the served HTML so the marker is
/// present WITHOUT JavaScript. That matters because the case we could not distinguish for three rounds
/// was "the page's script is dead" from "the page is old" from "you are looking at something else" — and
/// a JS-rendered marker is absent in the first two, telling the reader nothing about which.
fn console_build() -> &'static str {
    static B: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    B.get_or_init(|| {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        CONSOLE_HTML.hash(&mut h);
        format!("{:x}", h.finish())[..8].to_string()
    })
}

/// The committed keel read-API version (`viewerKeelApi`, D0114 shape B).
///
/// `SemVer`: a breaking change to any `/api/*` read contract bumps the major version. A separate viewer
/// app pins this; `GET /api/version` reports it.
// 2.0.0 (issue199/issue200, D0178/K6): BREAKING — the judgment write endpoints (disposition,
// decision/accept, decision/reject, gate-result) now REQUIRE judged_by in the body, and run/answer
// requires a registered-Person `by`. The binding fallback on judgment writes was the misattribution
// vector three times; a writer that omits the judge is refused, never guessed for.
pub const KEEL_API_VERSION: &str = "2.0.0";

/// The stable, committed read endpoints a viewer may depend on (the versioned contract surface).
const KEEL_API_READ_ENDPOINTS: &[&str] = &[
    "/api/version", "/api/schema", "/api/review-queue", "/api/orient", "/api/business", "/api/decisions",
    "/api/dispositions", "/api/processes", "/api/launchables", "/api/report/:name", "/api/history", "/api/recent",
    "/api/item/:name", "/api/section", "/api/slice", "/api/change-impact", "/api/snapshot", "/api/baseline-compare",
    "/api/computed/:cmd", "/api/critique-plan", "/api/boundary", "/api/boundary-sweep", "/api/events", "/api/check", "/api/fingerprint", "/api/index", "/api/relations", "/api/grammar",
    // issue178: registered but advertised by nothing, so a viewer could not discover them.
    "/api/scope", "/api/projects",
    // issue200/issue201: the registered-Person set, so no page ships a hardcoded human name.
    "/api/persons",
];

/// The committed WRITE endpoints a viewer may drive to change the model THROUGH keel processes + the
/// guarded write API (N-16 `viewerInProgramEdit` / D0117 generative UI — the write half of the surface,
/// advertised in `/api/version` so a viewer discovers actions rather than hardcoding them). Every write
/// goes through the write API + guards; none auto-commits (the human commits). `/api/decision` scaffolds
/// a `status=proposed` Decision — acceptance stays a separate explicit human gate (D0106).
const KEEL_API_WRITE_ENDPOINTS: &[&str] = &[
    "/api/decision", "/api/decision/accept", "/api/decision/reject", "/api/gate-result", "/api/disposition", "/api/testresult", "/api/resolver", "/api/edge", "/api/item", "/api/item/attr",
    // issue178: a POST outside the declared write contract - the one that mattered, because the
    // write contract is how an automation consumer discovers what it is allowed to change (D0093).
    "/api/project",
    // issue192: the deck records a sitting review as the HUMAN critique it is.
    "/api/deck/sitting",
    // D0181/D0182 (P5): the launch form, the headless-ask proxy queue, and the console commit action.
    "/api/launch/form", "/api/run/ask", "/api/run/asks", "/api/run/answer", "/api/commit",
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

/// id -> (path, session, answer: None = pending) — the headless-ask proxy queue (D0182).
/// id → (path, session, answer). The answer carries WHO answered (issue200, D0178/K6): the approve
/// click is a human judgment, and the actor is data the gesture carries — an unattributed allow
/// would authorize a protected write with no record naming the human.
type AskQueue = HashMap<String, (String, String, Option<(bool, String)>)>;

#[derive(Clone)]
struct AppState {
    /// The ACTIVE project root. Switchable at runtime (srConsoleProjectRebind / N-C4): a supervisor
    /// overseeing several projects reaches all of them through one surface rather than running one
    /// server per project and holding the port-to-project mapping in their head — unrecorded state of
    /// exactly the kind this engine forbids everywhere else, and the first thing lost after a break.
    root: Arc<Mutex<PathBuf>>,
    /// In-flight agent-bridge runs (concurrency guardrail, D0094).
    agents: Arc<AtomicUsize>,
    /// Pending headless-ask proxies (D0182/P1.1): a launched run's pre-write hook posts an ask here
    /// and polls for the human's answer; expiry on the hook side maps to deny + a recorded
    /// obligation. id -> (path, session, answer: None=pending).
    asks: Arc<Mutex<AskQueue>>,
    /// Per-view JSON cache keyed `view -> (fingerprint, json)` (D0094 serveLiveCache): recompute a view
    /// only when the model's content fingerprint changes; a materialized #View cache (regenerable, §2.1).
    cache: Arc<Mutex<HashMap<String, (u64, String)>>>,
    /// The latest OBSERVED fingerprint, published by the single watcher task. Every SSE connection
    /// subscribes to this instead of polling: before, each open tab ran its own 604-stat poll every
    /// 1.5s, so three tabs cost 1200 stats a second to answer one question.
    changes: tokio::sync::watch::Sender<u64>,
    /// View keys with a background recompute IN FLIGHT (dcServeWarmCache). Without this, a page load
    /// firing eight requests against a changed fingerprint would start eight recomputes of the same view
    /// - a stampede that makes the first interaction after every commit slower, not faster.
    refreshing: Arc<Mutex<std::collections::HashSet<String>>>,
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

impl AppState {
    /// The active project root. Cloned out under the lock so no caller holds it across a compute.
    fn rootpath(&self) -> PathBuf {
        self.root.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    /// Point the surface at another project. REFUSES rather than falling back (srConsoleProjectRebind):
    /// a silent fallback leaves the reader believing they are looking at project B while seeing project
    /// A, which is the same class of defect as a defaulted model scope — a confident wrong answer.
    fn rebind(&self, candidate: &Path) -> Result<PathBuf, String> {
        if !candidate.join(".tracking").is_dir() || !candidate.join(".engine").is_dir() {
            return Err(format!(
                "{} is not a keel project (needs .engine/ and .tracking/) — the active project is UNCHANGED",
                candidate.display()
            ));
        }
        let canon = candidate.canonicalize().unwrap_or_else(|_| candidate.to_path_buf());
        {
            let mut g = self.root.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            (*g).clone_from(&canon);
        }
        // Every view is computed against the root, so a rebind must invalidate the whole cache or the
        // new project would be shown last project's numbers.
        if let Ok(mut c) = self.cache.lock() {
            c.clear();
        }
        Ok(canon)
    }
}

type ViewStore = Arc<Mutex<HashMap<String, (u64, String)>>>;
static SHARED_CACHE: std::sync::OnceLock<ViewStore> = std::sync::OnceLock::new();

/// The ONE computed-view store for this process (dcServeWarmCache).
///
/// A static rather than a field on `AppState` because the expensive internal callers - `obligation_count`
/// computing four views to count them - are plain functions with no state handle, and before this they
/// computed into a private local. The arrival burst therefore computed `orient` TWICE: once inside
/// /api/obligations to read one number out of it, then again for /api/orient. `AppState` still holds a
/// handle to this same Arc, so a root rebind clearing the cache clears it for both paths.
fn view_store() -> ViewStore {
    Arc::clone(SHARED_CACHE.get_or_init(|| Arc::new(Mutex::new(HashMap::new()))))
}

/// Read `key` if it was computed against the CURRENT fingerprint, else compute, store and return it.
///
/// The key is the VIEW's name, never the route's: /api/dispositions and /api/computed/dispositions serve
/// the same computation, and keying by route cached it twice and expired it twice.
fn store_or_compute(
    root: &Path,
    key: &str,
    compute: impl FnOnce(&Path) -> Result<String, crate::view::ViewError>,
) -> Option<String> {
    let fp = crate::fingerprint::of(root);
    let store = view_store();
    let hit = {
        let g = store.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        g.get(key).filter(|(cfp, _)| *cfp == fp).map(|(_, json)| json.clone())
    };
    if let Some(json) = hit {
        return Some(json);
    }
    // Computed with the lock RELEASED. Holding it across a multi-second view computation would serialise
    // the arrival burst behind whichever request arrived first - a cache that made the page slower.
    let json = compute(root).ok()?;
    {
        let mut g = store.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        g.insert(key.to_string(), (fp, json.clone()));
    }
    Some(json)
}

/// THE ONE CHANGE DETECTOR (dcServeWarmCache).
///
/// It owns three jobs that were previously spread out or duplicated per connection: observe change,
/// advance the epoch so reads re-read, and warm the arrival views before anyone asks for them.
///
/// It runs whether or not a browser is attached. If it only ran inside an SSE connection - which is where
/// the poll used to live - a server with no open tab would never advance its epoch and would serve a
/// pre-edit answer to the next arrival: a correctness bug rather than a slow path.
fn spawn_watcher(state: &AppState) {
    let root = state.rootpath();
    let st = state.clone();
    tokio::spawn(async move {
        let mut last = crate::fingerprint::compute(&root);
        let _ = st.changes.send(last);
        loop {
            tokio::time::sleep(Duration::from_millis(1500)).await;
            let now = tokio::task::spawn_blocking({
                let r = root.clone();
                move || crate::fingerprint::compute(&r)
            })
            .await
            .unwrap_or(last);
            if now != last {
                last = now;
                // ORDER MATTERS: advance the epoch FIRST so the warmers below - and any request arriving
                // while they run - read the tree rather than the memo of the old tree.
                crate::fingerprint::new_epoch();
                for (key, f) in HOT_VIEWS {
                    warm(&st, key, f, now);
                }
                let _ = st.changes.send(now);
            }
        }
    });
}

async fn serve_async(root: PathBuf, port: u16) -> i32 {
    // CANONICALISE at startup. Served as given, a root of "." has no name and no parent, so the
    // surface could not say which project was active (N-C4 requires it always be named) and sibling
    // discovery found nothing. "." is a path, not an identity.
    let root = root.canonicalize().unwrap_or(root);
    let state = AppState { root: Arc::new(Mutex::new(root)), agents: Arc::new(AtomicUsize::new(0)), asks: Arc::new(Mutex::new(AskQueue::new())), cache: view_store(), changes: tokio::sync::watch::Sender::new(0), refreshing: Arc::new(Mutex::new(std::collections::HashSet::new())) };
    spawn_watcher(&state);

    let app = Router::new()
        .route("/", get(index))
        // viewerKeelApi (D0114 shape B / N-6): the COMMITTED, VERSIONED read API contract. A separate
        // viewer app consumes keel through it; breaking changes bump KEEL_API_VERSION.
        .route("/api/version", get(api_version))
        // viewerSchemaApi (N-17/D0117) — declared types + attributes, the generative-UI substrate
        .route("/api/schema", get(api_schema))
        // srConsoleNavigationDerived (D0152/D0154): the console asks the MODEL what its
        // navigation is. There is no list of surfaces in the console, here, or anywhere else.
        .route("/api/surfaces", get(api_surfaces))
        // srConsoleObligationsOnArrival (N-C1): what is waiting on the HUMAN, classes derived
        // from the act-surface viewpoints rather than enumerated here.
        .route("/api/obligations", get(api_obligations))
        // srConsoleProjectRebind (N-C4): several projects, one surface.
        .route("/api/projects", get(api_projects))
        .route("/api/project", post(api_project_switch))
        // srConsoleModelScopeResolved (N-C3): which model am I looking at.
        .route("/api/scope", get(api_scope))
        // review queue (D0121) — user-gated items awaiting human judgment (read side of the loop)
        .route("/api/review-queue", get(api_review_queue))
        .route("/api/orient", get(api_orient))
        .route("/api/recent", get(api_recent))
        .route("/api/decisions", get(api_decisions))
        .route("/api/business", get(api_business))
        .route("/api/launchables", get(api_launchables))
        .route("/api/dispositions", get(api_dispositions))
        .route("/api/persons", get(api_persons))
        .route("/api/processes", get(api_processes))
        .route("/api/report/:name", get(api_report))
        .route("/api/computed/:cmd", get(api_computed))
        .route("/api/history", get(api_history))
        // m2 — deterministic actions
        .route("/api/disposition", post(api_disposition))
        // The deck (issue192): the mobile review surface, generated by the engine, saving through
        // THESE endpoints - the tested path. GET is uncached: it is an act surface, always current.
        .route("/deck", get(deck_page))
        .route("/api/deck/sitting", post(api_deck_sitting))
        .route("/api/launch/form", get(api_launch_form))
        .route("/api/run/ask", post(api_run_ask))
        .route("/api/run/asks", get(api_run_asks))
        .route("/api/run/answer", get(api_run_answer_poll).post(api_run_answer))
        .route("/api/commit", post(api_commit))
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
        .route("/api/grammar", get(api_grammar))
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
        .route("/api/item/attr", post(api_set_attr))
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
    println!("  deck:  /deck - the mobile obligation review; every tap saves through THIS API (issue192)");
    println!("  agent: /api/agent/stream?action=critique&target=<x> (SSE; local `claude` CLI; directed-only — sr17)");
    println!("  live:  /api/events (SSE change-push; views cached per content fingerprint)");
    println!("  (requests logged to the terminal + keel-serve.log)");
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("serve: {e}");
        return 1;
    }
    0
}

/// The console, served NO-STORE (issue143).
///
/// The console is a client-side app whose behaviour lives entirely in embedded JavaScript, so a browser
/// holding an older copy runs older LOGIC against a newer API and the mismatch is invisible from the
/// server side - every endpoint answers correctly while the page misbehaves. There is no build hash to
/// bust a cache with, so the only reliable answer is to never let it cache: the page is a few tens of KB
/// from localhost, and correctness is worth more than that request. The API responses keep their own
/// per-fingerprint caching, which is unaffected.
async fn index() -> Response {
    (
        [
            (axum::http::header::CACHE_CONTROL, "no-store, must-revalidate"),
            (axum::http::header::PRAGMA, "no-cache"),
        ],
        Html(CONSOLE_HTML.replace("__KEEL_BUILD__", console_build())),
    )
        .into_response()
}

/// Request-logging middleware (D0094 m2 observability): logs method, path, status, and elapsed ms to
/// the terminal so the server is debuggable.
async fn log_request(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    // A WRITE IS A POINT IN TIME; A READ IS NOT (dcServeWarmCache, refining issue145). Bumping the epoch
    // on EVERY request meant each of the six requests in one page load re-statted 604 files to ask a
    // question none of them had changed the answer to: measured, a cache HIT cost 137-208ms of which the
    // fingerprint was ALL of it (`fp 180ms/604 stat, parse 0ms, build x0`).
    //
    // Reads now read the memo. What advances the epoch is an OBSERVED change - the watcher below compares
    // fingerprints and bumps when they differ - or a write, which is this server's own doing and must be
    // visible to the next read immediately. NOT a timer: the module's memo condition is that nothing
    // expires on elapsed time, and an observation is not elapsed time.
    let writes = method != axum::http::Method::GET;
    if writes {
        crate::fingerprint::new_epoch();
    }
    let start = std::time::Instant::now();
    let resp = next.run(req).await;
    if writes {
        // AND AGAIN AFTER. Bumping only before is not enough and the difference is the console's own
        // acts: the handler runs inside epoch N and memoizes the PRE-write fingerprint under it, so the
        // next read would answer from the cache of a tree that no longer exists until the watcher caught
        // up 1.5s later. The human accepts a Decision and the count does not move - which is precisely
        // the "I click accept and nothing changes" report that started this work.
        crate::fingerprint::new_epoch();
    }
    // With KEEL_PERF set, each request reports the cost of ITS OWN work. `perf::report` prints at process
    // exit, which never comes for a server, so before this the interactive surface was the one thing the
    // instrumentation could not see.
    let cost = crate::perf::interval().map_or_else(String::new, |s| format!("  [{s}]"));
    let line = format!(
        "[keel serve] {method} {path} -> {} ({}ms){cost}",
        resp.status().as_u16(),
        start.elapsed().as_millis()
    );
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
    crate::gitx::git()
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
    crate::gitx::git()
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

/// GET /deck — the obligation review deck (issue192): the same page the artifact publishes, served
/// locally where its save path is this server's own POST endpoints — the path the httpx test exercises.
async fn deck_page(State(s): State<AppState>) -> Response {
    match crate::deck::html(&s.rootpath()) {
        Ok(h) => (
            [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8"),
             (axum::http::header::CACHE_CONTROL, "no-store, must-revalidate")],
            h,
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(serde::Deserialize)]
struct DeckSittingReq {
    story: String,
    verdict: String,
    #[serde(default)]
    note: String,
    by: String,
    judged_at: String,
}

/// POST /api/deck/sitting — a sitting review recorded as a HUMAN critique on the sprint story.
///
/// `by` is REQUIRED and must name a registered Person: the per-sitting review is the one human gate
/// (D0049), and recording it as an AI would fabricate the exact attestation section 4 forbids. The
/// deck embeds the sole registered Person at generation time; a project with no Person gets a refusal.
async fn api_deck_sitting(
    State(s): State<AppState>,
    axum::Json(b): axum::Json<DeckSittingReq>,
) -> Response {
    let root = s.rootpath();
    // One Person registry, one reader: the same actor::person_names the D0178 carve-out and the
    // run-answer check use — three surfaces disagreeing on who counts as a Person is its own defect.
    let is_person = crate::actor::person_names(&root).iter().any(|n| n == &b.by);
    if !is_person {
        return (
            StatusCode::BAD_REQUEST,
            format!(
                "{{\"error\":\"`{}` is not a registered Person - a sitting review is the human gate and may not be recorded as an AI\"}}",
                b.by.replace('"', "'")
            ),
        )
            .into_response();
    }
    let severity = match b.verdict.as_str() {
        "accept" => None,
        "maybe" | "reject" => Some("Medium"),
        other => {
            return (StatusCode::BAD_REQUEST, format!("{{\"error\":\"unknown verdict {other}\"}}"))
                .into_response()
        }
    };
    let rationale = if b.note.is_empty() {
        format!("sitting review via deck: {}", b.verdict)
    } else {
        format!("sitting review via deck: {} - {}", b.verdict, b.note)
    };
    let critiques = root.join(".tracking").join("critiques.sysml");
    // The judgment is made against HEAD - the same commit source api_disposition uses - never
    // against a date masquerading as a sha (guard 36 exists precisely for that confusion).
    let sha = git_head(&root);
    let c = crate::write::Critique {
        element: &b.story,
        method: "critique",
        lens: "completeness",
        // CriticKind is a KIND (human/aiModel/tool), never an actor name - validate caught the
        // first version writing the name into the enum field. The reviewer's identity is judged_by.
        critiqued_by: "human",
        severity,
        rationale: &rationale,
        outcome: if b.verdict == "accept" { "pass" } else { "fail" },
        sha: &sha,
        judged_at: &b.judged_at,
        judged_by: &b.by,
    };
    match crate::write::append_critique(&critiques, &c) {
        Ok(name) => {
            // A sitting review COUNTS only through its #Covers edge to the sprint story - that edge is
            // what `sitting-coverage` computes over, and the e2e test proved a critique alone leaves
            // the due count unchanged. The write API authors the edge; a failure here is reported
            // rather than leaving a critique that silently covers nothing.
            if let Err(e) = crate::write::append_marker_edge(&critiques, "Covers", &name, &b.story) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("{{\"error\":\"critique recorded but the Covers edge failed: {}\"}}", e.to_string().replace('"', "'")),
                )
                    .into_response();
            }
            ok_json(format!("{{\"ok\":true,\"name\":\"{name}\"}}"))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'")),
        )
            .into_response(),
    }
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
    let sha = git_head(&s.rootpath());
    // issue197/issue199, closed at the endpoint (D0178/K6): a disposition is a judgment, and the
    // actor is data the gesture carries — never ambient state this layer resolves. The old binding
    // fallback is exactly how the human's 19 deck taps were recorded as the session AI.
    let Some(judged_by) = body.judged_by.as_deref().map(str::trim).filter(|a| !a.is_empty()) else {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"judged_by is required: the actor is data the gesture carries, never ambient state the server resolves (issue199/D0178)\"}".to_string()).into_response();
    };
    let judged_by = judged_by.to_string();
    let critiques = s.rootpath().join(".tracking").join("critiques.sysml");
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
    let author = match crate::actor::resolve(&s.rootpath(), b.author.as_deref()) {
        Ok(a) => a,
        // D0129/issue072: an omitted actor used to default to a named HUMAN, silently forging a
        // human attestation and making confirmation-authenticity (D0106) meaningless. Refuse instead.
        Err(msg) => return (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{}\"}}", msg.replace('"', "'").replace('\n', " "))).into_response(),
    };
    if b.slug.is_empty() || b.title.is_empty() || b.decision.is_empty() {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"slug, title, and decision are required\"}".to_string()).into_response();
    }
    match crate::write::record_decision(&s.rootpath(), &b.slug, &b.title, &b.date, &author, &b.context, &b.decision, &b.rationale, &b.consequences) {
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
    let Some(path) = safe_repo_path(&s.rootpath(), &b.file) else {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"file must be a repo-relative .sysml path\"}".to_string()).into_response();
    };
    // D0178/K6 (issue199's class): accepting a Decision is the human sign-off itself — the signer
    // must arrive IN the gesture. The binding fallback would let an empty body sign as whoever this
    // machine is bound to; `refuse_ai_judgment` below only catches the AI-bound case.
    let Some(judged_by) = b.judged_by.as_deref().map(str::trim).filter(|a| !a.is_empty()) else {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"judged_by is required: the signer is data the gesture carries, never ambient state the server resolves (issue199/D0178)\"}".to_string()).into_response();
    };
    let sha = git_head(&s.rootpath());
    match crate::write::accept_decision(&path, &b.decision, &sha, &b.judged_at, judged_by, &b.note) {
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
    let Some(path) = safe_repo_path(&s.rootpath(), &b.file) else {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"file must be a repo-relative .sysml path\"}".to_string()).into_response();
    };
    if b.rationale.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"a rejection rationale is required\"}".to_string()).into_response();
    }
    // D0178/K6 (issue199's class): same rule as accept — the rejecting human arrives IN the gesture.
    let Some(judged_by) = b.judged_by.as_deref().map(str::trim).filter(|a| !a.is_empty()) else {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"judged_by is required: the signer is data the gesture carries, never ambient state the server resolves (issue199/D0178)\"}".to_string()).into_response();
    };
    let sha = git_head(&s.rootpath());
    match crate::write::reject_decision(&path, &b.decision, &sha, &b.judged_at, judged_by, &b.rationale) {
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
    let Some(path) = safe_repo_path(&s.rootpath(), &b.file) else {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"file must be a repo-relative .sysml path\"}".to_string()).into_response();
    };
    // D0178/K6 (issue199's class): this endpoint's documented purpose is the review queue's human
    // sign-off of a confirmation gate, so the signer must arrive IN the gesture. (AI ceremony gates
    // go through the CLI's append-gate-result, which has its own explicit-actor semantics.)
    let Some(judged_by) = b.judged_by.as_deref().map(str::trim).filter(|a| !a.is_empty()) else {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"judged_by is required: the signer is data the gesture carries, never ambient state the server resolves (issue199/D0178)\"}".to_string()).into_response();
    };
    let judged_by = judged_by.to_string();
    let sha = git_head(&s.rootpath());
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
    if safe_repo_path(&s.rootpath(), &file_rel).is_none() {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"file must be a repo-relative .sysml path\"}".to_string()).into_response();
    }
    match crate::write::author_edge(&s.rootpath(), &file_rel, &kind, &b.from, &b.to) {
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
    let author = match crate::actor::resolve(&s.rootpath(), b.author.as_deref()) {
        Ok(a) => a,
        // D0129/issue072: an omitted actor used to default to a named HUMAN, silently forging a
        // human attestation and making confirmation-authenticity (D0106) meaningless. Refuse instead.
        Err(msg) => return (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{}\"}}", msg.replace('"', "'").replace('\n', " "))).into_response(),
    };
    let strs: Vec<(String, String)> = b.string_attrs.into_iter().map(|a| (a.name, a.value)).collect();
    let enums: Vec<(String, String, String)> = b.enum_attrs.into_iter().map(|a| (a.name, a.enum_type, a.value)).collect();
    let new_item = crate::write::NewItem {
        keyword: item_keyword(ty), type_name: ty, name_hint: b.name.as_deref().unwrap_or(""),
        string_attrs: &strs, enum_attrs: &enums, author: &author, created_at: &b.date,
    };
    match crate::write::create_item(&s.rootpath(), &new_item) {
        Ok((name, path)) => ok_json(format!("{{\"ok\":true,\"name\":\"{name}\",\"type\":\"{ty}\",\"path\":\"{path}\"}}")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// Types whose content is GOVERNED — an edit must go through a superseding Decision, not an in-place
/// overwrite (D0126 `viewerEditItem` / §2.4). Other types are owner-of-record editable (D0108).
const GOVERNED_TYPES: &[&str] = &["Decision", "Need", "SystemRequirement", "SubsystemRequirement"];

#[derive(serde::Deserialize)]
struct SetAttrReq {
    item: String,
    attr: String,
    value: String,
    #[serde(rename = "enumType")]
    enum_type: Option<String>,
    #[serde(rename = "itemType")]
    item_type: Option<String>,
}

/// POST /api/item/attr (viewerEditItem, D0126) — set an attribute on an existing NON-GOVERNED item in
/// place (owner-of-record, D0108). Governed types (`GOVERNED_TYPES`) are refused — they must supersede.
/// String value quoted+sanitized; an `enumType` writes an `EnumType::member` literal. Never auto-commits;
/// run `/api/check` after.
async fn api_set_attr(State(s): State<AppState>, axum::Json(b): axum::Json<SetAttrReq>) -> Response {
    if let Some(t) = b.item_type.as_deref() {
        if GOVERNED_TYPES.contains(&t) {
            return (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{t} is governed — edit via Supersede (a superseding Decision), not in place\"}}")).into_response();
        }
    }
    let ident_ok = |x: &str| !x.is_empty() && x.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if !ident_ok(&b.item) || !ident_ok(&b.attr) {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"item and attr must be bare identifiers\"}".to_string()).into_response();
    }
    let literal = match b.enum_type.as_deref().filter(|t| !t.is_empty()) {
        Some(ty) if ty.chars().all(|c| c.is_ascii_alphanumeric()) && b.value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') => format!("{ty}::{}", b.value),
        _ => format!("\"{}\"", b.value.replace('"', "'").replace(['\n', '\r', '\t'], " ")),
    };
    match crate::write::set_attr(&s.rootpath(), &b.item, &b.attr, &literal) {
        Ok(path) => ok_json(format!("{{\"ok\":true,\"item\":\"{}\",\"attr\":\"{}\",\"path\":\"{path}\"}}", b.item, b.attr)),
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
        let root = s.rootpath();
        let root = root.as_path();
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
    ok_json(format!("{{\"fingerprint\":\"{}\"}}", crate::fingerprint::of(&s.rootpath())))
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
    view_html(crate::view::report_html(&s.rootpath(), &name, false))
}

/// GET /view/diagram (D0094 m2) — the whole-model interactive diagram HTML (render action).
async fn view_diagram(State(s): State<AppState>) -> Response {
    view_html(crate::view::diagram_html(&s.rootpath()))
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
    ok_json(format!("{{\"agentBridge\":{}}}", agent_bridge_enabled(&s.rootpath())))
}

#[derive(serde::Deserialize)]
struct SettingsReq {
    #[serde(rename = "agentBridge")]
    agent_bridge: bool,
}

/// POST /api/settings — persist serve settings (currently the agent-bridge toggle) to `.keel-serve.json`.
async fn api_settings_post(State(s): State<AppState>, axum::Json(b): axum::Json<SettingsReq>) -> Response {
    let body = format!("{{\"agentBridge\": {}}}\n", b.agent_bridge);
    match std::fs::write(settings_path(&s.rootpath()), body) {
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
    let (action_ok, launch_undefined, prompt) = request_plan(&s.rootpath(), &q);
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

#[allow(clippy::too_many_lines)] // one SSE generator = one run lifecycle; splitting it would smear the yield points
async fn api_agent_stream(State(s): State<AppState>, Query(q): Query<AgentReq>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let root = s.rootpath();
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
        // D0182 run lifecycle for LAUNCHED PROCESS RUNS: dirty-tree refusal + spawn snapshot.
        // The critique action keeps its lighter path (read-mostly, reviewed in-stream).
        let run_setup = if q.action == "launch" {
            match crate::launcher::prepare(&root, &q.target) {
                Ok(s) => Some(s),
                Err(e) => {
                    yield Ok(Event::default().event("error").data(e));
                    return;
                }
            }
        } else {
            None
        };
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
        // Spawn hardening (D0182/P5.5): absolute KEEL_BIN so the run's hooks resolve THIS binary;
        // the run's stamped actor; KEEL_RUN_ID arms the headless-ask mapping in `keel hook pre-write`.
        if let Ok(exe) = std::env::current_exe() {
            command.env("KEEL_BIN", exe);
        }
        if let Some(setup) = &run_setup {
            command.env("KEEL_ACTOR", &setup.actor);
            command.env("KEEL_RUN_ID", &setup.id);
        }
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
        // Per-run WALL-CLOCK timeout (P5.5): --max-turns bounds turns, not stalls. A run that stops
        // producing output for the full budget is killed and recorded timed-out.
        const RUN_WALL_CLOCK_SECS: u64 = 1800;
        let mut turns: u64 = 0;
        let mut timed_out = false;
        if let Some(out) = child.stdout.take() {
            let mut lines = tokio::io::BufReader::new(out).lines();
            loop {
                match tokio::time::timeout(std::time::Duration::from_secs(RUN_WALL_CLOCK_SECS), lines.next_line()).await {
                    Err(_) => {
                        timed_out = true;
                        yield Ok(Event::default().event("error").data(format!("run TIMED OUT after {RUN_WALL_CLOCK_SECS}s of silence - killing the agent tree (P5.5)")));
                        break;
                    }
                    Ok(Ok(Some(line))) => {
                        if line.contains("\"type\":\"assistant\"") {
                            turns += 1;
                        }
                        yield Ok(Event::default().event("agent").data(line));
                    }
                    Ok(Ok(None)) => break,
                    Ok(Err(e)) => {
                        yield Ok(Event::default().event("error").data(format!("read error: {e}")));
                        break;
                    }
                }
            }
        }
        let code = if timed_out {
            drop(killer); // fire the tree killer NOW - the child is wedged
            killer = TreeKiller(None); // disarmed placeholder so the later disarm is a no-op
            None
        } else {
            child.wait().await.ok().and_then(|st| st.code())
        };
        killer.disarm(); // normal exit — the child is already reaped; don't taskkill a dead/recycled PID
        // Post-run gate + records (D0182): validate + guards + rules over the run's diff, the local
        // run record always, ONE tracked summary for non-empty diffs, the diff routed to human
        // review UNCONDITIONALLY.
        if let Some(setup) = &run_setup {
            let outcome = tokio::task::spawn_blocking({
                let root = root.clone();
                let setup2 = crate::launcher::RunSetup {
                    id: setup.id.clone(),
                    process: setup.process.clone(),
                    actor: setup.actor.clone(),
                    head_at_spawn: setup.head_at_spawn.clone(),
                    fingerprint_at_spawn: setup.fingerprint_at_spawn,
                    started: setup.started,
                };
                move || crate::launcher::finish(&root, &setup2, code, turns, timed_out)
            })
            .await;
            match outcome {
                Ok(Ok(o)) => {
                    let verdict = if o.diff_files.is_empty() { "empty diff (local record only)".to_string() }
                        else if o.gate_green { format!("gate GREEN over {} file(s) - diff awaits your review", o.diff_files.len()) }
                        else { format!("gate RED ({} problem(s)) - run recorded FAILED; diff still awaits your review", o.problems.len()) };
                    yield Ok(Event::default().event("status").data(format!("post-run gate: {verdict}")));
                }
                Ok(Err(e)) => yield Ok(Event::default().event("error").data(format!("post-run records failed: {e}"))),
                Err(e) => yield Ok(Event::default().event("error").data(format!("post-run gate task failed: {e}"))),
            }
        }
        yield Ok(Event::default().event("done").data(format!("agent finished (exit {code:?})")));
    };
    Sse::new(stream)
}

#[derive(serde::Deserialize)]
struct LaunchFormReq {
    target: String,
}

/// GET `/api/launch/form?target=X` (D0181/P5.1, THIN by the `DoD`'s own scope-pressure clause): the
/// per-process input form, generated from the MODEL — the declared purpose and steps of the
/// launchable, plus the one free-text field the launcher accepts today. Inputs stay untyped until
/// D0185's typed producedArtifact lands (the recorded trigger); breadth is demand-driven.
async fn api_launch_form(State(s): State<AppState>, Query(q): Query<LaunchFormReq>) -> Response {
    let root = s.rootpath();
    if !crate::view::is_launchable(&root, &q.target).unwrap_or(false) {
        return (StatusCode::NOT_FOUND, format!("{{\"error\":\"`{}` is not a declared launchable - see keel launchables\"}}", q.target.replace('"', "'"))).into_response();
    }
    // purpose + steps from the model text (the process/skill declaration), never hardcoded
    let mut purpose = String::new();
    let mut steps: Vec<String> = Vec::new();
    for f in crate::collect_sysml(&root.join(".engine").join("processes"))
        .into_iter()
        .chain(crate::collect_sysml(&root.join(".engine").join("skills")))
    {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        if !text.contains(&format!("part {} ", q.target)) && !text.contains(&format!("action {} ", q.target)) && !f.file_name().is_some_and(|n| n.to_string_lossy().contains(&q.target)) {
            continue;
        }
        for line in text.lines() {
            let l = line.trim_start();
            if let Some(v) = l.strip_prefix(":>> purpose = \"") {
                if purpose.is_empty() {
                    purpose = v.split('"').next().unwrap_or("").to_string();
                }
            }
            if let Some(v) = l.strip_prefix(":>> title = \"") {
                if l.contains("ProcessStep") || steps.len() < 24 {
                    steps.push(v.split('"').next().unwrap_or("").to_string());
                }
            }
        }
        if !purpose.is_empty() {
            break;
        }
    }
    let steps_json: Vec<String> = steps.iter().take(12).map(|s| format!("\"{}\"", s.replace('"', "'"))).collect();
    ok_json(format!(
        "{{\"target\":\"{}\",\"purpose\":\"{}\",\"declaredSteps\":[{}],\"fields\":[{{\"name\":\"prompt\",\"kind\":\"text\",\"label\":\"task input (typed fields arrive with D0185's typed contracts)\"}}],\"flow\":\"GET /api/agent/plan to review, then /api/agent/stream with approved=1 - approve-before-execute is structural\"}}",
        q.target.replace('"', "'"),
        purpose.replace('"', "'"),
        steps_json.join(",")
    ))
}

#[derive(serde::Deserialize)]
struct RunAskReq {
    path: String,
    session: String,
}

/// POST /api/run/ask (D0182 headless-ask proxy): a launched run's pre-write hook registers an ask
/// and receives an id to poll. The human answers from the console; hook-side expiry maps to deny.
async fn api_run_ask(State(s): State<AppState>, axum::Json(b): axum::Json<RunAskReq>) -> Response {
    let id = crate::write::gen_uuid();
    if let Ok(mut asks) = s.asks.lock() {
        asks.insert(id.clone(), (b.path, b.session, None));
    }
    ok_json(format!("{{\"id\":\"{id}\"}}"))
}

/// GET /api/run/asks — the human-facing queue (console renders it; a pending ask is an obligation).
async fn api_run_asks(State(s): State<AppState>) -> Response {
    let rows: Vec<String> = s.asks.lock().map_or_else(
        |_| Vec::new(),
        |asks| {
            asks.iter()
                .map(|(id, (path, session, answer))| {
                    format!(
                        "{{\"id\":\"{id}\",\"path\":\"{}\",\"session\":\"{}\",\"state\":\"{}\",\"by\":\"{}\"}}",
                        path.replace('"', "'"),
                        session.replace('"', "'"),
                        match answer {
                            None => "pending",
                            Some((true, _)) => "allowed",
                            Some((false, _)) => "denied",
                        },
                        answer.as_ref().map_or("", |(_, by)| by).replace('"', "'")
                    )
                })
                .collect()
        },
    );
    ok_json(format!("{{\"asks\":[{}]}}", rows.join(",")))
}

#[derive(serde::Deserialize)]
struct RunAnswerPoll {
    id: String,
}

/// GET /api/run/answer?id=X — the hook's poll: pending | allow | deny, plus WHO answered (`by`), so
/// the hook can record the authorization against the human who gave it (issue200/K7).
async fn api_run_answer_poll(State(s): State<AppState>, Query(q): Query<RunAnswerPoll>) -> Response {
    let state = s
        .asks
        .lock()
        .ok()
        .and_then(|asks| asks.get(&q.id).map(|(_, _, a)| a.clone()));
    let (word, by) = match state {
        None => ("unknown", String::new()),
        Some(None) => ("pending", String::new()),
        Some(Some((true, by))) => ("allow", by),
        Some(Some((false, by))) => ("deny", by),
    };
    ok_json(format!("{{\"answer\":\"{word}\",\"by\":\"{}\"}}", by.replace('"', "'")))
}

#[derive(serde::Deserialize)]
struct RunAnswerReq {
    id: String,
    allow: bool,
    by: String,
}

/// POST /api/run/answer — the HUMAN's click on the console approve queue (the human channel).
/// `by` is required and must be a registered Person (issue200, D0178/K6): allowing a launched run's
/// protected write is a human judgment, and an AI must not be able to answer its own ask.
async fn api_run_answer(State(s): State<AppState>, axum::Json(b): axum::Json<RunAnswerReq>) -> Response {
    let by = b.by.trim().to_string();
    if !crate::actor::person_names(&s.rootpath()).contains(&by) {
        return (
            StatusCode::BAD_REQUEST,
            format!(
                "{{\"error\":\"`{}` is not a registered Person - answering a run's ask is a human judgment and may not be recorded as an AI (issue200/D0178)\"}}",
                by.replace('"', "'")
            ),
        )
            .into_response();
    }
    let known = s.asks.lock().is_ok_and(|mut asks| {
        asks.get_mut(&b.id).map(|slot| slot.2 = Some((b.allow, by))).is_some()
    });
    if known {
        ok_json(format!("{{\"ok\":true,\"id\":\"{}\"}}", b.id))
    } else {
        (StatusCode::NOT_FOUND, "{\"error\":\"unknown ask id\"}".to_string()).into_response()
    }
}

#[derive(serde::Deserialize)]
struct CommitReq {
    message: String,
}

/// POST /api/commit (D0182 charter note 3, the dirty-tree friction valve): console accepts land
/// uncommitted by design, so accept-then-launch would hit the dirty-tree refusal — the HUMAN
/// clicking commit here is correct attribution for integrating their own accepts. Runs the normal
/// commit path, so the pre-commit gate applies unbypassed.
async fn api_commit(State(s): State<AppState>, axum::Json(b): axum::Json<CommitReq>) -> Response {
    if b.message.trim().len() < 8 {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"a commit message of at least 8 characters is required\"}".to_string()).into_response();
    }
    let root = s.rootpath();
    let add = crate::gitx::git().arg("-C").arg(&root).args(["add", "-A"]).output();
    if !add.as_ref().is_ok_and(|o| o.status.success()) {
        return (StatusCode::INTERNAL_SERVER_ERROR, "{\"error\":\"git add failed\"}".to_string()).into_response();
    }
    let msg_file = root.join(".keel").join("console-commit-msg.txt");
    let _ = std::fs::create_dir_all(root.join(".keel"));
    if crate::write::write_atomic(&msg_file, b.message.as_str()).is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "{\"error\":\"cannot stage the commit message\"}".to_string()).into_response();
    }
    let out = crate::gitx::git().arg("-C").arg(&root).args(["commit", "-F"]).arg(&msg_file).output();
    match out {
        Ok(o) if o.status.success() => ok_json("{\"ok\":true,\"committed\":true}".to_string()),
        Ok(o) => {
            let text = format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("{{\"error\":\"commit gate refused\",\"detail\":\"{}\"}}", text.replace('"', "'").replace(['\n', '\r'], " ").chars().take(1500).collect::<String>()),
            )
                .into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{e}\"}}")).into_response(),
    }
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
    let backlog = s.rootpath().join(".tracking").join("backlog.sysml");
    let issues = s.rootpath().join(".tracking").join("issues.sysml");
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


/// Serve a view's JSON from the per-fingerprint cache, recomputing ONLY when the model changed
/// (D0094 serveLiveCache) — this is what kills the per-request 2s recompute on unchanged data.
/// Stamp an explicit view status into a computed JSON body (srConsoleViewStatusExplicit / N-C2).
///
/// EVERY cached response passes through here, and that is the point: the requirement says no render
/// path may emit content without a status, and a choke point is the only way to mean it. Without it,
/// an empty list and a failed computation are the same absence of rows, and the surface cannot help
/// misleading its reader — the exact failure this engine's own tooling produced repeatedly.
fn with_status(body: &str, status: &str, extra: &str) -> String {
    let b = body.trim_start();
    b.strip_prefix('{').map_or_else(
        // A non-object body (an array) is WRAPPED rather than left unstamped.
        || format!("{{\"viewStatus\":\"{status}\"{extra},\"data\":{b}}}"),
        |rest| {
            // An EMPTY object leaves nothing after the stamp, so the separating comma would be
            // trailing and the whole body invalid JSON. Found on the FAILED path — the one N-C2 leans
            // on hardest: a refusal that cannot be parsed is a refusal whose reason never reaches the
            // reader, which is worse than the failure it was reporting.
            if rest.trim() == "}" {
                format!("{{\"viewStatus\":\"{status}\"{extra}}}")
            } else {
                format!("{{\"viewStatus\":\"{status}\"{extra},{rest}")
            }
        },
    )
}

/// The views on the ARRIVAL path and the one-click destinations behind it (dcServeWarmCache).
///
/// Read off the console's own source rather than guessed: `buildNav` fetches `surfaces`, `obligationBar`
/// fetches `obligations`, `render` fetches `orient`, and the obligation card's `dischargePanel` sends the
/// human to `authority-queue` or `dispositions`. `review-queue` is the review panel's own list. Nothing
/// else is warmed - warming a view nobody opened spends the human's CPU to no purpose.
const HOT_VIEWS: [(&str, ViewFn); 6] = [
    ("obligations", obligations_json),
    ("surfaces", crate::view::surfaces_json),
    ("orient", |r| Ok(crate::orient::compute(r).to_json())),
    ("computed:authority-queue", crate::view::authority_queue),
    ("computed:dispositions", crate::view::dispositions),
    ("review-queue", crate::view::review_queue_json),
];

/// Recompute `key` in the background and store it under fingerprint `fp`.
///
/// One recompute per key at a time: a page load fires eight requests, and without the in-flight set a
/// changed fingerprint would start eight copies of the same expensive view. Dropped silently if one is
/// already running - the running one will store a result for this fingerprint or a later one, and either
/// way the next request is warm.
fn warm(state: &AppState, key: &str, compute: ViewFn, fp: u64) {
    {
        let mut inflight =
            state.refreshing.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !inflight.insert(key.to_string()) {
            return;
        }
    }
    let root = state.rootpath();
    let cache = Arc::clone(&state.cache);
    let refreshing = Arc::clone(&state.refreshing);
    let owned = key.to_string();
    tokio::task::spawn_blocking(move || {
        let result = compute(&root);
        if let Ok(json) = result {
            let mut guard = cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            guard.insert(owned.clone(), (fp, json));
        }
        // Clear the flag even on failure, or one error would wedge this view as un-refreshable for the
        // life of the process - a silent permanent staleness, which is worse than a slow request.
        let mut inflight = refreshing.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        inflight.remove(&owned);
    });
}

type ViewFn = fn(&Path) -> Result<String, crate::view::ViewError>;

/// `cached` for a view that CAPTURES something (a per-item detail keyed by name).
///
/// Blocks on recompute rather than serving stale, deliberately: these are per-item views, so the reader
/// asked for THIS item and a stale answer about one item is more confusing than a short wait. There is
/// also nothing sensible to warm - a reader opens one item, not all 8700.
fn cached_owned(
    state: &AppState,
    key: &str,
    compute: impl FnOnce(&Path) -> Result<String, crate::view::ViewError>,
) -> Response {
    let fp = crate::fingerprint::of(&state.rootpath());
    let previous = {
        let guard = state.cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        guard.get(key).cloned()
    };
    if let Some((cfp, json)) = previous.clone() {
        if cfp == fp {
            return ok_json(with_status(&json, "computed", ""));
        }
    }
    match tokio::task::block_in_place(|| compute(&state.rootpath())) {
        Ok(json) => {
            {
                let mut guard = state.cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                guard.insert(key.to_string(), (fp, json.clone()));
            }
            ok_json(with_status(&json, "computed", ""))
        }
        Err(e) => {
            let reason = e.to_string().replace('"', "'");
            if let Some((_, json)) = previous {
                return ok_json(with_status(&json, "stale", &format!(",\"staleReason\":\"{reason}\"")));
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{{\"viewStatus\":\"failed\",\"reason\":\"{reason}\"}}"),
            )
                .into_response()
        }
    }
}

fn cached(state: &AppState, key: &str, compute: ViewFn) -> Response {
    let fp = crate::fingerprint::of(&state.rootpath());
    let previous = {
        let guard = state.cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        guard.get(key).cloned()
    };
    if let Some((cfp, json)) = previous.clone() {
        if cfp == fp {
            return ok_json(with_status(&json, "computed", ""));
        }
        // STALE-WHILE-REVALIDATE (dcServeWarmCache). The fingerprint moved, so this value is out of date -
        // but it was TRUE at a commit, and returning it labelled `stale` in microseconds beats blocking an
        // interactive surface for seconds on a recompute the human did not ask for. Honest only because the
        // label travels: serving it as `computed` is what N-C2 forbids, and the console renders the banner.
        // The SSE change-push re-renders the page when the refresh lands, so the fresh value arrives
        // without the reader doing anything.
        warm(state, key, compute, fp);
        return ok_json(with_status(
            &json,
            "stale",
            ",\"staleReason\":\"the model changed and this view is recomputing; the page refreshes itself when it lands\"",
        ));
    }
    // issue063: the compute can shell out to git (orient suspect/drift) for up to a second-plus on a cold
    // hit; run it via block_in_place so it never STARVES the multi-thread runtime's worker — other
    // requests + the SSE change-push keep flowing while this view computes. (No async refactor needed:
    // block_in_place offloads the current worker's other tasks; the serve runtime is new_multi_thread.)
    match tokio::task::block_in_place(|| compute(&state.rootpath())) {
        Ok(json) => {
            {
                let mut guard = state.cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                guard.insert(key.to_string(), (fp, json.clone()));
            }
            ok_json(with_status(&json, "computed", ""))
        }
        Err(e) => {
            let reason = e.to_string().replace('"', "'");
            // STALE rather than FAILED when a previous good answer exists. Showing the last true
            // value LABELLED as un-recomputable beats an error page, and stays honest only because
            // the label travels with it; serving it as current is precisely what N-C2 forbids.
            if let Some((_, json)) = previous {
                return ok_json(with_status(&json, "stale", &format!(",\"staleReason\":\"{reason}\"")));
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                with_status("{}", "failed", &format!(",\"reason\":\"{reason}\"")),
            )
                .into_response()
        }
    }
}

/// GET /api/version (viewerKeelApi / D0114 shape B) — the committed API version + the stable read-endpoint
/// contract a viewer app pins to. Static (no model read); the one endpoint a client hits first.
async fn api_version() -> Response {
    let eps = KEEL_API_READ_ENDPOINTS.iter().map(|e| Json::s((*e).to_string())).collect();
    let weps = KEEL_API_WRITE_ENDPOINTS.iter().map(|e| Json::s((*e).to_string())).collect();
    ok_json(Json::Obj(vec![
        ("apiVersion".to_string(), Json::s(KEEL_API_VERSION.to_string())),
        // The CONSOLE build, distinct from the API version (issue153): the API contract can hold steady
        // across many page changes, so a page cannot tell whether it is current by reading apiVersion.
        ("consoleBuild".to_string(), Json::s(console_build().to_string())),
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
/// GET /api/projects — the projects this surface can reach, and which is ACTIVE (N-C4).
///
/// Discovered rather than configured: the active root plus any sibling directory that is itself a keel
/// project. A config file would be a second place to keep the list true, and the filesystem already
/// knows. The active project is always named, which is the half of N-C4 that stops a supervisor
/// wondering which project they are looking at.
async fn api_projects(State(s): State<AppState>) -> Response {
    let active = s.rootpath();
    let mut found: Vec<PathBuf> = vec![active.clone()];
    if let Some(parent) = active.parent() {
        if let Ok(rd) = std::fs::read_dir(parent) {
            for e in rd.flatten() {
                let p = e.path();
                if p != active && p.join(".engine").is_dir() && p.join(".tracking").is_dir() {
                    found.push(p);
                }
            }
        }
    }
    found.sort();
    found.dedup();
    let rows: Vec<crate::json::Json> = found
        .iter()
        .map(|p| {
            crate::json::Json::Obj(vec![
                ("root".to_string(), crate::json::Json::s(p.display().to_string())),
                (
                    "name".to_string(),
                    crate::json::Json::s(
                        p.file_name().map_or_else(String::new, |n| n.to_string_lossy().to_string()),
                    ),
                ),
                ("active".to_string(), crate::json::Json::Bool(*p == active)),
            ])
        })
        .collect();
    ok_json(with_status(
        &crate::json::Json::Obj(vec![
            ("activeProject".to_string(), crate::json::Json::s(active.display().to_string())),
            ("projects".to_string(), crate::json::Json::Arr(rows)),
        ])
        .dump(),
        "computed",
        "",
    ))
}

/// POST /api/project — rebind the surface to another project, or REFUSE and change nothing.
async fn api_project_switch(State(s): State<AppState>, body: String) -> Response {
    let target = serde_json::from_str::<serde_json::Value>(&body)
        .ok()
        .and_then(|v| v.get("root").and_then(|r| r.as_str().map(str::to_owned)))
        .unwrap_or_default();
    if target.is_empty() {
        return (StatusCode::BAD_REQUEST, with_status("{}", "failed", ",\"reason\":\"no root given\"")).into_response();
    }
    match s.rebind(Path::new(&target)) {
        Ok(p) => ok_json(with_status(
            &format!("{{\"activeProject\":\"{}\"}}", p.display().to_string().replace('\\', "/")),
            "computed",
            "",
        )),
        Err(e) => (StatusCode::BAD_REQUEST, with_status("{}", "failed", &format!(",\"reason\":\"{}\"", e.replace('"', "'").replace('\\', "/")))).into_response(),
    }
}

/// GET /api/scope — which MODEL is in view, and every item's resolved scope (N-C3).
///
/// The repo-level question comes first and is the one that actually confuses a reader: where the engine
/// builds ITSELF, its own tracked work and its deliverable's work are the same programme, and the
/// surface must say so rather than leaving the reader to infer it. Per item the scope follows the file
/// the item was authored in — `.engine/` is the engine's own definitions, `.tracking/` is the work
/// being tracked — and anything else is UNSCOPED, never defaulted.
async fn api_scope(State(s): State<AppState>) -> Response {
    cached(&s, "scope", |root| {
        let coincident = root.join("keel-cli").join("Cargo.toml").is_file();
        let (mut engine, mut deliverable, mut unscoped) = (0i64, 0i64, 0i64);
        let mut unscoped_names: Vec<crate::json::Json> = Vec::new();
        for (name, file) in crate::view::item_files(root)? {
            let f = file.replace('\\', "/");
            if f.starts_with(".engine/") {
                engine += 1;
            } else if f.starts_with(".tracking/") {
                deliverable += 1;
            } else {
                unscoped += 1;
                if unscoped_names.len() < 25 {
                    unscoped_names.push(crate::json::Json::s(format!("{name} ({f})")));
                }
            }
        }
        Ok(crate::json::Json::Obj(vec![
            (
                "scope_note".to_string(),
                crate::json::Json::s(
                    "which MODEL is in view. `coincident` means this repository builds the engine                      itself, so the engine's tracked work and the deliverable's are the same programme                      — stated rather than left to be inferred. Per-item scope follows the authoring                      file; anything outside .engine/ or .tracking/ is UNSCOPED and never defaulted.",
                ),
            ),
            (
                "modelsCoincide".to_string(),
                crate::json::Json::Bool(coincident),
            ),
            (
                "activeScope".to_string(),
                crate::json::Json::s(if coincident { "coincident" } else { "distinct" }),
            ),
            ("engineItems".to_string(), crate::json::Json::Int(engine)),
            ("deliverableItems".to_string(), crate::json::Json::Int(deliverable)),
            ("unscopedItems".to_string(), crate::json::Json::Int(unscoped)),
            ("unscopedSample".to_string(), crate::json::Json::Arr(unscoped_names)),
        ])
        .dump())
    })
}

/// GET /api/surfaces — navigation computed from the declared Viewpoint registry, never enumerated.
async fn api_surfaces(State(s): State<AppState>) -> Response {
    cached(&s, "surfaces", crate::view::surfaces_json)
}

/// The full computed JSON for a declared viewpoint's `renderer` command — ONE dispatch table, shared by
/// the obligation counter and by `/api/computed/:cmd`.
///
/// Why it exists (srConsoleObligationActionable): an obligation card must take the reader to the place the
/// work is done, and two of the four act viewpoints named views with no endpoint at all — a click would
/// have silently changed nothing. Serving the command's own JSON generically means a NEWLY DECLARED act
/// viewpoint is both countable and reachable with no route and no console change.
///
/// `None` for a command with no binding, reported as NOT AVAILABLE naming the command — never as an empty
/// result, which is the same rule the counter follows.
type ComputedFn = fn(&Path) -> Result<String, crate::view::ViewError>;

/// The command -> view binding, resolved WITHOUT computing anything (issue146).
///
/// Returning the function rather than its result is the whole point. The first version of this table
/// returned `Option<Result<String, _>>`, which conflated two questions - IS this command bound, and
/// WHAT does it compute - so the only way to ask the first was to answer the second. `api_computed`
/// asked whether the command was bound, threw the computed view away, then computed it again through
/// the cache: EVERY request paid a full uncached computation, cache hit or not. A binding is a fact
/// about the table and must be answerable without touching the model.
fn computed_binding(cmd: &str) -> Option<ComputedFn> {
    Some(match cmd {
        "orient" => |root: &Path| Ok(crate::orient::compute(root).to_json()),
        "dispositions" => crate::view::dispositions,
        "decision-follow-through" => crate::view::decision_follow_through,
        "enforcement-report" => crate::pm::enforcement_report,
        "authority-queue" => crate::view::authority_queue,
        "sitting-coverage" => crate::view::sitting_coverage,
        "open-issues" => crate::view::open_issues,
        "intake" => crate::view::intake,
        "suspect" => |root: &Path| Ok(crate::govern::suspect(root, false)),
        "critique-coverage" => crate::view::critique_coverage,
        "concern-coverage" => crate::view::concern_coverage,
        "coverage" => crate::view::coverage,
        "rootedness" => crate::view::rootedness,
        "tier-satisfaction" => crate::view::tier_satisfaction,
        "indicators" => |root: &Path| crate::view::indicators(root, false),
        "orphans" => |root: &Path| {
            crate::algo::orphans(root)
                .map_err(|e| crate::view::ViewError::Track(String::from("orphans"), e.to_string()))
        },
        _ => return None,
    })
}

/// Compute the view bound to `cmd`, or `None` when nothing is bound.
fn computed_view(root: &Path, cmd: &str) -> Option<Result<String, crate::view::ViewError>> {
    computed_binding(cmd).map(|f| f(root))
}

/// GET /api/computed/:cmd — any declared viewpoint's computed JSON, so a card can land on the view its
/// renderer names. Cached per command like every other view.
async fn api_computed(State(s): State<AppState>, axum::extract::Path(cmd): axum::extract::Path<String>) -> Response {
    // The VIEW's name is the key. It used to be prefixed "computed:", which cached `dispositions`
    // separately from the identical JSON served by /api/dispositions.
    let key = cmd.clone();
    // Ask the TABLE whether the command is bound; never compute a view to find out (issue146).
    computed_binding(&cmd).map_or_else(
        // Not an empty body and not a 200: a command with no binding is a stated gap, so the reader learns
        // the view exists in the model and has no server-side computation rather than seeing a blank panel.
        || {
            (
                StatusCode::NOT_FOUND,
                format!("{{\"viewStatus\":\"failed\",\"reason\":\"no computed view is bound to the command '{cmd}'\"}}"),
            )
                .into_response()
        },
        |f| cached(&s, &key, f),
    )
}

/// The obligation count for one declared act-surface viewpoint, read from the view its `renderer`
/// names — so the AUTHORITY stays the existing computed view and nothing is re-implemented here.
///
/// Returns `None` when no counter is bound to that command. That is reported as NOT COMPUTABLE, never
/// as zero: a class the console cannot count must not look like a class with nothing in it (N-C2).
/// The console panel that can DISCHARGE this obligation class, or `None` when none can (issue155).
///
/// Two clicks were needed to reach the work: one to open the class, one to cross a bridge to a panel that
/// had controls. The bridge existed because the console decided AFTER landing whether the destination
/// could act. The server already computes each class's view to produce its count, so it can say where the
/// work is done and the card can go straight there.
///
/// STATED PER COMMAND WITH A REASON, not inferred from the payload's shape. A regex over item `kind`
/// strings would be a guess, and a wrong guess sends the human to a panel that cannot act - the exact
/// defect being fixed. `None` is a first-class answer and degrades gracefully: a newly declared act
/// viewpoint still needs no console change, it lands on the read-only rendering with the note saying so,
/// which is the honest destination for work the console genuinely cannot discharge.
const fn discharge_panel(cmd: &str) -> Option<&'static str> {
    match cmd.as_bytes() {
        // Proposed Decisions and pending confirmation gates: the review panel holds accept and reject for
        // both, and `pendingAcceptances` and `awaiting` are largely the same items counted twice.
        b"orient" | b"authority-queue" => Some("review"),
        // The dispositions panel carries the per-finding disposition controls.
        b"dispositions" => Some("dispositions"),
        // A sitting review is a human reading a sitting and recording a judgment. NO console control does
        // that today, so this is None rather than a panel that would only display the work again.
        _ => None,
    }
}

fn obligation_count(root: &Path, cmd: &str) -> Option<(i64, Option<String>)> {
    let num = |json: &str, key: &str| -> Option<i64> {
        serde_json::from_str::<serde_json::Value>(json).ok()?.get(key).and_then(|v| match v {
            serde_json::Value::Array(a) => i64::try_from(a.len()).ok(),
            other => other.as_i64(),
        })
    };
    // The COUNT key per command; the JSON itself comes from computed_view, so the counter and the panel
    // can never disagree about which view a renderer names (one dispatch table, not two).
    // THROUGH THE SHARED STORE, not a private compute: these four views are the same four the console
    // fetches moments later, so computing them here also serves those requests.
    let json = store_or_compute(root, cmd, |r| computed_view(r, cmd).unwrap_or_else(|| Err(crate::view::ViewError::NotFound(cmd.to_string()))))?;
    match cmd {
        "orient" => num(&json, "pendingAcceptances").map(|n| (n, None)),
        "dispositions" => num(&json, "undispositioned").map(|n| (n, None)),
        "authority-queue" => num(&json, "awaiting").map(|n| (n, None)),
        // `due`, not `uncovered`: the live obligation is what postdates D0155's grandfather line. The
        // 313 sittings uncovered when that line landed are accepted-unreviewed by human attestation and
        // stay visible in the view's own `grandfathered_unreviewed` — they are not a thing to act on, so
        // they do not belong on the act surface (N-C1: obligations requiring judgment AND NOTHING ELSE).
        // No caveat: the number is defensible now, and a caveat on a defensible number is noise.
        "sitting-coverage" => num(&json, "due").map(|n| (n, None)),
        _ => None,
    }
}

/// GET /api/obligations — what is waiting on the human, with the CLASSES derived from the declared
/// act-surface viewpoints (srConsoleObligationsOnArrival / N-C1).
///
/// The classes are not enumerated here. Declaring a viewpoint with `surface = "act"` adds one, which
/// is what makes N-C1's "without having been told a new class exists" achievable at all.
/// The obligation set — WHAT IS WAITING ON A HUMAN — as JSON (issue150).
///
/// Extracted from `api_obligations` so it has exactly one home. The turn-boundary hook needs the TOTAL
/// to decide whether the human has anything to act on, and re-deriving "which viewpoints are act-surface
/// and what do they count" in a second place is the dual truth §1 forbids: the two copies would drift and
/// the hook would advise about a different set than the console displays.
///
/// # Errors
/// Returns [`crate::view::ViewError`] if the surfaces view cannot be computed.
pub fn obligations_json(root: &Path) -> Result<String, crate::view::ViewError> {
    // Through the store for the same reason as the counts below: /api/surfaces asks for this exact JSON
    // on the same page load, and it is what tells us which viewpoints are act-surface.
    let surfaces = store_or_compute(root, "surfaces", crate::view::surfaces_json)
        .map_or_else(|| crate::view::surfaces_json(root), Ok)?;
    let parsed: serde_json::Value = serde_json::from_str(&surfaces).unwrap_or(serde_json::Value::Null);
    let act = parsed
        .get("surfaces")
        .and_then(|v| v.as_array())
        .and_then(|a| a.iter().find(|s| s.get("surface").and_then(|x| x.as_str()) == Some("act")))
        .and_then(|s| s.get("viewpoints"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let mut classes: Vec<crate::json::Json> = Vec::new();
    let mut total = 0i64;
    let mut uncountable = 0i64;
    for vp in &act {
        let get = |k: &str| vp.get(k).and_then(|x| x.as_str()).unwrap_or_default().to_string();
        let renderer = get("renderer");
        let cmd = renderer.trim_start_matches("keel ").split([' ', '(']).next().unwrap_or("").to_string();
        let mut row = vec![
            // The viewpoint's ELEMENT NAME, so a console can link the card to the place the work is
            // done by identity rather than by matching a title (srConsoleObligationActionable).
            ("viewpoint".to_string(), crate::json::Json::s(get("viewpoint"))),
            ("title".to_string(), crate::json::Json::s(get("title"))),
            ("concern".to_string(), crate::json::Json::s(get("concern"))),
            ("renderer".to_string(), crate::json::Json::s(renderer.clone())),
        ];
        if let Some(panel) = discharge_panel(&cmd) {
            // WHERE THE WORK IS DONE, so the card lands there in one click instead of via a bridge.
            row.push(("dischargePanel".to_string(), crate::json::Json::s(panel.to_string())));
        }
        if let Some((n, caveat)) = obligation_count(root, &cmd) {
            {
                total += n;
                row.push(("count".to_string(), crate::json::Json::Int(n)));
                row.push(("countable".to_string(), crate::json::Json::Bool(true)));
                if let Some(c) = caveat {
                    row.push(("caveat".to_string(), crate::json::Json::s(c)));
                }
            }
        } else {
            {
                uncountable += 1;
                row.push(("countable".to_string(), crate::json::Json::Bool(false)));
                row.push((
                    "why".to_string(),
                    crate::json::Json::s(format!(
                        "no counter bound to `{cmd}` — reported as NOT COUNTABLE rather than as zero (N-C2)"
                    )),
                ));
            }
        }
        classes.push(crate::json::Json::Obj(row));
    }
    Ok(crate::json::Json::Obj(vec![
        (
            "obligations_note".to_string(),
            crate::json::Json::s(
                "what is waiting on a HUMAN. Classes are derived from viewpoints declaring \
                 surface=\"act\" — declaring one adds a class with no console change. A class with no \
                 bound counter is reported NOT COUNTABLE, never as zero.",
            ),
        ),
        ("total".to_string(), crate::json::Json::Int(total)),
        ("classes".to_string(), crate::json::Json::Arr(classes)),
        ("uncountableClasses".to_string(), crate::json::Json::Int(uncountable)),
    ])
    .dump())
}

/// How many items are waiting on a human, or `None` when that cannot be computed.
///
/// Reads the TOTAL out of [`obligations_json`] rather than counting anything itself, so the number the
/// hook advises on is the number the console shows. `None` is distinct from `Some(0)` and must stay so:
/// "nothing is waiting" and "I could not tell what is waiting" are different answers, and reporting the
/// second as the first is how a quiet failure becomes a false all-clear (N-C2).
#[must_use]
pub fn obligations_total(root: &Path) -> Option<i64> {
    let json = obligations_json(root).ok()?;
    let v: serde_json::Value = serde_json::from_str(&json).ok()?;
    v.get("total").and_then(serde_json::Value::as_i64)
}

async fn api_obligations(State(s): State<AppState>) -> Response {
    cached(&s, "obligations", obligations_json)
}

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

/// GET /api/persons (issue200/issue201) — the registered `Person` set. Pages offer these as the
/// judgedBy choices instead of shipping a hardcoded human name; the write endpoints still verify.
async fn api_persons(State(s): State<AppState>) -> Response {
    let names: Vec<String> = crate::actor::person_names(&s.rootpath())
        .into_iter()
        .map(|n| format!("\"{}\"", n.replace('"', "'")))
        .collect();
    ok_json(format!("{{\"persons\":[{}]}}", names.join(",")))
}

async fn api_processes(State(s): State<AppState>) -> Response {
    cached(&s, "processes", |r| Ok(processes_json(r)))
}

async fn api_report(State(s): State<AppState>, AxPath(name): AxPath<String>) -> Response {
    cached_owned(&s, &format!("report:{name}"), |r| crate::view::report(r, &name, false))
}

/// History reads ~/.claude transcripts (outside the fingerprint), so it is computed fresh (uncached).
async fn api_history(State(s): State<AppState>) -> Response {
    ok_json(interaction_history(&s.rootpath()))
}

/// GET /api/recent (sr15) — the git-derived recent-activity timeline. Reads git history (outside the
/// model fingerprint), so it is computed fresh (uncached); a git failure yields an empty timeline.
async fn api_recent(State(s): State<AppState>) -> Response {
    match crate::view::recent(&s.rootpath()) {
        Ok(json) => ok_json(json),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("recent error: {e}")).into_response(),
    }
}

/// GET /api/item/:name (D0094 serveItemIntrospect) — any item's detail (attrs + edges + neighbors).
async fn api_item(State(s): State<AppState>, AxPath(name): AxPath<String>) -> Response {
    cached_owned(&s, &format!("item:{name}"), |r| crate::view::item_detail(r, &name))
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
    match crate::view::section_json(&s.rootpath(), q.view.as_deref(), q.element.as_deref()) {
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
    match crate::view::slice_json(&s.rootpath(), &q.seed, depth, &edges, dir, df) {
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

/// GET /api/grammar (srViewerRelationshipGrammar, D0126/D0127) — the computed relationship grammar:
/// the observed (sourceType, edge, targetType) triples + per-type up/down summary. Drives scope-aware
/// creation + valid-edge offers generically. Cached per fingerprint.
async fn api_grammar(State(s): State<AppState>) -> Response {
    cached(&s, "grammar", crate::view::grammar_json)
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
    cached_owned(&s, &format!("relations:{kind}:{focus}"), move |r| crate::view::relations_json(r, &focus, kind))
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
    match crate::view::change_impact_json(&s.rootpath(), &q.seed, &edges, dir) {
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
    let commit = git_head(&s.rootpath());
    let as_of = git_head_date(&s.rootpath());
    match crate::view::snapshot_json(&s.rootpath(), &q.seed, depth, &edges, dir, &commit, &as_of) {
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
    match crate::view::baseline_compare_json(&s.rootpath(), &q.seed, &q.from, &q.to, depth, &edges, dir) {
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
    match crate::view::critique_plan_json(&s.rootpath(), &q.seed, depth, &edges, dir, lens) {
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
    match crate::view::boundary_json(&s.rootpath(), &q.need) {
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
    let Some(file) = find_task_file(&s.rootpath(), &b.task) else {
        return (StatusCode::NOT_FOUND, format!("{{\"error\":\"no `action {}` found in .tracking\"}}", b.task.replace('"', "'"))).into_response();
    };
    let by = match crate::actor::resolve(&s.rootpath(), b.judged_by.as_deref()) {
        Ok(a) => a,
        // D0129/issue072: an omitted actor used to default to a named HUMAN, silently forging a
        // human attestation and making confirmation-authenticity (D0106) meaningless. Refuse instead.
        Err(msg) => return (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{}\"}}", msg.replace('"', "'").replace('\n', " "))).into_response(),
    };
    let sha = git_head(&s.rootpath());
    match crate::write::append_result(&file, &b.task, &sha, &verdict, &b.judged_at, &by) {
        Ok(name) => ok_json(format!("{{\"ok\":true,\"name\":\"{name}\",\"verdict\":\"{verdict}\"}}")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e.to_string().replace('"', "'"))).into_response(),
    }
}

/// GET /api/events (D0094 serveLiveCache) — SSE change-push: poll the content fingerprint server-side
/// (~1.5s) and emit a `changed` event only when it flips, so the UI refetches event-driven (not blind
/// polling). `ping` keepalives in between; `hello` carries the initial fingerprint.
async fn api_events(State(s): State<AppState>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // SUBSCRIBES to the watcher; does not poll. Each connection used to run its own 1.5s fingerprint
    // loop, so the cost of watching scaled with the number of open tabs while the answer was identical
    // for all of them. The `ping` still goes out on the same cadence so a dead connection is still
    // detectable from the page - the header's `live:` indicator depends on traffic, not on change.
    let mut rx = s.changes.subscribe();
    let stream = async_stream::stream! {
        // The guard is dropped BEFORE the yield: holding a watch borrow across an await makes the whole
        // stream future non-Send and the router will not accept it.
        let hello = { *rx.borrow_and_update() };
        yield Ok(Event::default().event("hello").data(hello.to_string()));
        loop {
            match tokio::time::timeout(Duration::from_millis(1500), rx.changed()).await {
                Ok(Ok(())) => {
                    let now = { *rx.borrow_and_update() };
                    yield Ok(Event::default().event("changed").data(now.to_string()));
                }
                // The sender is gone: the watcher task died, so this stream can no longer speak for the
                // state of the tree. Ending the stream makes the page say `reconnecting` rather than
                // sitting silently on a value nothing is refreshing.
                Ok(Err(_)) => return,
                Err(_) => yield Ok(Event::default().event("ping").data("")),
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
    const SERVE_RS: &str = include_str!("serve.rs");
    use super::{CONSOLE_HTML, build_launch_prompt, claude_in_dirs, is_localhost_origin, KEEL_API_READ_ENDPOINTS, KEEL_API_VERSION, KEEL_API_WRITE_ENDPOINTS};

    /// issue141: the console must bind a card's target BY IDENTITY, never by its position in a rendered
    /// list. `CONSOLE_HTML` is `include_str!`-embedded, so this costs no toolchain and fails at
    /// `cargo test` the moment the anti-pattern returns.
    ///
    /// HONEST CEILING, stated because I previously overclaimed the opposite: this is a TEXT assertion over
    /// the asset. It catches the specific anti-pattern that produced issue141 -- indexing every `.c` card
    /// against an array while a second card kind renders into the same container -- and it does NOT verify
    /// BEHAVIOUR. A wrong destination that is bound by identity would pass this and needs a DOM-level or
    /// browser-level test (D0160).
    #[test]
    fn console_binds_card_targets_by_identity_not_position() {
        // The anti-pattern: selecting every card in the container and using the loop INDEX as the key.
        // The obligation bar renders `.c` cards into that same container, so this was off by four.
        assert!(
            !CONSOLE_HTML.contains("querySelectorAll('.c')"),
            "the console selects every .c card as a group -- if it indexes them against a list, adding              another card kind to the container silently shifts every target (issue141). Bind by a data-*              attribute carried on the element instead."
        );
        // and the identity bindings that replaced it must be present
        for needle in ["data-oblig", "data-vp", "[data-oblig],[data-vp]"] {
            assert!(CONSOLE_HTML.contains(needle), "console is missing the identity binding `{needle}`");
        }
    }
    /// issue143: a console failure must not be able to impersonate a console no-op. Three controls, all
    /// text-level because D0160 records that we cannot execute the page - which is exactly why the page
    /// has to be built so that a failure is loud without a test needing to see it.
    #[test]
    fn a_console_failure_can_never_look_like_nothing_happened() {
        // (1) The page must be served no-store. A cached console runs OLD LOGIC against a NEW API and no
        //     server-side check can see the mismatch, which is what made issue143 undiagnosable.
        assert!(
            SERVE_RS.contains("no-store"),
            "the console must be served no-store: a browser holding an older copy runs older logic              against a newer API and the mismatch is invisible from here"
        );
        // (2) A thrown render must write the failure INTO the panel. Writing only to the status line
        //     leaves the previous view on screen, which is the exact reported symptom.
        assert!(
            CONSOLE_HTML.contains("this view FAILED to render"),
            "a thrown render must say so in the panel -- leaving the previous panel up and whispering              into the status line is how a failure disguises itself as a no-op"
        );
        // (3) A click must announce its target BEFORE its first await, so 'fired and failed' and 'never
        //     fired' are distinguishable observations.
        // Scan CODE lines only: the first draft of this assertion matched the word `await` inside the
        // comment explaining the rule, and failed the very code that follows it.
        let go = CONSOLE_HTML.find("function goToVp").expect("goToVp must exist");
        let body: Vec<&str> = CONSOLE_HTML[go..]
            .lines()
            .take_while(|l| !l.starts_with("main.addEventListener"))
            .filter(|l| !l.trim_start().starts_with("//"))
            .collect();
        let announce = body.iter().position(|l| l.contains("status.textContent="));
        let first_await = body.iter().position(|l| l.contains("await"));
        assert!(
            announce.is_some() && announce < first_await.or(Some(usize::MAX)),
            "goToVp must set the status line before anything that can await and throw"
        );
        // (4) The page must be able to say WHICH build it is, or 'is your browser stale?' stays unanswerable.
        assert!(
            CONSOLE_HTML.contains("v.apiVersion"),
            "the console must stamp the API version it loaded against"
        );
    }

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

#[cfg(test)]
mod view_status_tests {
    use super::with_status;

    /// srConsoleViewStatusExplicit: the four states must be DISTINGUISHABLE in the payload, because
    /// an empty list and a failed computation are otherwise the same absence of rows (N-C2).
    #[test]
    fn the_four_states_are_distinguishable() {
        let computed = with_status(r#"{"ready":[]}"#, "computed", "");
        let failed = with_status("{}", "failed", ",\"reason\":\"parse error\"");
        let stale = with_status(r#"{"ready":["x"]}"#, "stale", ",\"staleReason\":\"parse error\"");
        assert!(computed.contains(r#""viewStatus":"computed""#));
        assert!(failed.contains(r#""viewStatus":"failed""#) && failed.contains("parse error"));
        assert!(stale.contains(r#""viewStatus":"stale""#) && stale.contains("staleReason"));
        // The load-bearing assertion: a COMPUTED-EMPTY body must not look like a FAILED one.
        assert_ne!(
            computed.replace("computed", "X"),
            failed.replace("failed", "X"),
            "an empty computed view and a failed view must not be interchangeable"
        );
    }

    #[test]
    fn an_empty_result_keeps_its_content_and_is_still_stamped() {
        let s = with_status(r#"{"items":[]}"#, "computed", "");
        assert!(s.contains(r#""items":[]"#), "the payload survives the stamp");
        assert!(s.starts_with(r#"{"viewStatus":"computed","#));
    }

    #[test]
    fn an_empty_object_body_stays_valid_json() {
        // Regression: the FAILED path stamps an empty {} body, and the separating comma made it
        // `{"viewStatus":"failed","reason":"...",}` — invalid, so the reason never reached the reader.
        let s = with_status("{}", "failed", ",\"reason\":\"nope\"");
        assert!(!s.contains(",}"), "trailing comma makes the refusal unparseable: {s}");
        assert_eq!(s, r#"{"viewStatus":"failed","reason":"nope"}"#);
    }

    #[test]
    fn a_non_object_body_is_wrapped_rather_than_left_unstamped() {
        // No render path may emit content without a status — including an array body.
        let s = with_status("[1,2]", "computed", "");
        assert!(s.contains(r#""viewStatus":"computed""#) && s.contains(r#""data":[1,2]"#));
    }
}
