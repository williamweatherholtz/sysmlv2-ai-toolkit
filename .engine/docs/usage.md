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
| See what's next | `python .engine/tools/query.py orient` — or Rust-native (no kernel): `sysmlv2 orient [ROOT]` (JSON) / `sysmlv2 whats-next [ROOT]` (ready list, one per line) |
| Track an issue | Author a `part <name> : Issue { ... }` in `.tracking/issues.sysml` with `description`, `discoveredInField`, and `relatedTask` (the backlog action to address it). Surface with `python .engine/tools/query.py issues`. |
| Mark done | Use `sysmlv2 append-result --file FILE --task TASK --sha SHA [--verdict pass\|fail] [--judged-by ACTOR] [--judged-at DATE]` — auto-generates UUID, enforces append-only N+1. Or directly APPEND `part <task>DoDR<n> : TestResult` (same fields: `id`, `judgedAgainst`, `judgedAt`, `judgedBy`, `outcome = VerdictKind::pass`). `method=confirmation` requires the human's explicit sign-off. |
| Record a phase-gate result | Use `sysmlv2 append-gate-result --file FILE --gate GATE --sha SHA [--verdict pass\|fail] [--judged-by ACTOR] [--judged-at DATE]` — auto-generates UUID, enforces append-only N+1, inserts the `part <gate>R<n> : TestResult` after the gate's `verification` block. (Gates are `verification`s, not actions — distinct from `append-result`.) `method=confirmation` gates require the human's explicit sign-off as `judgedBy`. |
| Add a new task | Use `sysmlv2 add-task --file FILE --def DEF --task TASK --dod TEXT [--method test\|inspect\|confirmation\|demo\|analysis]` — auto-generates UUID, rejects duplicate names. |
| Record a decision | Author a `Decision` part (see any `.engine/decisions/` file for the pattern). |
| Register an AI skill | Add an `AISkill`/`Agent` to `skills-registry.sysml`. |

## Edge cheatsheet (pilot-confirmed syntax only)

| To say... | Use |
|---|---|
| X fulfills requirement R | `satisfy R by X;` |
| Verification V checks requirement R | `verification def V { subject s; objective r; }` (structural — `verify R by V` does **not** parse) |
| B derives from / refines A | `B :> A;` (specialization; v1 `derive`/`refine`/`trace` don't exist in v2) |
| B can't start until A | `first A then B;` in a backlog; `#OrderingOnly` marks non-semantic dependencies |
| Function F runs on component C | `allocate f to c;` |
| D2 replaces D1 | a `dependency` from D2 to D1 marked `#Supersede` |

## What never goes in the model

Computed state (`ready`, `done`, `in_review` — views, never fields), runtime events
(CI runs, telemetry), and deliverable-domain vocabulary (belongs in the deliverable's
model, not the engine).

## For AI agents

Follow CLAUDE.md: classify every request (§3) before acting; direct text editing is the
bootstrap write path (§4); validation green before every commit (§5); `main` is the only
branch; commits auto-push. `writePolicy` in the skills registry is the *intended* write
boundary — enforcement arrives with the write API (until then it binds by discipline).
