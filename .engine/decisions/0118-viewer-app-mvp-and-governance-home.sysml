// =====================================================================
// D0118 (PROPOSED — NOT YET ACCEPTED) — the keel-viewer APP MVP architecture + the viewer program's
// governance home. (a) The app is a BUILDLESS, generative static SPA over keel's committed API
// (shape B / D0114, generative / D0117). (b) The viewer PROGRAM is governed in THIS (keel) repo,
// where its Needs/Decisions already live; the sysmlv2-viewer repo holds the CODE deliverable only.
// Serves srViewerGenerativeUi + srViewerGraphExpand. Human-directed 2026-07-15 (build the app).
// Acceptance is the human sitting review (never fabricated — D0106).
// =====================================================================
package Decision0118 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0118 : Decision {
        :>> id = "10a1b0c1-4118-42a3-ab5c-516273849718";
        :>> title = "D0118: viewer app MVP is a buildless generative SPA over the keel API; the viewer program is governed in the keel repo";
        :>> createdAt = "2026-07-15";
        :>> createdBy = "claudeOpus";
        :>> status = DecisionStatus::proposed;
        :>> context = "The user chose to BUILD the viewer app (2026-07-15, over shape B / D0114). Two things need recording as governance — because recording is part of the task, not a later step: (a) the MVP app's architecture/stack, and (b) WHERE the viewer program is governed, given the sysmlv2-viewer repo is itself keel-init'd with its own .tracking, while ALL viewer Needs (N-1..N-17) and Decisions (D0114-D0117) already live in the keel repo.";
        :>> decision = "(1) ARCHITECTURE: the viewer app is a BUILDLESS static SPA (index.html + app.js + vendored cytoscape; no CDN, no toolchain) that consumes keel's committed API — discovering the API from /api/version, the type+attribute schema from /api/schema, seed-anchored subgraphs from /api/slice, element detail from /api/item/:name, and proposing Decisions via POST /api/decision. Rendering is GENERATIVE (D0117): the type browser, node colour palette, detail fields, and the write form are all built from the DECLARED model at runtime — nothing per-type is hardcoded. Committed to the sysmlv2-viewer repo (commit 5f55952). (2) GOVERNANCE HOME: the viewer PROGRAM is governed in THIS (keel) repo, where its Needs/Decisions/delivery sprints already live; the sysmlv2-viewer repo holds the CODE deliverable and is NOT a second tracker.";
        :>> rationale = "ARCHITECTURE alternatives: (i) a framework SPA (React/Vite with a build step) — REJECTED for the MVP: adds a toolchain that contradicts keel's offline/self-contained ethos (keel's own console is buildless); a framework can come later if the app outgrows vanilla JS. (ii) grow keel serve to host the UI (shape A) — already REJECTED by D0114 (separate app). CHOSEN buildless generative SPA: fastest path to a working, browser-verifiable artifact, reuses keel's vendored cytoscape, and embodies the generative principle end-to-end. GOVERNANCE alternatives: (i) govern the viewer in the sysmlv2-viewer repo's own .tracking — REJECTED for now: all viewer Needs/Decisions already live in keel, so splitting would FORK truth across two repos (the dual-truth trap the discipline explicitly warns against) and duplicate the needs. CHOSEN single governance home in keel (dogfooding legitimately tracks the ecosystem); revisit and MIGRATE via a tracked Decision + the migration process if the viewer later needs independent governance/release.";
        :>> consequences = "Recorded PROPOSED; acceptance is the human sitting review (D0049). The viewer MVP build is recorded as delivery sprint 223 in this repo, satisfying srViewerGenerativeUi + srViewerGraphExpand. The sysmlv2-viewer repo's .tracking stays a minimal skeleton (not a parallel tracker) until/unless a migration Decision moves governance there. NOTE (host specific): keel is NOT on PATH on this host though the viewer repo's CLAUDE.md assumes it — use the built binary path; installing keel on PATH is a follow-up if viewer-repo commands become routine."; }

    #DerivedFrom dependency from d0118 to nViewerGenerativeUi;
    #DependsOn dependency from d0118 to d0114;
}
