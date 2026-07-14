// =====================================================================
// D0114 — keel-viewer architecture = SHAPE B: a SEPARATE viewer app over a committed, versioned keel
// read/query/graph/temporal API. The Architecture-phase keystone from the business-architecture skill
// run on the keel-viewer handoff; serves the accepted Need nViewerSeparation (N-6). Human-accepted
// 2026-07-14 (chose B). NOT a process/schema change (a product-architecture decision) — no
// #ProspectiveChange; but as an accepted Decision it carries a human-judged acceptance event.
// =====================================================================
package Decision0114 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0114 : Decision {
        :>> id = "10a1b0c1-4114-42a3-ab5c-516273849714";
        :>> title = "D0114: keel-viewer architecture = B (separate viewer over a committed, versioned keel read/query/graph/temporal API)";
        :>> createdAt = "2026-07-14";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The keel-viewer Business phase (Brief + 8 accepted Needs + 6 UseCases, keel-viewer.sysml) established the need for a viewpoint-driven stakeholder explorer over SysML v2 data, with keel remaining the strict management AUTHORITY. Need nViewerSeparation (N-6, accepted) requires the viewing surface to CONSUME keel rather than duplicate management. The handoff (§3) posed three shapes: A extend keel serve; B a separate viewer app over a committed, versioned keel read/query/graph/temporal API; C keel emits declarative view DATA + a thin renderer (which collapses toward B for interactivity). Needs-before-architecture: this Decision is recorded only AFTER the Needs were human-accepted (keelViewerNeedsAcceptR1).";
        :>> decision = "Adopt SHAPE B: keel commits to a STABLE, VERSIONED read/query/graph/temporal API (much of /api/* already exists in keel serve) as a first-class deliverable; the viewer is its own app (web) consuming that API. keel stays the lean engine + the only write path (write API + guards); the viewer never becomes a second store. This makes the versioned keel API the enabling first increment; auth/multi-user/saved-views (N-7/N-8) live in the viewer app, not the engine.";
        :>> rationale = "ALTERNATIVES (handoff §3): (A) extend keel serve — REJECTED as the long-term shape: fastest to something, but pushes UI/deployment/auth concern into the engine keel is deliberately keeping lean. (C) emit view data + thin renderer — REJECTED: maximal separation on paper, but drill-down/interactivity needs a running service, collapsing back toward B. (B CHOSEN by the human): best honors 'separate management from viewing' (N-6) while reusing keel's engine/query/graph/write-back; viewer iterates + deploys independently + adds auth without touching the engine. COST: keel must commit to a versioned API surface as a deliverable (the enabling first increment). CRITERIA: separation-of-concerns (N-6) x reuses-keel-substrate x independent-deploy/auth x keel-stays-lean.";
        :>> consequences = "ACCEPTED by the human 2026-07-14 (discrete d0114Accept TestResult, human-judged — not prose, D0058/D0106): selected 'B — separate viewer over a versioned keel API'. Enables the Architecture phase: SystemRequirements derive from the accepted Needs (keel-viewer-requirements.sysml) with the versioned keel API as the keystone SR; implementation Stories are the keel-SIDE deliverables (the API + N-3 temporal state + N-2 graph-expand + N-1 visual viewpoint), the viewer app itself being a SEPARATE project consuming them. Six open questions remain (handoff §6: temporal date-fact model Q2, query-expansion bound Q3, visual-form authoring Q4, auth scope Q5, audience Q6) — captured as SR-level TBDs / follow-on Decisions, not blockers to starting the API. Serves N-6 (#DerivedFrom nViewerSeparation)."; }

    // The architecture Decision serves the Need it derives from (needs-before-architecture).
    #DerivedFrom dependency from d0114 to nViewerSeparation;

    verification d0114Accept : Test { :>> id = "10a1b0c1-4114-42a3-bb5c-516273849714"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of D0114 (keel-viewer architecture = B, a separate viewer over a committed versioned keel read/query/graph/temporal API) via selecting 'B — separate viewer over a versioned keel API (Recommended)' on 2026-07-14, in direct response to the presented A/B/C architecture options after accepting the Needs."; }
    part d0114AcceptR1 : TestResult { :>> id = "10a1b0c1-4114-42a3-cb5c-516273849714"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5f82eec"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "wweatherholtz"; }
}
