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
| Track an issue | Author a `part <name> : Issue { ... }` in `.tracking/issues.sysml` with `description`, `discoveredInField`, and `relatedTask` (the backlog action to address it). Surface with `sysmlv2 view issues` (the TOML view, M2.1). |
| Mark done | Use `sysmlv2 append-result --file FILE --task TASK --sha SHA [--verdict pass\|fail] [--judged-by ACTOR] [--judged-at DATE]` — auto-generates UUID, enforces append-only N+1. Or directly APPEND `part <task>DoDR<n> : TestResult` (same fields: `id`, `judgedAgainst`, `judgedAt`, `judgedBy`, `outcome = VerdictKind::pass`). `method=confirmation` requires the human's explicit sign-off. |
| Record a phase-gate result | Use `sysmlv2 append-gate-result --file FILE --gate GATE --sha SHA [--verdict pass\|fail] [--judged-by ACTOR] [--judged-at DATE]` — auto-generates UUID, enforces append-only N+1, inserts the `part <gate>R<n> : TestResult` after the gate's `verification` block. (Gates are `verification`s, not actions — distinct from `append-result`.) `method=confirmation` gates require the human's explicit sign-off as `judgedBy`. |
| Add a new task | Use `sysmlv2 add-task --file FILE --def DEF --task TASK --dod TEXT [--method test\|inspect\|confirmation\|demo\|analysis]` — auto-generates UUID, rejects duplicate names. |
| Record a decision | Author a `Decision` part (copy a recent `.engine/decisions/` file) with `context`/`decision`/`rationale`/`consequences`; a NEW accepted decision also carries an acceptance event — `verification dNNNNAccept : Test {method=confirmation}` + `part dNNNNAcceptR1 : TestResult {outcome=pass; judgedBy=<human>}` (D0066). |
| Check attestation coverage | Rust-native (no kernel): `sysmlv2 attestation-coverage [ROOT]`. Lists `status=accepted` decisions missing their acceptance event (the declared `attestation-coverage` viewpoint; M2.2a). |
| Find orphaned / dangling items | Rust-native (no kernel): `sysmlv2 orphans [ROOT]`. Tasks with no `DoD`, Issues with no/dangling `relatedTask` (the `orphans` viewpoint; M2.2b). |
| Audit sprint-process adherence | Rust-native (no kernel): `sysmlv2 audit [ROOT]`. Charter coverage, ceremony completeness, estimation discipline, sitting-review currency, split ACTIONABLE vs grandfathered (D0046; M2.2b). |
| Run the forward guards | `sysmlv2 guard` runs ALL six (exit≠0 on any violation); `sysmlv2 guard <name> [ROOT]` runs one of `actors`/`acceptance-events`/`sprint-coverage`/`ceremony`/`charter`/`process-change` (D0074 M3, parity-verified vs the `.engine/tools/validate/validate_*.py` guards, which retire at M4). |
| Register an AI skill | Add an `AISkill`/`Agent` to `skills-registry.sysml`. |
| Charter work to its origin | `#CharteredBy dependency from <workItem> to <decision/need/requirement>;` (import `EngineRelationships::*`) — the charter-lineage edge (D0068). List: `sysmlv2 view charter-trace`. |
| Record a process change | Prefix the Decision part with `#ProspectiveChange` (or `#SafetyChange` if downstream items must be reprocessed) — `#ProspectiveChange part dNNNN : Decision { ... }` (import `EngineRelationships::*`); which process + when are git-derived (D0070). List: `sysmlv2 view process-changes`. |
| Which process version governed an item | Rust-native (no kernel): `sysmlv2 governing-version <storyName> [ROOT]`. The process-def state as-of the item's charter (charter-time freeze, D0068) + which process-change Decisions were in force then vs. after (D0070). M2.2c. |
| What must be re-processed after a safety change | Rust-native (no kernel): `sysmlv2 reprocess-candidates [ROOT]`. Items chartered under a process version later superseded by a `#SafetyChange` (prospective changes never flag — D0062). M2.2c. |
| List suspect (stale-evidence) tasks | `sysmlv2 suspect [ROOT]` — done tasks whose evidence is stale: criterion-text drift **and** D0050 deliverable-source drift. orient's authoritative suspect set (D0076; orient is the single source of truth). |

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
