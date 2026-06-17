---
name: sprint-closeout
description: |
  Autonomously closes a sprint (D0049): verify DoD passes, record actualHours,
  record the closeOut gate (method=inspect, AI-judged) — NO human confirmation.
  Use at the closeOut gate, or when asked "close out this sprint," "sprint closeout,"
  or "record sprint done." Human acceptance is NOT here; it moved to the per-sitting
  sprint review (sprint-review skill).
metadata:
  version: 0.2.0
  domain: [agile, sprint-closeout, autonomous, DoD, SysMLv2]
  writePolicy: direct
  engine: sysmlv2-ai-toolkit
---

# sprint-closeout (autonomous)

Covers the CloseOut phase. Per D0049 closeOut is **autonomous** — it records once the
sprint DoD passes; it is no longer a human gate (`method=inspect`, AI-judged). The
human's acceptance happens once per sitting at the sprint review, not here.

## Behavioral Instructions

1. **Verify DoD is complete:**
   - The sprint story's `DoDR1` TestResult is recorded `outcome = pass`.
   - The `DoDR1` is appended to the backlog (`DeliveryRun`/`NextWork`).
   - All earlier phase gates (refine/standup/implement/review) are recorded — the
     ceremony-ordering guard (D0047) enforces no out-of-order closeOut.
   - The `.tracking` validator (`sysmlv2 validate .`, D0048) is green.
2. **Record `actualHours`** on the sprint Story if known (it feeds efficiency metrics;
   if genuinely unknown at closeOut, leave unset rather than guessing).
3. **Record the closeOut gate** TestResult: `method = inspect`, `outcome = pass`,
   `judgedBy` = the AI actor, `judgedAt` = today, `judgedAgainst` = HEAD. No human
   confirmation (D0049) — do NOT write `judgedBy = wweatherholtz` and do NOT pause to
   ask "do you accept?"
4. **Hand off to sprint-retro** (autonomous) to identify avoidable issues + create items.
5. **Validate + commit** `CR:`; the post-commit hook pushes.

## Anti-Patterns

- **Pausing for human sign-off** — closeOut is autonomous now (D0049). The human gate
  is the per-sitting review. Don't block the sprint on a confirmation.
- **Closing out of order** — closeOut requires the earlier gates recorded (the guard
  enforces it). refine→standup→implement→review→closeOut→retro.
- **Committing before validation** — run `sysmlv2 validate .` after adding the result.
- **Skipping the retro hand-off** — closeOut is followed by the autonomous retro.

## Output Format

```
CLOSEOUT — Sprint N (autonomous)
[ ] DoDR1 in story: pass
[ ] DoDR1 in backlog: pass
[ ] refine/standup/implement/review recorded: pass
[ ] sysmlv2 validate .: green
[ ] actualHours: <N> h | unset
Gate: PASS (inspect, AI, <date>)  → hand off to sprint-retro
```

## Questions This Skill Answers

- "Close out this sprint" / "Sprint closeout" / "Record sprint done"
- "All gates passed — close it"
