// =====================================================================
// ProjectBusinessKeelViewer — Business-workflow artifacts for a viewpoint-driven SysML v2 explorer
// (the "keel-viewer" handoff, W. Weatherholtz 2026-07-14: KEEL-VIEWER-HANDOFF.md). Deployed via the
// business-architecture skill — a GENUINE design/scoping request (the second real dogfood, advancing
// issue061). Phase 1: Brief + Personas + Needs + UseCases. NEEDS-BEFORE-ARCHITECTURE: the human accepts
// these Needs (the Business gate) before any SystemRequirement / architecture Decision (A/B/C) is
// recorded. The gate + architecture are NOT authored here — they await explicit human sign-off.
// =====================================================================
package ProjectBusinessKeelViewer {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part keelViewerBrief : Brief {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-7c5d1f0e2b13";
        :>> title = "keel-viewer — a viewpoint-driven explorer for SysML v2 systems (brief)";
        :>> createdAt = "2026-07-14";
        :>> createdBy = "wweatherholtz";
        :>> problem = "There is no good way to EXPLORE an arbitrary SysML v2-backed system BY VIEWPOINT for stakeholders — 'show me safety processes', 'show me the safety data around THIS process', 'show me requirement traceability for X', 'show me business processes and their state (last annual review, by whom, is one due)' — filtering out everything unrelated. The OSS landscape (SysON, the Eclipse pilot) is diagram-AUTHORING for modelers, not query-a-viewpoint EXPLORATION for stakeholders. keel already has most of the substrate (a declared-view engine, an interactive graph, a live console with disciplined write-back) but lacks the stakeholder-facing, viewpoint-driven, temporally-aware surface on top.";
        :>> opportunity = "keel already provides: a declared-view engine (.view.toml ViewSpec + Viewpoint registry: select/traverse/project), an interactive offline cytoscape graph (keel diagram / render --mode graph), table + review render modes with write-back (apply-review), a live axum console (keel serve, SSE change-push, disciplined writes), and seed-anchored slices (trace/trace-need/need_slice/element_neighbourhood). The opportunity is to extend that substrate into a stakeholder explorer: declarable VISUAL viewpoints (per audience), an EXPLORABLE (seed+expand, drill-down, stable-layout, scalable) graph, CALENDAR-based temporal/freshness state, cross-cutting filtered slices, and multi-stakeholder self-serve deployment — while keel stays the AUTHORITY (write API + guards; the viewer never becomes a second store).";
        :>> constraintsNote = "keel remains the strict SysML-v2 management authority; the viewer reads (and optionally writes back THROUGH keel's write API + guards), never duplicating management. The stated instinct is to SEPARATE management (keel) from viewing (the explorer) — the core unresolved architecture decision is A (extend keel serve) / B (separate viewer over a committed, versioned keel read/query/graph/temporal API) / C (keel emits declarative view DATA, thin renderer). Handoff recommends B. Six open questions require human decision before architecture (see the handoff §6).";
    }

    // ── Personas (stakeholders). ──
    part safetyEngineerPersona : Persona {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-000000000p01";
        :>> title = "Safety engineer";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "Reviews safety processes and the safety data/evidence connected to them; needs the safety slice without wading through unrelated model content.";
        :>> goals = "See the safety-process landscape and a given process's complete safety picture (connected artifacts/evidence) as a navigable, filtered view.";
    }
    part processOwnerAuditorPersona : Persona {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-000000000p02";
        :>> title = "Process owner / auditor";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "Manages business-process compliance; needs to see current STATE — when the last (annual) review happened, by whom, and whether one is now due/overdue.";
        :>> goals = "See at a glance which processes are current / due-soon / overdue, with reviewer + date, and manage compliance from that.";
    }
    part technicalModelerPersona : Persona {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-000000000p03";
        :>> title = "Technical modeler / viewpoint author";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "Curates viewpoints (query + filter + visual form + audience) so stakeholders get self-serve views without the author's ongoing involvement.";
        :>> goals = "Declare and share a new viewpoint — including its VISUAL form, not just one of three fixed modes — for a named audience.";
    }
    part nonTechnicalStakeholderPersona : Persona {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-000000000p04";
        :>> title = "Non-technical stakeholder";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "Never touches the keel CLI; opens a URL, picks a viewpoint, and explores to get answers without engineering support.";
        :>> goals = "Self-serve answers from a deployed, authenticated URL with saved personal views — read-only by default.";
    }

    // ── Needs (solution-free, MoSCoW). ACCEPT THESE BEFORE ARCHITECTURE (needs-before-architecture). ──
    requirement nViewerViewpointForm : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000n1";
        :>> title = "N-1: a viewpoint is a query AND a visual form for an audience";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "A viewpoint must be declarable as a query (select/traverse/project) AND a chosen VISUAL form targeting a named audience — not limited to three fixed render modes (graph/table/review) with hardcoded palette.";
        :>> priority = Priority::must;
    }
    requirement nViewerExplorableGraph : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000n2";
        :>> title = "N-2: explorable, navigable, scalable graph";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "The graph must support seed-and-expand navigation, drill-into-detail, and stable layout, scaling past a few thousand nodes via INCREMENTAL loading rather than embedding the whole model (which froze at ~2500 nodes).";
        :>> priority = Priority::must;
    }
    requirement nViewerTemporalState : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000n3";
        :>> title = "N-3: calendar-based temporal / freshness state";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "The system must compute and surface CALENDAR-based state from authored date facts — last-reviewed-by/when, due-for-review, overdue, stale/expiring — and allow filtering by it. Today freshness is only git-content-drift, not calendar aging. (Blocks UC-4, the primary gap-driven use case.)";
        :>> priority = Priority::must;
    }
    requirement nViewerCrossCuttingSlices : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000n4";
        :>> title = "N-4: cross-cutting filtered slices around a seed";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "A stakeholder must be able to view only the data interrelated with a seed (process / requirement / need) and hide everything else, over CONFIGURABLE depth and edge selection — generalizing the fixed 1-hop / Need-only slices (element_neighbourhood / need_slice) that exist today.";
        :>> priority = Priority::must;
    }
    requirement nViewerQueryExpansion : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000n5";
        :>> title = "N-5: controlled query expansion (date/range, likely OR)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::internal;
        :>> statement = "The view query vocabulary must support at least date/range predicates (and likely OR) needed for temporal + stakeholder filtering — WITHOUT becoming an unbounded query language (respecting keel's deliberate 'fixed predicate vocabulary, not a sledgehammer' stance). The bound is an open question (handoff Q3).";
        :>> priority = Priority::should;
    }
    requirement nViewerSeparation : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000n6";
        :>> title = "N-6: separate management (keel) from viewing";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::internal;
        :>> statement = "The viewing surface must consume keel as the authoritative engine (query/view/graph/temporal API) rather than duplicating SysML-v2 management; keel's write API + guards remain the ONLY write path. WHICH shape (A extend serve / B separate app over a versioned keel API / C thin renderer over emitted view data) is the core architecture decision (handoff §3; recommendation: B).";
        :>> priority = Priority::must;
    }
    requirement nViewerMultiStakeholder : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000n7";
        :>> title = "N-7: multi-stakeholder deployment (auth, per-user views)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "The viewer must be deployable beyond localhost with authentication and per-user saved views; read-only by default, writes gated. Today keel serve is single-user, localhost-only, no auth. (Blocks UC-6.)";
        :>> priority = Priority::should;
    }
    requirement nViewerPersistedViewpoints : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000n8";
        :>> title = "N-8: persisted, shareable viewpoints + filters + view state";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "Viewpoints, saved filters, and per-user view state must persist and be shareable across users; nothing persists a stakeholder's saved query/scope/layout today.";
        :>> priority = Priority::should;
    }

    // ── Use Cases (concrete usage; trace to the Needs above). ──
    use case ucViewSafetyProcesses : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000u1";
        :>> title = "UC-1: View safety processes";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a safety engineer, I want to see all safety processes as a filtered graph/list, so that I understand the safety-process landscape without unrelated model content. (Serves N-1, N-4.)";
        :>> mainSuccess = "Pick the safety-process viewpoint; the explorer shows only safety-process elements as a navigable graph and a list; unrelated content is hidden.";
        :>> extensions = "Empty result (no safety processes) is reported clearly, not a blank canvas.";
    }
    use case ucViewSafetyDataAroundProcess : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000u2";
        :>> title = "UC-2: View safety data related to a specific process";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a safety engineer, I want to seed on one process and expand to the safety data/artifacts/evidence connected to it via typed edges, so that I can review a process's complete safety picture. (Serves N-2, N-4; generalizes element_neighbourhood/need_slice beyond fixed 1-hop.)";
        :>> mainSuccess = "Seed on a process; expand configurable-depth over chosen edges; only the connected subgraph shows; drill into any element's detail.";
        :>> extensions = "Depth/edge selection is adjustable mid-exploration; layout stays stable as neighbors load.";
    }
    use case ucViewRequirementTraceability : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000u3";
        :>> title = "UC-3: View requirement traceability";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As an engineer/auditor, I want a requirement's full bidirectional trace (need -> requirement -> allocation -> component -> verifying test), so that I can assess coverage and gaps. (Serves N-2, N-4; keel's trace/trace-need already compute this — this is about good explorable presentation + gap highlighting.)";
        :>> mainSuccess = "Seed on any element; see the bidirectional trace; unsatisfied/unverified gaps are highlighted.";
        :>> extensions = "If no satisfy/verify edges exist yet, the gap is shown explicitly, not returned empty.";
    }
    use case ucViewBusinessProcessState : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000u4";
        :>> title = "UC-4: View business processes with calendar state (PRIMARY gap use case)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a process owner/auditor, I want to see business processes and their current state — when the last annual review happened and by whom, and whether one is now due — so that I can manage compliance and see what's overdue at a glance. (Serves N-3, N-5; nothing in keel computes calendar aging today.)";
        :>> mainSuccess = "The business-process viewpoint classifies each process current / due-soon / overdue from authored date facts; shows reviewer + date; filterable by state.";
        :>> extensions = "Review-cadence + last-review compute a next-due date (exact date-fact model is open question Q2).";
    }
    use case ucAuthorViewpoint : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000u5";
        :>> title = "UC-5: Author / curate a viewpoint";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a technical modeler, I want to define a new viewpoint (query + filter + visual form + audience) and share it, so that stakeholders get a self-serve view without my involvement. (Serves N-1, N-8.)";
        :>> mainSuccess = "Declare a viewpoint including its visual form; persist + share it; stakeholders open it self-serve.";
        :>> extensions = "The visual-form authoring surface is an open question (Q4: extend .view.toml / new artifact / presentation layer keyed on audience).";
    }
    use case ucSelfServeStakeholder : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000u6";
        :>> title = "UC-6: Self-serve read for a non-technical stakeholder";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a stakeholder who never touches the keel CLI, I want to open a URL, pick a viewpoint, and explore, so that I get answers without engineering support. (Serves N-6, N-7, N-8.)";
        :>> mainSuccess = "Open a deployed, authenticated URL; pick a viewpoint; explore; save personal views; read-only by default.";
        :>> extensions = "Whether multi-user deployment belongs in keel or a separate viewer is the core A/B/C decision (Q1/Q5).";
    }

    // ── Business gate: human acceptance of the Needs (needs-before-architecture). ──
    verification keelViewerNeedsAccept : Test { :>> id = "6e50d1c0-0113-4c8b-9d2e-00000000000a"; :>> title = "keel-viewer Needs accepted (Business gate)"; :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz accepts the 8 keel-viewer Needs (N-1..N-8: nViewerViewpointForm / nViewerExplorableGraph / nViewerTemporalState / nViewerCrossCuttingSlices / nViewerSeparation [must], nViewerQueryExpansion / nViewerMultiStakeholder / nViewerPersistedViewpoints [should]) AS-IS via the explicit 'Accept the Needs as-is' selection on 2026-07-14 — the Business gate. Architecture (SystemRequirements + the A/B/C Decision) may now be derived from these accepted Needs."; }
    part keelViewerNeedsAcceptR1 : TestResult { :>> id = "6e50d1c0-0113-4c8b-9d2e-00000000000b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5f82eec"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "wweatherholtz"; }
}
