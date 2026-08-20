---
name: sprint-retro
description: |
  Autonomously retrospects the just-finished sprint (D0049): scans the sprint for
  AVOIDABLE issues — problems a guard, skill, doc, or check could have prevented —
  and creates tracked Issue + backlog items for the fixes, then records the retro
  gate (method=analysis, AI-judged) with NO human input. Use at the retro gate, or
  when asked "retro," "what could we have avoided," or "what should we improve."
  The human does not gate the retro (that moved to the per-sitting sprint review).
metadata:
  version: 0.2.0
  domain: [agile, retrospective, avoidable-issues, continuous-improvement, autonomous, SysMLv2]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# sprint-retro (autonomous)

The last per-sprint ceremony. Runs WITHOUT human input (D0049). Its job: find what
was **avoidable** in the sprint and turn each into a tracked, actionable item — so the
next sprint can't repeat it. Human acceptance is NOT sought here; it happens once per
sitting at the sprint review (sprint-review skill).

## What counts as an AVOIDABLE issue

A problem is *avoidable* if some durable artifact — a guard, validator, lint, skill
rule, doc line, or check — could have prevented it or caught it earlier. The test:
*"what one change would have stopped this from happening, or surfaced it immediately?"*

| Signal (from the sprint transcript + git + diffs) | Avoidable by …                    |
|---------------------------------------------------|-----------------------------------|
| A correction the human had to point out           | a guard/lint that fails on it     |
| A check done by hand that should be automatic      | wiring it into a validator/hook   |
| Acting before classifying / skipped ceremony step  | a guard (e.g. validate_ceremony)  |
| Re-did work after a wrong approach                 | a skill rule / clearer DoR        |
| A tool in the validation path that hung/leaked     | a design rule (e.g. kernel-free)  |
| Doc said X but reality was Y                        | doc-sync in the same commit       |
| Estimate off by >2× vs guideline                   | record for accuracy calibration   |

Not every annoyance is avoidable — a genuine one-off (an upstream outage, a
truly-novel discovery) is a `retro-note`, not an item. Apply judgment; do not
manufacture items.

## Behavioral Instructions (autonomous)

1. **Scan the just-finished sprint** — its conversation, commits, and diffs — for the
   signals above. (If a sprint-review Phase-3 scan already produced improvement_items,
   start from those.)
2. **For each finding, decide avoidable vs one-off.** One-offs → a brief `retro-note`
   in the retro gate result text. Avoidable → an item (next step).
3. **Create a tracked item for each avoidable issue:**
   - An `Issue` in `.tracking/issues.sysml` (`part issueNNN : Issue { … relatedTask = "…" }`)
     describing the incident + the preventing change.
   - If it needs tooling/automation, a backlog `action` + `…DoD` in the relevant
     `action def` (the change to make).
   - If a guard is cheap and obvious, building it now is in-scope (per D0047 the
     correction must become a permanent guard, not a note).
4. **Record the retro gate** TestResult via `keel append-gate-result --file <delivery file>
   --gate <sprintRetroGate> --sha <HEAD> --judged-by <AI actor> --judged-at <today>` (auto-UUID,
   append-only `{gate}R{n}`). The gate is `method = analysis`, AI-judged: NO human confirmation (D0049).
5. **Do NOT ask the human to accept.** Acceptance is the per-sitting sprint review.
   Schema/process changes still validate green + commit `CR:`; doc-sync rides along.

## Anti-Patterns

- **Waiting for human sign-off** — the retro is autonomous now (D0049). Don't block.
- **Notes instead of items** — "we should be careful" is not trackable. Every
  avoidable finding becomes an Issue/backlog item with a concrete preventing change.
- **Manufacturing items** — a true one-off is a retro-note; don't inflate the count.
- **Patching without guarding** — if the fix is a guard, build/track it (D0047), don't
  just describe it.

## Output Format

```yaml
sprint_retro: Sprint N   (autonomous)
avoidable_issues:
  - incident: "<one sentence>"
    preventing_change: "<guard / skill rule / doc / backlog item>"
    tracked_as: "issueNNN" | "backlog:<action>" | "applied:<file>"
one_off_notes:
  - "<retro-note>"
retro_gate_result: recorded (method=analysis, AI)
```

## Questions This Skill Answers

- "Run the retro" / "What could we have avoided this sprint?"
- "Make items for the avoidable issues"
- "Close the sprint's retro"
