---
name: sprint-standup
description: |
  Guides the Standup phase of a sprint: orient on current status, surface blockers,
  confirm the active phase is the right one, and state what will be done next.
  Use at the standup gate, at the start of any work session, or when asked
  "standup," "what's the status," "where are we," or "any blockers?"
metadata:
  version: 0.1.0
  domain: [agile, standup, orient, blockers, SysMLv2]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# sprint-standup

Covers the Standup phase of the Delivery workflow. This ceremony is intentionally
lightweight — the goal is a 2-minute orient, not a planning session.

## Three Questions (classic standup)

1. **What was done** since the last standup (or since sprint start)?
2. **What is next** — the immediate next action within the active phase?
3. **Any blockers** — dependencies, missing information, gating failures?

## Behavioral Instructions

1. **Run orient.** Execute `keel orient [root]` (or query orient). Report:
   - Active sprint + current phase
   - Done count / suspect count
   - Ready frontier items

2. **Confirm phase alignment.** The standup gate is between Refine and Implement.
   If the cursor is in a different phase, surface it — do not silently jump phases.
   Switching phases is a recorded Decision, not a standup action.

3. **Check for blockers.** Scan the backlog for:
   - Items in the ready frontier that have an unresolved `dependency`
   - Any TestResult with `outcome = fail` in the current sprint
   - Any suspect items (judgedAgainst commit ≠ current HEAD)

4. **State next action.** One concrete step within the active Implement phase.
   Not a plan, not a proposal — the NEXT thing you will do.

5. **Record the standup gate TestResult** (method = inspect, outcome = pass) once
   no blockers are active. If there are blockers, document them and do NOT pass the gate.

## Anti-Patterns

- **Standup becoming planning** — if the answer to "what's next" requires more than
  one sentence, stop and use sprint-planning instead.
- **Passing the gate with unresolved blockers** — blockers must be explicitly resolved
  or escalated before the gate passes.
- **Skipping orient** — never answer standup questions from memory. Run orient first.
- **Phase drift** — if the team is in Implement but the standup gate hasn't been passed,
  record the standup gate retroactively before proceeding.

## Output Format

```
STANDUP — Sprint N (<sprint name>)
Phase: <active phase>
Done: <N> / Suspect: <N>

Yesterday/last session: <one sentence>
Next: <one concrete action>
Blockers: none | <description>

Gate: PASS | BLOCKED
```

## Questions This Skill Answers

- "Standup"
- "What's the status?"
- "Where are we in the sprint?"
- "Any blockers?"
- "What are we doing today?"
- "Orient me"
