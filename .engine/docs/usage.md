# Engine Usage Guide

How to use the work-tracking engine, for humans and AI agents.
(Rewritten 2026-06-11 — the previous guide taught pre-rewrite syntax that no longer parses.)

## Mental model

Every artifact — work items, requirements, tests, decisions, safety analysis — is a
typed element connected by typed edges. The model is `.sysml` text; there is no other
tracking system. Computed state (done/ready/suspect, coverage, trace) is a **view**,
recomputed from the text + git history — never stored or authored.

## Starting a project on the engine

Instance data lives in **`.tracking/`** (see `.tracking/README.md`). Copy authoring
idioms from `.engine/docs/tracking-template.sysml` — it parses green. A typical file:

```sysml
package MyProjectNeeds {
    private import EngineElement::*;
    private import EngineNeeds::*;

    requirement n1 : Need {
        :>> id = "<uuid>";
        :>> title = "...";
        :>> source = NeedSource::customer;
        :>> statement = "...";
        :>> priority = Priority::must;
    }
}
```

`schema/core` packages (`EngineNeeds`, `EngineWork`, `EngineVerification`, ...) are the
canonical vocabulary. Validate every change (CLAUDE.md §5) with the validator covering
the layer you touched.

## Day-to-day

| You want to... | Do this |
|---|---|
| Add work | Add an `action` task to a `.tracking` backlog `action def`, with a one-line `verification <task>DoD : Test` criterion |
| Order work | `first taskA then taskB;` (succession) |
| See what's next | Rust-native (no kernel): `sysmlv2 orient [ROOT]` (JSON) / `sysmlv2 whats-next [ROOT]` (ready list, one per line) |
| Run / define a viewpoint | `sysmlv2 view <name> [ROOT]` — executes `.engine/views/<name>.view.toml` (a select + optional traverse + project filter; D0075) and prints the subgraph JSON. Author/edit the `.view.toml` directly; see the `view` skill. Structural lenses are TOML views: `issues`, `decisions`, `process-changes`, `charter-trace`, `processes` (M2.1). |
| Track an issue | Author a `part <name> : Issue { ... }` in `.tracking/issues.sysml` (`description`, `discoveredInField`, `relatedTask` = the related/causing task). Then TRIAGE it (issue-resolution skill, D0077/D0078): add `#Resolves dependency from <resolver> to <issueNNN>;` where `<resolver>` is a resolving action (create one if none) or a mooting Decision — `sysmlv2 guard issues` fails on an untriaged Issue. |
| See open issues | `sysmlv2 open-issues` — issues with no complete resolver (open set + each resolver + `untriaged`); also surfaced as `open_issues` in `sysmlv2 orient`. Resolution is COMPUTED (resolved iff the `#Resolves` resolver is done/accepted), never a prose "RESOLVED" note (D0077). |
| Inspect the backlog | `sysmlv2 outstanding` (not-done tasks); `sysmlv2 item <task>` (done/deps/DoD/results); `sysmlv2 trace <task>` (transitive upstream + downstream over the succession DAG). |
| Trace a need | `sysmlv2 trace-need <need>` — forward closure over satisfy/allocate edges (need → requirement → component). |
| Inspect the workflows | `sysmlv2 workflows` — each workflow's phases as Kahn topological waves over its succession edges. |
| List declared viewpoints | `sysmlv2 view viewpoints` — the viewpoint-registry as a listing (concern-coverage index, D0056/D0057). |
| Mark done | Use `sysmlv2 append-result --file FILE --task TASK --sha SHA [--verdict pass\|fail] [--judged-by ACTOR] [--judged-at DATE]` — auto-generates UUID, enforces append-only N+1. Or directly APPEND `part <task>DoDR<n> : TestResult` (same fields: `id`, `judgedAgainst`, `judgedAt`, `judgedBy`, `outcome = VerdictKind::pass`). `method=confirmation` requires the human's explicit sign-off. |
| Record a phase-gate result | Use `sysmlv2 append-gate-result --file FILE --gate GATE --sha SHA [--verdict pass\|fail] [--judged-by ACTOR] [--judged-at DATE]` — auto-generates UUID, enforces append-only N+1, inserts the `part <gate>R<n> : TestResult` after the gate's `verification` block. (Gates are `verification`s, not actions — distinct from `append-result`.) `method=confirmation` gates require the human's explicit sign-off as `judgedBy`. |
| Add a new task | Use `sysmlv2 add-task --file FILE --def DEF --task TASK --dod TEXT [--method test\|inspect\|confirmation\|demo\|analysis\|critique]` — auto-generates UUID, rejects duplicate names. |
| Record a decision | Author a `Decision` part (copy a recent `.engine/decisions/` file) with `context`/`decision`/`rationale`/`consequences`; a NEW accepted decision also carries an acceptance event — `verification dNNNNAccept : Test {method=confirmation}` + `part dNNNNAcceptR1 : TestResult {outcome=pass; judgedBy=<human>}` (D0066). |
| Check attestation coverage | Rust-native (no kernel): `sysmlv2 attestation-coverage [ROOT]`. Lists `status=accepted` decisions missing their acceptance event (the declared `attestation-coverage` viewpoint; M2.2a). |
| Find orphaned / dangling items | Rust-native (no kernel): `sysmlv2 orphans [ROOT]`. Tasks with no `DoD`, Issues with no/dangling `relatedTask` (the `orphans` viewpoint; M2.2b). |
| Audit sprint-process adherence | Rust-native (no kernel): `sysmlv2 audit [ROOT]`. Charter coverage, ceremony completeness, estimation discipline, sitting-review currency, split ACTIONABLE vs grandfathered (D0046; M2.2b). |
| Run the forward guards | `sysmlv2 guard` runs ALL seven (exit≠0 on any violation); `sysmlv2 guard <name> [ROOT]` runs one of `actors`/`acceptance-events`/`sprint-coverage`/`ceremony`/`charter`/`process-change`/`issues` (D0074 M3 + D0077; parity-verified vs the retired `.engine/tools/validate/validate_*.py` guards). |
| Check assurance coverage | `sysmlv2 coverage [ROOT]` — for each Need / SystemRequirement / accepted Decision, the computed coverage **tier** (D0082): `verified` (reproducible verify-edge evidence; needs transitively via a verified requirement) > `attested` (decision acceptance event) > `addressed` (charter-dod work / trace only — a claim) > `suspect` (stale) > `uncovered`. Per-type tier summary + gap set; gate-covered = verified|attested. Honest by construction — never stored (D0079 C). |
| Critique an element | Run the `element-critique` skill (D0080/D0079): adversarially verify a Need/Requirement/Decision through canon lenses by an INDEPENDENT critic — `verification <name> : Test { method = VerificationMethod::critique; lens = CritiqueLens::<lens>; critiquedBy = CriticKind::<aiModel\|human\|tool>; severity = Severity::<...> }` + a TestResult, linked to the target by `#Verify dependency from <name> to <element>;` (judgedBy != author). Findings → severity-carrying `Issue`s (#Resolves loop); ≥Medium needs human disposition. |
| Check critique coverage | `sysmlv2 critique-coverage [ROOT]` — per-element × required-lens matrix (Core-3: Need/Requirement = completeness/correctness/testability; Decision = completeness/correctness/feasibility), per-type summary + the gap set (D0080). CHARTER-TIME SCOPED (D0081): each element shows `governed` (created after D0080 = held to rigor) vs grandfathered; the gap set + `guard critique` count only governed elements. `guard critique` is ENFORCED in pre-commit (a hard gate) — grandfathered work passes, every NEW requirement/need/decision must carry its Core-3 critiques. |
| See the whole model (diagram) | `sysmlv2 diagram [ROOT] > x.html` — a comprehensive interactive traceability diagram (D0085): every element as a typed node + its metadata, every typed edge, in one self-contained HTML page (cytoscape) with type/edge filters, search, click-to-focus a neighborhood, and fit. `Test`/`TestResult` toggle OFF by default for legibility. A computed `#View` — regenerate on demand, never commit (generated `*.html` is git-ignored). The `diagram` viewpoint; deploy via the `diagram` skill. |
| Render any view as an artifact | `sysmlv2 render <view> --mode graph\|table\|review [ROOT] > x.html` — the modular renderer over the view layer (D0086): `graph` (cytoscape; `model` = whole-model diagram, else a view's subgraph; derived `contains`/`resultof` edges default off), `table` (sortable/searchable rows of any declared view), `review` (table + per-row accept/finding + lens/severity/rationale capture + Export-JSON). A computed `#View` (git-ignored). The `render` viewpoint; deploy via the `render` skill. |
| Health/opportunity scorecards (reports) | `sysmlv2 report <assurance\|traceability\|quality-debt\|flow\|governance> [--html] [--trend] [ROOT]` — computed AGGREGATE scorecards (D0087) rolling up the per-element views: **assurance** (verification coverage %, critique %, attestation %, open findings by severity, suspect load, READY/NOT), **traceability** (% needs/requirements verified, satisfy/verify edge completeness — DO-178C-style), and **governance** (decisions accepted/superseded, acceptance integrity, process-change, supersession) are HEALTH reads; **quality-debt** (charter debt, requirements volatility, suspect+stale) + **flow** (ready, WIP, velocity, cycle time, time/story-point, lead time, predictability, throughput, aging WIP) are OPPORTUNITY reads; **friction** is the D0054 write-path-vs-spreadsheet authoring-friction benchmark. `--html` emits a card scorecard. Grounded in the INCOSE DE Measurement Framework + SE Leading Indicators. Add `--trend` for a git-derived sparkline of each report's headline metric (recomputes the full pipeline at ~12 recent commits via a throwaway worktree — accurate but slow; computed from git, never stored). A computed `#View` (git-ignored); the `report` viewpoint, deployed by the `report` skill. |
| Review/disposition elements (round-trip) | `sysmlv2 render <view> --mode review > r.html`, disposition (accept/finding + rationale), Export JSON, then `sysmlv2 apply-review --batch r.json --sha <commit> --judged-by <you> --judged-at <date>` (D0086). Each disposition lands as a NEW LINKED `method=critique` verification + TestResult + `#Verify` edge in `.tracking/critiques.sysml` (the human is an independent critic, D0080): accept attests state; a finding (fail) carries severity + lens and INDUCES SUSPICION (`sysmlv2 suspect` → `critique_suspect`) until cleared. The JSON is transport; `.tracking` is truth. |
| Which decisions are most load-bearing? | `sysmlv2 decisions [ROOT]` — accepted Decisions ranked by dependence (charters-to ×2 + cross-citations from other decisions) + antiquation flags: `uncritiqued` (lacks full Core-3 element-critique — the critique worklist), `references_retired` (cites a retired mechanism, e.g. query.py/parity_check), `superseded_in_part` (heuristic: a later decision mentions it near supersede/retire/replace). The formalized report (replaces ad-hoc ranking scripts); computed, never stored. |
| Is the deliverable assured? | `sysmlv2 assured [ROOT]` — the composite readiness verdict (D0079 c; charter-time scoped, D0081): READY iff GOVERNED coverage complete AND GOVERNED critique complete AND every ≥Medium finding dispositioned AND no Critical open AND invariants green. `stale_verifications` is advisory (re-verify; not gating). NOT-READY lists exact per-category blockers + advisories. `sysmlv2 guard assured` is ENFORCED in pre-commit (passes on grandfathered work; holds future work to full assurance). |
| Register an AI skill | Add an `AISkill`/`Agent` to `skills-registry.sysml`. |
| Charter work to its origin | `#CharteredBy dependency from <workItem> to <decision/need/requirement>;` (import `EngineRelationships::*`) — the charter-lineage edge (D0068). List: `sysmlv2 view charter-trace`. |
| Record a process change | Prefix the Decision part with `#ProspectiveChange` (or `#SafetyChange` if downstream items must be reprocessed) — `#ProspectiveChange part dNNNN : Decision { ... }` (import `EngineRelationships::*`); which process + when are git-derived (D0070). List: `sysmlv2 view process-changes`. |
| Which process version governed an item | Rust-native (no kernel): `sysmlv2 governing-version <storyName> [ROOT]`. The process-def state as-of the item's charter (charter-time freeze, D0068) + which process-change Decisions were in force then vs. after (D0070). M2.2c. |
| What must be re-processed after a safety change | Rust-native (no kernel): `sysmlv2 reprocess-candidates [ROOT]`. Items chartered under a process version later superseded by a `#SafetyChange` (prospective changes never flag — D0062). M2.2c. |
| List suspect (stale-evidence) tasks | `sysmlv2 suspect [ROOT]` — done tasks whose evidence is stale: criterion-text drift **and** D0050 deliverable-source drift (per-task source paths, `.engine/deliverable-manifest.txt`). `sysmlv2 suspect --explain` prints WHY each is flagged (which path drifted, at which commit, vs verified-at). orient's authoritative suspect set (D0076; the single source of truth). Clear a deliverable-drift suspect by re-verifying at HEAD (`append-result --sha <HEAD>`). |

## Edge cheatsheet (pilot-confirmed syntax only)

| To say... | Use |
|---|---|
| X fulfills requirement R | `satisfy R by X;` |
| Verification V checks requirement R | `verification def V { subject s; objective r; }` (structural — `verify R by V` does **not** parse) |
| B derives from / refines A | `B :> A;` (specialization; v1 `derive`/`refine`/`trace` don't exist in v2) |
| B can't start until A | `first A then B;` in a backlog; `#OrderingOnly` marks non-semantic dependencies |
| Function F runs on component C | `allocate f to c;` |
| D2 replaces D1 | a `dependency` from D2 to D1 marked `#Supersede` |
| Work W was chartered by origin O | `#CharteredBy dependency from W to O;` (D0068; carries process identity by lineage) |
| Decision D changed a process | `#ProspectiveChange part D : Decision { ... }` (`#SafetyChange` = downstream must reprocess) — a prefix marker on the Decision; which process is git-derived (D0070) |

## What never goes in the model

Computed state (`ready`, `done`, `in_review` — views, never fields), runtime events
(CI runs, telemetry), and deliverable-domain vocabulary (belongs in the deliverable's
model, not the engine).

## For AI agents

Follow CLAUDE.md: classify every request (§3) before acting; direct text editing is the
bootstrap write path (§4); validation green before every commit (§5); `main` is the only
branch; commits auto-push. `writePolicy` in the skills registry is the *intended* write
boundary — enforcement arrives with the write API (until then it binds by discipline).
