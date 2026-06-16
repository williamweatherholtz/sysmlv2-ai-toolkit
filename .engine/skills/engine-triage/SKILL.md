---
name: engine-triage
description: |
  Routes EVERY substantive request in an engine repo through the work-tracking
  discipline before any action. Classifies the request into exactly one category
  — CHANGE / EXECUTE / RECORD / VIEW / BOOTSTRAP / ORIENT (CLAUDE.md §3) — states
  the category + route out loud, then follows that route's rules. Use at the start
  of any request that changes a workflow/schema, produces a tracked artifact,
  records a decision/test result/issue, asks for a computed answer, builds the
  engine's own tooling, or asks where things stand. This is the guard that stops
  actions slipping past the process (e.g. recording a confirmation that was never
  explicitly given). Do NOT use for purely conversational replies that change
  nothing. The categories' rules live in CLAUDE.md §3 (source of truth) — this
  skill is the always-on checklist that makes classify-first visible and mandatory.
metadata:
  version: 0.1.0
  domain: [process-discipline, request-routing, work-tracking, MBSE, SysMLv2]
  writePolicy: read-only
  engine: sysmlv2-ai-toolkit
---

# engine-triage — classify before you act

The engine tracks the *work of building things*. Every substantive request must be
routed through the discipline **before** acting. CLAUDE.md §3 is the source of truth;
this skill is the per-request checklist that makes the classify-first step visible and
mandatory, so nothing slips past silently.

## The checklist (do this first, every time)

1. **Classify** the request into exactly one category by *what it changes*:

   | Category   | The request…                                                  | Route |
   |------------|---------------------------------------------------------------|-------|
   | CHANGE     | changes a workflow / phase / gate / schema definition         | §3a   |
   | EXECUTE    | produces the active phase's typed artifact (tracked work)     | §3b   |
   | RECORD     | records ONE atomic fact (decision / test result / issue)      | §3c   |
   | VIEW       | asks for a computed answer (status, trace, stale set, a doc)  | §3d   |
   | BOOTSTRAP  | builds or fixes the engine's OWN runtime / tooling            | §3e   |
   | ORIENT     | asks where things stand / what is next                        | §3f   |

2. **State it out loud** in the first line of your response: e.g. `RECORD → §3c`.
   If the request spans categories, **split it** and name each route.

3. **If you cannot classify confidently, ask** — do not default to EXECUTE.
   Engine vs deliverable test: building the engine that *tracks* the work ⇒ BOOTSTRAP;
   producing what the work *delivers* ⇒ EXECUTE.

4. **Follow that route's rules** (CLAUDE.md §3a–§3f). The traps that most need this guard:
   - **CHANGE** never freelances: state change + rationale → **explicit human acceptance**
     → apply → validate green → record a `Decision` → commit `CR:`. `schema/core` is frozen.
   - **RECORD** of a `method=confirmation` result needs the human's **explicit** sign-off of
     that specific claim — never inferred from "go do the sign-offs" or from the work being
     done. Capture provenance: who, when (ISO-8601 `*At`), and `verifiedAtCommit`.
   - **VIEW** computes from authored facts + git and **never** stores or mutates.

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

6. **Continuous improvement capture.** When an improvement item surfaces mid-sprint
   (a process violation, a skill gap, a schema gap), record it immediately as an
   `Issue` in `.tracking/issues.sysml` rather than letting it slip to memory. It
   will be triaged at retro. Use `relatedTask` to point to the relevant backlog action.

## Why this exists

CLAUDE.md describes the discipline but cannot force the classify-first step; a passive
doc only works if the step is actually performed each turn. This skill makes it an active,
visible gate. (Bootstrap note: until the engine's runtime can intercept requests itself,
this checklist is enforced by you + CLAUDE.md; it graduates to runtime enforcement once the
write API exists. Register it in `skills/skills-registry.sysml` during the instance-file
migration.)
