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

    // =====================================================================
    // EXPANSION (Sprint 210) — additional Personas + UseCases (scenarios) + Needs. The new Needs
    // (N-9..N-14) are PENDING a SECOND acceptance gate — do NOT derive architecture for them until the
    // human accepts (needs-before-architecture). Personas + UseCases inform; the Needs gate is the bar.
    // =====================================================================

    // ── New Personas. ──
    part systemsEngineerArchitectPersona : Persona {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000c1";
        :>> title = "Systems engineer / architect";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "Walks the full model, assesses coverage/gaps, and reasons about the blast radius of a change before making it.";
        :>> goals = "Navigate deeply, see what's unverified/uncovered, and understand what a proposed change would affect.";
    }
    part programManagerPersona : Persona {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000c2";
        :>> title = "Program / project manager";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "Tracks status, coverage, and what is overdue at a glance; non-technical depth, decision-support breadth.";
        :>> goals = "See a scope's health (coverage %, overdue reviews, open gaps) without reading the model in detail.";
    }
    part complianceQualityManagerPersona : Persona {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000c3";
        :>> title = "Compliance / quality manager";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "Owns regulatory/quality audits; needs evidence completeness, provenance (who reviewed/decided what, when), and exportable snapshots.";
        :>> goals = "Prove coverage + provenance for an audit and export a defensible snapshot for records.";
    }

    // ── New Needs (N-9..N-14). PENDING acceptance — no architecture until accepted. ──
    requirement nViewerCoverageGap : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000d9";
        :>> title = "N-9: coverage / gap views over a scope";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "A stakeholder must see, over a chosen scope, what is unsatisfied / unverified / uncovered (the gaps) — not just the present items — so coverage can be assessed and gaps closed. (keel computes assurance coverage today; this surfaces it explorably.)";
        :>> priority = Priority::must;
    }
    requirement nViewerChangeImpact : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000da";
        :>> title = "N-10: change-impact (blast radius)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "A user must see, for a given element, what depends on it (downstream dependents over typed edges) so the blast radius of a proposed change is visible before making it.";
        :>> priority = Priority::should;
    }
    requirement nViewerNotifications : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000db";
        :>> title = "N-11: subscribe to state-change / freshness events";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "A stakeholder must be able to subscribe to state changes on a scope — a review became due/overdue, evidence went stale, a gap opened — and be notified, rather than having to poll. (Builds on N-3 temporal state.)";
        :>> priority = Priority::should;
    }
    requirement nViewerExportShare : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000dc";
        :>> title = "N-12: export / share a viewpoint snapshot";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "A stakeholder must be able to export or share a point-in-time snapshot of a viewpoint (link / PDF / image) for someone who cannot or need not open the live tool — with the snapshot's as-of date recorded.";
        :>> priority = Priority::should;
    }
    requirement nViewerBaselineCompare : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000dd";
        :>> title = "N-13: compare a viewpoint across baselines / time";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "A stakeholder must be able to compare a viewpoint between two points in time or baselines — what was added / removed / changed / re-verified since — so drift and progress are visible. (keel's git history is the substrate.)";
        :>> priority = Priority::could;
    }
    requirement nViewerAuditProvenance : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000de";
        :>> title = "N-14: audit-trail / provenance browsing";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::regulatory;
        :>> statement = "An auditor must be able to browse the provenance of any element — who reviewed / decided / verified it, when, and against which commit — as first-class navigable history, so an audit can trace evidence to its author + date. (keel authors judgedBy/judgedAt/judgedAgainst today.)";
        :>> priority = Priority::should;
    }

    // ── New UseCases (UC-7..UC-13) with scenarios (main success + alternate + exception). ──
    use case ucCoverageGapDashboard : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000e7";
        :>> title = "UC-7: Coverage / gap dashboard";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a program manager / auditor, I want a dashboard of what is unsatisfied / unverified / uncovered across a chosen scope, so that I can see health and target the gaps. (Serves N-9.)";
        :>> mainSuccess = "Pick a scope; the dashboard shows coverage % and lists the uncovered/unverified/unsatisfied items, each drillable to the element.";
        :>> extensions = "ALT: scope fully covered -> a green 'no gaps' state (not an empty screen). EXCEPTION: scope has no verifiable elements -> report 'nothing to cover here' rather than 0%. ALT: click a gap -> jump to the element in the explorable graph (UC-2/UC-3).";
    }
    use case ucChangeImpact : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000e8";
        :>> title = "UC-8: Change-impact (blast radius)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a systems engineer, I want to see everything that depends on a given element, so that I understand the blast radius before I change it. (Serves N-10; inverse-direction slice of N-4.)";
        :>> mainSuccess = "Seed on an element; the view shows the downstream dependents over configurable typed edges, ranked/grouped by distance; each is drillable.";
        :>> extensions = "ALT: no dependents -> 'safe to change, nothing depends on this'. EXCEPTION: a dependency cycle -> the cycle is shown, not infinite-expanded. ALT: filter impact to a type (e.g. only affected Tests/Requirements).";
    }
    use case ucFreshnessNotifications : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000e9";
        :>> title = "UC-9: Subscribe to freshness / review-due events";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a process owner, I want to subscribe to a scope and be notified when a review comes due/overdue or evidence goes stale, so that I act without polling. (Serves N-11; builds on N-3.)";
        :>> mainSuccess = "Subscribe to a scope; when temporal state crosses a threshold (due-soon/overdue/stale), a notification is delivered with the element + what changed.";
        :>> extensions = "ALT: nothing due -> no noise (only fire on transitions). EXCEPTION: an element's cadence is undefined -> a one-time 'cannot compute due date, configure cadence' notice, not silence. ALT: digest vs immediate delivery preference.";
    }
    use case ucAuditProvenance : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000ea";
        :>> title = "UC-12: Audit-trail / provenance browsing";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a compliance/quality manager or auditor, I want to browse who reviewed/decided/verified an element, when, and against which commit, so that I can trace evidence to its author + date for an audit. (Serves N-14.)";
        :>> mainSuccess = "Open any element's provenance; see its ordered history of reviews/decisions/verifications with reviewer, date, and commit, each linking to the evidence.";
        :>> extensions = "ALT: element never reviewed -> 'no recorded provenance' (a finding, not a blank). EXCEPTION: a judgedAgainst commit is unresolvable in git -> flag it as suspect provenance, don't hide it. ALT: export this provenance (UC-10).";
    }
    use case ucExportShare : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000eb";
        :>> title = "UC-10: Export / share a viewpoint snapshot";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a stakeholder, I want to export or share a point-in-time snapshot of a viewpoint (link/PDF/image), so that someone who won't open the tool still gets the view. (Serves N-12.)";
        :>> mainSuccess = "From a viewpoint, export a snapshot stamped with its as-of date + scope; share it as a link or file.";
        :>> extensions = "ALT: share a live link (auth-gated) vs a frozen file. EXCEPTION: viewpoint too large to render statically -> export the scoped/filtered subset with a note. ALT: snapshot recorded so it is reproducible from the as-of commit.";
    }
    use case ucBaselineCompare : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000ec";
        :>> title = "UC-11: Compare a viewpoint across baselines / time";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As an engineer/auditor, I want to compare a viewpoint between two times or baselines, so that I can see what was added/removed/changed/re-verified since. (Serves N-13; git history is the substrate.)";
        :>> mainSuccess = "Pick two points (dates/baselines); the view diffs the viewpoint's contents and highlights added/removed/changed/re-verified elements.";
        :>> extensions = "ALT: no change between the two points -> 'no drift'. EXCEPTION: one point predates an element's existence -> shown as 'added since'. ALT: scope the comparison to one type (e.g. only requirement changes).";
    }
    use case ucGuidedOrientation : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000ed";
        :>> title = "UC-13: Guided orientation for a first-time stakeholder";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a non-technical stakeholder opening a viewpoint for the first time, I want a short guided tour of what I'm seeing and how to explore, so that I can self-serve without training. (Serves N-1, N-6.)";
        :>> mainSuccess = "On first open, a brief tour explains the viewpoint's purpose, the key elements, and how to expand/filter/drill; dismissible + re-openable.";
        :>> extensions = "ALT: returning user -> tour is skipped by default. EXCEPTION: an empty viewpoint -> the tour explains why it's empty + how to widen the scope.";
    }

    // ── Business gate 2: human acceptance of the expansion Needs N-9..N-14 (needs-before-architecture). ──
    verification keelViewerNeedsAccept2 : Test { :>> id = "6e50d1c0-0113-4c8b-9d2e-00000000001a"; :>> title = "keel-viewer expansion Needs N-9..N-14 accepted (Business gate 2)"; :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz accepts the 6 expansion Needs (N-9 nViewerCoverageGap [must], N-10 nViewerChangeImpact / N-11 nViewerNotifications / N-12 nViewerExportShare / N-14 nViewerAuditProvenance [should], N-13 nViewerBaselineCompare [could]) AS-IS via the explicit 'Accept N-9..N-14 as-is' selection on 2026-07-14 — the second Business gate. Architecture (SystemRequirements) may now be derived from these accepted Needs, under the existing shape B (D0114)."; }
    part keelViewerNeedsAccept2R1 : TestResult { :>> id = "6e50d1c0-0113-4c8b-9d2e-00000000001b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4510d0e"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "wweatherholtz"; }

    // =====================================================================
    // SCOPE REVISION (Sprint 212, D0115) — human feedback 2026-07-14. Four Needs superseded (out of
    // current scope); two new Needs added (PENDING a third acceptance gate). See D0115 for the why.
    // =====================================================================

    // N-7 dropped (single-user, unauthenticated; saveable viewpoints = N-8); N-11 + N-14 deferred;
    // N-9 refocused from a coverage dashboard onto AI critique (replaced by N-15/N-16).
    #Supersede dependency from d0115 to nViewerMultiStakeholder;
    #Supersede dependency from d0115 to nViewerNotifications;
    #Supersede dependency from d0115 to nViewerAuditProvenance;
    #Supersede dependency from d0115 to nViewerCoverageGap;

    // ── New Needs (N-15, N-16) — the human's actual goal: AI-assisted critique of constrained systems.
    //    PENDING acceptance (third gate); no architecture until accepted. ──
    requirement nViewerIterativeCritique : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000f5";
        :>> title = "N-15: iterative AI critique over a constrained query (axis-iterated)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "A user must be able to run AI critique ITERATIVELY over a constrained query result, iterating over ONE chosen axis (each element along that dimension) and recording a finding per element — e.g. critique each process + its downstream members for best practices, or critique whether a requirement's downstream items semantically fully satisfy it. Reuses keel's element-critique + agent bridge; findings land as tracked critiques (apply-review).";
        :>> priority = Priority::must;
    }
    requirement nViewerInProgramEdit : Need {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000f6";
        :>> title = "N-16: modify AND critique elements in-program";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> source = NeedSource::operator;
        :>> statement = "A user must be able to MODIFY an element and (re)critique it WITHIN the viewer — edit-then-critique in one place — with every change written back through keel's write API + guards (never a second store), so critique and revision iterate tightly on the same surface.";
        :>> priority = Priority::must;
    }

    // ── Business gate 3: human acceptance of N-15 + N-16 (needs-before-architecture). ──
    verification keelViewerNeedsAccept3 : Test { :>> id = "6e50d1c0-0113-4c8b-9d2e-00000000002a"; :>> title = "keel-viewer critique Needs N-15 + N-16 accepted (Business gate 3)"; :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz accepts N-15 (nViewerIterativeCritique) + N-16 (nViewerInProgramEdit), both must, via the explicit 'Accept N-15 + N-16 — build downstream' selection on 2026-07-14. Architecture (SystemRequirements + use cases + backlog) may now be derived, under shape B (D0114)."; }
    part keelViewerNeedsAccept3R1 : TestResult { :>> id = "6e50d1c0-0113-4c8b-9d2e-00000000002b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "83993ce"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "wweatherholtz"; }

    // ── UseCases for the accepted critique Needs (the human's #1/#2 examples + in-program edit). ──
    use case ucCritiqueProcessDownstream : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000fa";
        :>> title = "UC-14: Critique a process + its downstream for best practices";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a modeler/reviewer, I want to seed on a process and run AI critique over it AND its downstream members (best-practice lens), so that I improve the whole chain, not just one node. (Serves N-15; the human's example #1.)";
        :>> mainSuccess = "Seed a process; the tool slices its downstream (configurable depth/edges) and iterates AI critique per element on a best-practice lens; each finding is recorded as a tracked critique (apply-review).";
        :>> extensions = "ALT: findings ranked by severity across the chain. EXCEPTION: an element already critiqued at HEAD -> reuse/skip unless forced. ALT: accept/dismiss a finding inline (disposition).";
    }
    use case ucCritiqueRequirementComprehensiveness : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000fb";
        :>> title = "UC-15: Critique requirement comprehensiveness (do downstream items satisfy it?)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As an engineer, I want to seed on a requirement and have AI critique whether its downstream items SEMANTICALLY fully satisfy it, so that I find comprehensiveness gaps a coverage count would miss. (Serves N-15; the human's example #2.)";
        :>> mainSuccess = "Seed a requirement; the tool gathers its satisfying/derived downstream and runs an AI critique judging semantic sufficiency; the verdict + gaps are recorded as a tracked critique.";
        :>> extensions = "ALT: 'fully satisfied' -> a positive recorded finding. EXCEPTION: no downstream items -> critique reports the requirement is undecomposed. ALT: iterate the critique over each downstream item as the axis.";
    }
    use case ucModifyAndCritiqueInProgram : UseCase {
        :>> id = "6e50d1c0-0113-4c8b-9d2e-0000000000fc";
        :>> title = "UC-16: Modify and (re)critique an element in-program";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> description = "As a modeler, I want to edit an element and immediately (re)critique it within the viewer, so that critique -> fix -> re-critique iterates on one surface. (Serves N-16.)";
        :>> mainSuccess = "Open an element; edit a field (written back via the keel write API + guards); re-run critique; the new finding supersedes the prior, all on the same surface.";
        :>> extensions = "ALT: an edit failing a guard is rejected inline with the guard message (no bad write lands). EXCEPTION: a concurrent change to the same element -> the edit rebases/flags a conflict (multi-thread, D0108). ALT: revert the edit if the re-critique is worse.";
    }
}
