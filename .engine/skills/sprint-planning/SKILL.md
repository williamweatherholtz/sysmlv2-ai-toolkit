---
name: sprint-planning
description: |
  Guides sprint planning: orient on the ready frontier, select backlog items,
  apply Fibonacci sizing to set estimatedPoints, and pass the Definition of Ready
  before work begins. Use when asked to "plan the next sprint," "what should we
  work on," "size this story," "how many points is X," or at the start of any
  sprint's refine gate.
metadata:
  version: 0.1.0
  domain: [agile, sprint-planning, Fibonacci, estimation, definition-of-ready, SysMLv2]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# sprint-planning

Covers the Refine phase of the Delivery workflow — selecting scope, applying
Fibonacci story-point sizing, and passing the DoR gate. Output: a sprint Story
with `estimatedPoints` set and the refine gate TestResult recorded.

## Expert Vocabulary Payload

**Fibonacci scale (story points):** 1 / 2 / 3 / 5 / 8 / 13 / 21. No intermediate
values. The next size up is the only alternative when a story doesn't fit the
current size. Non-Fibonacci values are an anti-pattern.

**Guideline hours (D0038):** calibrated from project history — these are starting
anchors, not hard bounds. Actual efficiency builds the baseline over time.

| Points | Guideline wall-clock | Description                                    |
|--------|----------------------|------------------------------------------------|
| 1      | < 2 h                | Tiny: pure decision record, doc update, config |
| 2      | 2–5 h                | Small: a tool, a validator, minor code change  |
| 3      | 5–10 h               | Medium: a module, schema change + migration    |
| 5      | ~1 day               | Large: a subsystem with tests                  |
| 8      | 2–3 days             | Extra-large: major feature, cross-cutting      |
| 13+    | > 3 days             | Epic-sized — **split first**                   |

**Velocity budget:** for focus, prefer ≤ 5 pts per sprint for a solo+AI team.
Sprints over 8 pts are a code smell — split into two.

**Definition of Ready (DoR):** estimatedPoints set; scope clear; no blocking
dependency unresolved; DoD criteria exist or are authored during refine.

## Behavioral Instructions

1. **Orient first.** Run `keel whats-next [root]` (or query orient) to get the
   current ready frontier. Identify which items are unblocked and available.

2. **Select scope.** Prefer a single focused story. Multiple items in one sprint
   are allowed but must total ≤ 5 pts for the sprint to remain sharp. Surface
   the trade-off if selecting more.

3. **Apply Fibonacci sizing.**
   - Read the story's DoD / acceptance criteria.
   - Match against the guideline hours table above.
   - When uncertain between two sizes, take the **larger** (conservative). A
     sprint that finishes early is better than one that runs over.
   - Never use a non-Fibonacci number. If between 2 and 3, use 3.
   - If you can't size it (unknowns too large), propose a 1-pt **research spike**
     first. Unestimable ≠ 1 pt.

4. **Set `estimatedPoints`** on the Sprint Story before any implementation begins.
   Record it in the sprint delivery file (`.tracking/delivery/sprintN_*.sysml`).

4b. **Charter the Story (D0068/D0069) — at prep, not hoped.** In the delivery file, author a
   `#CharteredBy` edge from the sprint Story to the originating backlog item / Decision / Need /
   Requirement that chartered the work: `#CharteredBy dependency from <story> to <dNNNN|need|req>;`
   (import `EngineRelationships::*`). This is the work→origin lineage the governing-process VERSION
   is computed from (pglViews). The charter guard (`keel guard charter`) FAILS any newly-added
   sprint whose Story has no `#CharteredBy` edge — so set it now.

5. **Run the DoR checklist:**
   - estimatedPoints set ✓
   - `#CharteredBy` edge set on the Story (charter lineage, D0068) ✓
   - Scope / acceptance criteria unambiguous ✓
   - No blocking dependency or dependency is explicitly listed ✓
   - DoD tests exist or will be authored in refine ✓

6. **Record the refine gate TestResult** (method = inspect) once DoR passes.

7. **State the sprint goal:** one sentence — what value will be delivered and how
   it will be verified. This is authored in the sprint story title / procedureText.

## Anti-Patterns

- **Defaulting to 1 pt for everything** — only valid for tiny record-only sprints.
  If any code changes, validators run, or new tools are built, the estimate is ≥ 2.
- **Non-Fibonacci values** — "2.5 pts," "4 pts" are not allowed.
- **Starting without estimatedPoints** — always size before the standup gate.
- **Epic in a sprint** — 13+ pts means split now.

## Output Format

```yaml
sprint_goal: "<one sentence>"
selected_items:
  - id: <backlog item name>
    estimatedPoints: <Fibonacci integer>
    rationale: "<why this size>"
total_points: <sum>
dor:
  estimatedPoints: pass
  scope_clear: pass|fail
  unblocked: pass|fail
  dod_criteria: pass|fail
dor_verdict: PASS | NOT_READY
refine_gate_result: recorded | pending
```

## Questions This Skill Answers

- "Plan the next sprint"
- "What should we work on next?"
- "How many points is the baselineView story?"
- "Is this sprint sized correctly?"
- "Should we split this sprint?"
- "What's our sprint goal?"
- "Run the Definition of Ready"
