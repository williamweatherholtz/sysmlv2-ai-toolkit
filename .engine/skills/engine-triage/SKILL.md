---
name: engine-triage
description: |
  Routes EVERY request in this engine repo through the work-tracking discipline before any
  action. Breaks the request into parts and routes EACH to its process — CHANGE / EXECUTE /
  RECORD / VIEW / ORIENT (CLAUDE.md §3) — states the routes out loud, and FLAGS anything that
  does not cleanly map to a process rather than force-fitting it. Use at the start of every
  request that changes a workflow/schema, produces a tracked artifact, records a
  decision/test result/issue, asks for a computed answer, or asks where things stand. This is
  the guard that stops actions slipping past the process (e.g. recording a confirmation that
  was never given, or doing delivery/engine work with no sprint). Do NOT use for purely
  conversational replies that change nothing. CLAUDE.md §3 is the source of truth — this skill
  is the always-on checklist, fired every turn by a UserPromptSubmit hook (D0064).
metadata:
  version: 0.2.0
  domain: [process-discipline, request-routing, work-tracking, MBSE, SysMLv2]
  writePolicy: read-only
  engine: keel-ai-toolkit
---

# engine-triage — classify before you act

The engine tracks the *work of building things*. Every substantive request must be
routed through the discipline **before** acting. CLAUDE.md §3 is the source of truth;
this skill is the per-request checklist that makes the route-first step visible and
mandatory, so nothing slips past silently. It is fired every turn by a `UserPromptSubmit`
hook (`.engine/tools/triage_reminder.py`, D0064).

## The checklist (do this first, every time)

1. **Break the request into parts** and classify EACH by *what it changes*:

   | Category   | The part…                                                     | Route |
   |------------|---------------------------------------------------------------|-------|
   | CHANGE     | changes a workflow / phase / gate / schema definition         | §3a   |
   | EXECUTE    | produces the active phase's typed artifact (tracked work)     | §3b   |
   | RECORD     | records ONE atomic fact (decision / test result / issue)      | §3c   |
   | VIEW       | asks for a computed answer (status, trace, stale set, a doc)  | §3d   |
   | ORIENT     | asks where things stand / what is next                        | §3f   |

2. **State the routes out loud** in the first line of your response — e.g. `RECORD → §3c`.
   A request often spans categories: **split it** and name each route. **Flag anything that
   does not cleanly map** to a category rather than forcing it into one — say so and ask.
   Routing *every* part (not just the first) is mandatory (D0064).

3. **If you cannot classify confidently, ask** — do not default to EXECUTE. Engine work
   (building the engine's own runtime/tooling) is not a separate route: route it by *what it
   changes* — schema/process ⇒ CHANGE §3a, otherwise ⇒ EXECUTE §3b — and it goes through a
   **sprint** like all substantive work; only trivial one-off edits (a typo, a single rename,
   one doc line) skip a sprint (D0064).

4. **Follow that route's rules** (CLAUDE.md §3a–§3f). The traps that most need this guard:
   - **CHANGE** never freelances: state change + rationale → **explicit human acceptance**
     → apply → validate green → record a `Decision` → commit `CR:`. `schema/core` is frozen.
   - **RECORD** of a `method=confirmation` result needs the human's **explicit** sign-off of
     that specific claim — never inferred from "go do the sign-offs" or from the work being
     done. Capture provenance: who, when (ISO-8601 `*At`), and `verifiedAtCommit`.
   - **VIEW** computes from authored facts + git and **never** stores or mutates.
   - **substantive work goes through a sprint** — no raw backlog execution; the no-sprint
     guard (`keel guard sprint-coverage`) enforces it.
   - **bulk migration** (rename/split/drop/add a field across many instances/files) → invoke the
     `migration` skill (D0067): committed transform script, dry-run+reconcile control totals,
     expand/migrate/contract green at every step, backfill-before-tighten, never fabricate provenance.

5. **Recurring-or-one-time check (D0040).** After classifying into a category, for
   EXECUTE and VIEW: ask *will this task recur?*

   | Determination | Action |
   |---------------|--------|
   | Recurring, no skill exists | CHANGE first: create/update a skill; then execute using it |
   | Recurring, skill exists    | Invoke the skill, execute using it |
   | Clearly one-time           | Execute directly |
   | Ambiguous                  | Ask the user before acting |

   Examples: "I'm on Windows" → permanent env fact → CLAUDE.md §6.
   "Generate HTML report" → recurring → status-report skill (create if absent, then use).
   "Rename this one local variable" → one-time → execute directly.

6. **Ceremony routing.** Sprint ceremonies are rigid EXECUTE sub-types — route to the
   matching skill before acting:

   | Ceremony phrase                           | Skill to invoke          |
   |-------------------------------------------|--------------------------|
   | "plan the sprint," "what's next," "size"  | `sprint-planning`        |
   | "standup," "status," "any blockers"       | `sprint-standup`         |
   | "sprint review," "velocity," "efficiency" | `sprint-review`          |
   | "close out," "accept the sprint"          | `sprint-closeout`        |
   | "retro," "retrospective," "improvements"  | `sprint-retro`           |
   | "refine this story," "INVEST check"       | `backlog-refinement`     |

7. **Continuous improvement capture.** When an improvement item surfaces mid-sprint
   (a process violation, a skill gap, a schema gap), record it immediately as an
   `Issue` in `.tracking/issues.sysml` rather than letting it slip to memory. It
   will be triaged at retro. Use `relatedTask` to point to the relevant backlog action.

## Why this exists

CLAUDE.md describes the discipline but cannot force the route-first step; a passive doc only
works if the step is actually performed each turn. This skill makes it an active, visible gate,
fired every turn by a `UserPromptSubmit` hook (D0064) so the route-first move is structural,
not vigilance.
