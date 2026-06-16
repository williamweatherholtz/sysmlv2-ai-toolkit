---
name: sprint-closeout
description: |
  Guides the CloseOut phase of a sprint: verify DoD, confirm actualHours is set,
  record the confirmation gate TestResult with explicit human sign-off, and ensure
  the storyDoDR1 is in the backlog. Use at the closeOut gate, or when asked
  "close out this sprint," "sprint closeout," or "record sprint done."
metadata:
  version: 0.1.0
  domain: [agile, sprint-closeout, confirmation, DoD, SysMLv2]
  writePolicy: direct
  engine: sysmlv2-ai-toolkit
---

# sprint-closeout

Covers the CloseOut phase of the Delivery workflow. The defining constraint: the
closeOut gate uses `method = confirmation` — it REQUIRES an explicit human attestation.
Never record a confirmation TestResult until the human has said "yes," "accepted,"
or equivalent for that specific sprint's DoD claim.

## Behavioral Instructions

1. **Verify DoD is complete.** Check that:
   - The sprint story's DoDR1 TestResult is recorded with `outcome = pass`.
   - The DoDR1 is appended to the backlog (`DeliveryRun`).
   - All phase gate TestResults (refine/standup/implement/review) are recorded.
   - `validate_tracking.py` and `validate_instances.py` exit green.

2. **Confirm `actualHours` is set.** If not already recorded, ask for it now:
   *"How many wall-clock hours did this sprint take?"*
   Record `actualHours` on the sprint Story before proceeding.

3. **Present the DoD claim for human confirmation.** State explicitly:
   > "Sprint N DoD: [DoD procedureText]. Do you accept this sprint as complete?"
   
   Wait for the human's explicit "yes" / "accepted" / equivalent. **Never infer
   acceptance** from the work being done, from a general "continue" instruction,
   or from your own judgment. This is a `method=confirmation` verification — the
   human's word IS the evidence.

4. **Record the closeOut gate TestResult** (method = confirmation, outcome = pass)
   only after receiving explicit confirmation. Include:
   - `judgedAt`: current ISO-8601 date
   - `judgedBy`: wweatherholtz (human who confirmed)
   - `judgedAgainst`: current HEAD commit SHA

5. **Run `validate_tracking.py`** to confirm the file is still clean after adding
   the TestResult.

6. **Commit** with standard sprint commit message. The post-commit hook will push.

## D0019 Batch Confirmation (when applicable)

When multiple sprints are closed in one session, confirmation gates may be batched
per D0019: present them together at a natural pause and record all on a single "yes."
The batch must be presented explicitly — do not silently accumulate without surfacing.

## Anti-Patterns

- **Recording confirmation without explicit sign-off** — hardest violation to catch.
  If you are tempted to write "judgedBy = wweatherholtz" before the human has spoken,
  that is the violation. Stop. Ask first.
- **Missing `actualHours`** — closeOut is the last point to capture this. If it is
  missing, the sprint contributes no data to efficiency metrics.
- **Committing before validation** — always run `validate_tracking.py` after adding
  the TestResult and before `git commit`.
- **CloseOut gate before review gate** — gates run in order: refine→standup→implement→
  review→closeOut→retro. Never skip a phase gate.

## Output Format

```
CLOSEOUT CHECKLIST — Sprint N
[ ] DoDR1 in story: pass
[ ] DoDR1 in backlog: pass
[ ] All phase gates recorded: pass
[ ] validate_tracking: green
[ ] actualHours set: <N> h
[ ] Human confirmation received: yes | WAITING

Gate: PASS (confirmed by wweatherholtz, <date>) | PENDING CONFIRMATION
```

## Questions This Skill Answers

- "Close out this sprint"
- "Sprint closeout"
- "Record sprint done"
- "Accept the sprint"
- "All gates passed — close it"
