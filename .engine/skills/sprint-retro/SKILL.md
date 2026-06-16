---
name: sprint-retro
description: |
  Guides the Retro phase of a sprint: consume improvement items from the sprint-review
  transcript analysis, classify each by remediation type, dispatch accepted items to
  the right process (skill update / Decision / CLAUDE.md / backlog), and record the
  retro gate confirmation. Use at the retro gate, or when asked "retrospective,"
  "retro," "what should we improve," or "process improvement."
metadata:
  version: 0.1.0
  domain: [agile, retrospective, process-improvement, continuous-improvement, SysMLv2]
  writePolicy: direct
  engine: sysmlv2-ai-toolkit
---

# sprint-retro

Covers the Retro phase of the Delivery workflow — the last ceremony of each sprint.
Its job: convert improvement findings from the sprint-review transcript scan into
**concrete, tracked process changes** dispatched to the right place, then close the
sprint permanently.

## The Three Retro Questions

1. **What went well?** — identify and reinforce practices worth keeping.
2. **What didn't go well?** — improvement items from sprint-review Phase 3.
3. **What will we change?** — accepted improvements dispatched to process.

## Behavioral Instructions

1. **Load improvement items** from the sprint-review output (Phase 3 transcript scan).
   If sprint-review was not yet run, run it first — do not retro without findings.

2. **Triage each item** by remediation type and priority:

   | Type              | Dispatch to                                       | Condition                     |
   |-------------------|---------------------------------------------------|-------------------------------|
   | `skill-update`    | Edit the named skill's SKILL.md                   | Always — skills are process   |
   | `claude-md-change`| Edit CLAUDE.md at the referenced section          | Accept if structural          |
   | `decision`        | New Decision file in `.engine/decisions/`         | Capture even if no action     |
   | `backlog-item`    | New `action` in backlog's `DeliveryRun`           | If tooling/automation needed  |
   | `retro-note`      | Log but do not act; revisit next retro if recurs  | One-off incidents only        |

3. **Apply accepted skill-updates** inline during the retro (they are process/CHANGE
   items — get explicit human acceptance per §3a, then apply, validate, commit `CR:`).

4. **Record accepted decisions** as new Decision files immediately — not "later."
   The critique finding that prompted D0038+ was that CRs were missing their Decision
   records for ~11 consecutive sprints. Do not let that recur.

5. **Add accepted backlog items** to `.tracking/backlog.sysml` DeliveryRun action def.

6. **Validate green** after any skill/schema/process changes.

7. **Confirm process improvements with the human** before applying. Retro changes are
   CHANGE-category work (§3a) — they need explicit acceptance.

8. **Record the retro gate TestResult** (method = confirmation) with explicit human
   sign-off. After applying all accepted improvements, ask:
   > "Sprint N retro complete. Process improvements recorded/applied: [list].
   >  Do you accept the retro as complete?"

## Continuous Improvement Capture (between sprints)

When improvement items surface mid-sprint (not just at retro), capture them as
`Issue` items in `.tracking/issues.sysml` with `relatedTask` pointing to the
relevant backlog action. This prevents them from being lost before the next retro.
Record as: `part issueXxx : Issue { :>> description = "..."; :>> discoveredInField = false; }`.

## Anti-Patterns

- **Retro without transcript review** — "nothing to improve" is almost never true.
  Sprint-review Phase 3 surfaces what conversation-level memory misses.
- **Improvement items as prose** — "we should be more careful" is not actionable.
  Every item must be typed (skill-update/decision/etc.) with a target and a specific fix.
- **Applying improvements without human acceptance** — retro changes are CHANGE-category.
  Present, accept, then apply. Not the other way around.
- **Silently discarding retro-notes** — log them. If the same note recurs three sprints
  in a row, it is no longer a one-off: escalate to a tracked improvement item.
- **Skipping the retro** — every sprint has a retro gate, even if the sprint was trivial.
  A trivial retro takes two minutes. Skipping it means improvements accumulate silently.

## Output Format

```yaml
sprint_retro: Sprint N
went_well:
  - "<observation>"
improvement_items_reviewed: <count>
accepted:
  - item: "<incident>"
    type: skill-update | claude-md-change | decision | backlog-item
    target: "<skill / section / file>"
    status: applied | recorded | queued
deferred:
  - item: "<incident>"
    reason: "<why deferred>"
retro_gate_result: recorded | PENDING_CONFIRMATION
```

## Questions This Skill Answers

- "Retrospective" / "retro"
- "What should we improve?"
- "Process improvement"
- "Apply the improvement items"
- "Close the sprint"
- "Were there any process violations this sprint?"
