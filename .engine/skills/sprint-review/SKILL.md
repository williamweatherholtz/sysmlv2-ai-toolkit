---
name: sprint-review
description: |
  The per-SITTING human review (D0049) — the single human confirmation touchpoint.
  After a sitting (one or more sprints), summarize the sitting's content + metrics,
  run the transcript scan that feeds the autonomous retro, present it to the human,
  and record their explicit confirmation of the sitting. Use when asked "sprint
  review," "review the sitting," "what's our velocity," or at the end of a sitting.
metadata:
  version: 0.2.0
  domain: [agile, sprint-review, sitting, human-confirmation, metrics, process-improvement, SysMLv2]
  writePolicy: direct
  engine: sysmlv2-ai-toolkit
---

# sprint-review (per-sitting, human-confirmed)

The human touchpoint of the ceremony (D0049). Per-sprint closeOut + retro run
autonomously (sprint-closeout / sprint-retro skills); the human does NOT gate each
sprint. Instead, after a **sitting** (one continuous work session, ≥1 sprint), this
review presents the sitting's content and records the human's explicit confirmation.

Three outputs:
1. **Sitting summary** — the sprints completed this sitting + what shipped.
2. **Metrics snapshot** — velocity, efficiency, accuracy + trailing trend.
3. **Improvement queue** — transcript-scan findings (feeds the autonomous retro).
4. **Human confirmation** — the human accepts the sitting's content (the one gate).

## Expert Vocabulary Payload

**Velocity:** sum of `estimatedPoints` delivered per sprint; trailing 3-sprint average
gives a planning baseline.

**Efficiency** (D0038): `estimatedPoints / actualHours` for a sprint. Unitless ratio —
higher = more points per hour. Track the trailing average to see if the team is
improving or degrading. A sudden drop signals unexpected complexity or rework.

**Accuracy:** How close was the Fibonacci estimate to what the sprint actually cost?
Compare `actualHours` to the guideline range for `estimatedPoints` (see sprint-planning
skill). If 1 pt took 6 h (guideline < 2 h), the estimate was off by 3×. Note the
direction (over/under) for calibration at retro.

**Transcript review:** structured scan of the session conversation (or git log + commit
messages) for: errors taken, bad directions, unnecessary rework, missing context,
confusion points, repeated questions, workflow violations, anti-patterns from any skill.
Each finding becomes a **process-improvement item** classified by remediation type.

## Phase 1 — Verify DoD

1. Confirm all DoD TestResults for this sprint are recorded with `outcome = pass`.
2. Confirm the story's DoDR1 is present in the backlog.
3. If any gate is missing or failed, **stop** — the sprint is not reviewable until DoD passes.

## Phase 2 — Record Metrics

1. **Prompt for `actualHours`** if not yet set on the sprint Story. Ask: *"How many
   wall-clock hours did this sprint take?"* Do not proceed to metrics until you have it.
2. **Record `actualHours`** on the sprint Story in the delivery file.
3. **Compute sprint metrics:**

   | Metric       | Formula                              | This sprint | Trailing 3 avg |
   |--------------|--------------------------------------|-------------|----------------|
   | Velocity     | estimatedPoints                      | _           | _              |
   | Efficiency   | estimatedPoints / actualHours        | _           | _              |
   | Accuracy     | actualHours vs guideline range       | within / over / under | trend |

4. **Build the history table** from all past sprints with `estimatedPoints` + `actualHours`
   set. Display as a running log.

## Phase 3 — Transcript Review (process-improvement scan)

Scan the sprint's session conversation and git log for the following signals:

| Signal type             | Detection cue                                                       |
|-------------------------|---------------------------------------------------------------------|
| Wrong route taken       | Corrected direction ("no, not that"), re-do after wrong approach    |
| Skill gap               | Repeated question, missing context, skill had no guidance for it    |
| Workflow violation      | Acted before classifying; recorded confirmation without sign-off    |
| Anti-pattern triggered  | Any item from a skill's anti-pattern watchlist was hit              |
| Unnecessary rework      | File edited > once for the same logical change; validator run twice |
| Missing guard           | A check that SHOULD have been automatic was done manually           |
| Schema / process gap    | Had to improvise because no rule existed                            |
| Documentation drift     | Code/process changed but a doc wasn't updated                       |

For each finding:

1. **Describe** the incident in one sentence.
2. **Classify** the remediation type:
   - `skill-update` — a skill's behavioral instructions need a new rule or anti-pattern
   - `claude-md-change` — CLAUDE.md §N needs an addition or clarification
   - `decision` — an architectural choice needs to be recorded (it's now implicit)
   - `backlog-item` — new tooling/automation needed
   - `retro-note` — observe and discuss; no immediate process change
3. **Propose the fix** — exact skill section, CLAUDE.md paragraph, or new backlog item.

Format as:

```yaml
improvement_items:
  - incident: "<one sentence>"
    type: skill-update | claude-md-change | decision | backlog-item | retro-note
    target: "<skill name / CLAUDE.md §N / decision title / backlog action name>"
    proposed_fix: "<what to add, change, or record>"
    priority: high | medium | low
```

4. **Route high-priority items to retro** for immediate action. Medium/low go into the
   backlog or are held for the next retro.

## Phase 4 — Human confirmation of the sitting (the one gate, D0049)

This is the single human touchpoint. Per-sprint review gates may be recorded
autonomously (method=inspect, AI) as a technical check; the HUMAN gate is here, at
sitting granularity:

1. **Present the sitting** — list its sprints, what each shipped, the metrics, and the
   improvement items the retro will/​did dispatch.
2. **Ask for explicit confirmation:**
   > "Sitting review: sprints [N..M] — [summary]. Do you accept this sitting's content?"
   This is a `method=confirmation` attestation (D0016): never inferred, always the
   human's explicit word. Confirmations across the sitting's sprints may be batched
   (D0019) onto a single "yes."
3. **Record** the per-sitting review confirmation (`judgedBy = wweatherholtz`) once given.
   closeOut + retro for the sitting's sprints are already autonomous — they do not wait
   on this; this is sitting acceptance, not a per-sprint blocker.

## Anti-Patterns

- **Skipping actualHours** — never record the review gate without actualHours on the Story.
- **No transcript review** — metrics alone miss process drift. The transcript scan is
  mandatory, not optional.
- **Improvement items as prose blobs** — each finding must be a typed, actionable item
  (improvement_items list above), not a paragraph. Blobs can't be tracked or resolved.
- **Accepting every improvement item** — not every finding warrants a process change.
  Apply judgment: if a finding is a one-off, log as retro-note; if it will recur, fix.

## Questions This Skill Answers

- "Sprint review"
- "What's our velocity?"
- "How efficient were we this sprint?"
- "How accurate were our estimates?"
- "What should we improve?"
- "Review the sprint transcript for issues"
- "Did we follow the process correctly?"
