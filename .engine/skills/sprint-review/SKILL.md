---
name: sprint-review
description: |
  Guides the sprint Review phase: verify DoD criteria pass, record actualHours,
  compute velocity/efficiency/accuracy metrics, and perform a transcript review
  that converts errors/inefficiencies/bad-directions into process-improvement
  items routed to retro. Use at the review gate, or when asked "sprint review,"
  "what's our velocity," "how accurate were our estimates," or "review this sprint."
metadata:
  version: 0.1.0
  domain: [agile, sprint-review, metrics, velocity, efficiency, process-improvement, SysMLv2]
  writePolicy: direct
  engine: sysmlv2-ai-toolkit
---

# sprint-review

Covers the Review phase of the Delivery workflow. Two distinct outputs:
1. **Metrics snapshot** — velocity, efficiency, accuracy for this sprint + trailing trend.
2. **Improvement queue** — findings from transcript review, dispatched to retro.

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

## Phase 4 — Record the Review Gate

Once metrics are computed and improvement items are identified, record the review gate
TestResult (method = inspect). The `procedureText` should reference the decisions reviewed
for consistency.

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
