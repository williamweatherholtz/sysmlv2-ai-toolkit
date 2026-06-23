---
name: backlog-refinement
description: |
  Takes a work item (Story/Epic/Task) through INVEST and the Definition-of-Ready
  gate, and turns its acceptance criteria into atomic Gherkin-style Test items
  linked to the work item by `verify`. Use during Backlog Refinement or the
  Standup gate, or when asked to "refine this story," "is this ready to work?",
  "write acceptance criteria," "make these Gherkin scenarios," "split this
  story," "is this story too big?", or "get this to ready." Also triggers on
  INVEST checks, story-slicing, and DoR gating. Do NOT use for requirement
  wording quality (use requirement-quality), STPA (use stpa), or committing /
  PRs (use repo-push).
metadata:
  version: 0.2.0
  domain: [agile, INVEST, BDD, Gherkin, definition-of-ready, SysMLv2]
  writePolicy: pr-only
  engine: keel-ai-toolkit
---

# backlog-refinement

Runs the engine's Refinement process and Definition-of-Ready gate on `WorkItem`
items (`Story`, `Epic`, `Task`) from `Engine::Core`. Its defining move: acceptance
criteria are NOT a text blob — each becomes an atomic `Test` item linked by
`verify` (engine decision 0004). Authoring is pr-only.

## Expert Vocabulary Payload

**Story quality (INVEST, Wake):** Independent, Negotiable, Valuable, Estimable,
Small, Testable; story-slicing / vertical slice; spike (for unknowns).

**BDD / Gherkin (North; Cucumber):** Given/When/Then, scenario, Background,
Scenario Outline, declarative vs imperative steps, one-behavior-per-scenario.

**Flow gates:** Definition of Ready, Definition of Done, acceptance criteria
(rule-oriented vs scenario-oriented), story points (Fibonacci only — see scale
below), epic→story→task hierarchy.

**Fibonacci scale (D0038):** Only these values are valid for `estimatedPoints`:

| Points | Guideline wall-clock | Typical scope                              |
|--------|----------------------|--------------------------------------------|
| 1      | < 2 h                | Tiny: decision record, config, doc update  |
| 2      | 2–5 h                | Small: a tool, a validator, minor code     |
| 3      | 5–10 h               | Medium: a module, schema change            |
| 5      | ~1 day               | Large: a subsystem with tests              |
| 8      | 2–3 days             | Extra-large: major cross-cutting feature   |
| 13+    | > 3 days             | Epic-sized — **split before sprint start** |

Non-Fibonacci values (4, 6, 7, 9, 10, 11, 12) are an anti-pattern. When in
doubt between two sizes, take the larger. For final sizing + DoR gate at
sprint start, use the **sprint-planning** skill.

**Engine binding:** `Story.estimatedPoints` (Fibonacci integer), `Story.actualHours`
(Real — wall-clock hours recorded at closeOut per D0038), atomic `Test` (method =
test/demonstration), `verify` edge (Test → Story), `dependency` (blocked-by),
`currentState` (modular workflow), event-driven Sprint.

## Anti-Pattern Watchlist

1. **Text-blob acceptance criteria** — Detection: criteria live in a prose field
   on the Story instead of as atomic `Test` items. Resolution: create one `Test`
   per criterion, linked by `verify`; the prose blob is deleted (decision 0004).
2. **Unverifiable "Then"** — Detection: a Then step says "works well", "is fast",
   "looks good". Resolution: every Then is objectively pass/fail with a value +
   unit ("within 60 s", "HTTP 200", "row count = 3").
3. **Horizontal slice** — Detection: a story is a technical layer ("add a DB
   column", "build the API") with no user-visible value. Resolution: fails
   INVEST-V; re-slice into a vertical slice that delivers observable value, or
   demote to a `Task` under a value-bearing Story.
4. **Epic masquerading as a story** — Detection: cannot fit in one sprint /
   spans many behaviors. Resolution: fails INVEST-S; split into multiple stories
   or promote to `Epic` with child stories.
5. **Hidden dependency** — Detection: story silently needs another unfinished
   story. Resolution: fails INVEST-I; add an explicit `dependency` edge or
   re-order; don't mark ready while blocked.
6. **Unsized "ready"** — Detection: `estimatedPoints` empty but state set to
   ready. Resolution: DoR requires a size; if unsizable, spawn a `research`
   (spike) work item first.
7. **Non-Fibonacci estimate** — Detection: `estimatedPoints` set to 4, 6, 7, 9,
   etc. Resolution: round up to the next Fibonacci number. Document the rationale.
7. **Imperative over-specified scenario** — Detection: Gherkin steps dictate UI
   mechanics ("click the blue button at x,y"). Resolution: write declarative
   steps about behavior/outcome, not implementation.

## Behavioral Instructions

1. **Scan for the anti-patterns above first**, especially #1 (text-blob
   criteria) — it's the engine's hard rule.
2. **Load the work item.** Read `title`, `kind`, description, existing criteria,
   `estimatedPoints`, and any `dependency` edges.
3. **Run the six INVEST checks** as pass/fail:
   - **I** independent — no hidden ordering dependency (else add a `dependency`).
   - **N** negotiable — states what/why, not prescriptive how.
   - **V** valuable — names a role and a benefit; user-visible.
   - **E** estimable — sizable now (else propose a `research` spike).
   - **S** small — fits one sprint (else split / promote to `Epic`).
   - **T** testable — has objective acceptance criteria.
4. **IF INVEST-S fails:** propose a slice into child stories (vertical slices) or
   restructure Epic→Story; stop and surface the split rather than forcing ready.
5. **Author acceptance criteria as atomic `Test` items.** For each distinct
   behavior, write one Gherkin scenario (Given/When/Then), create a `Test`
   (method = `test` if automatable, else `demonstration`), and link it
   `verify`→ the Story. WHY: atomic, independently queryable, and feeds the
   computed coverage/satisfaction views.
6. **Verify the story format** for `Story` kind: `As a <role>, I want <goal>, so
   that <benefit>.` Fill gaps by asking, not inventing the benefit.
7. **Run the Definition-of-Ready gate** (see Output Format checklist). IF all
   pass: propose `currentState = ready`. ELSE: list the misses; if a gate needs
   judgment that can't be resolved, recommend invoking `grill-me`.
8. **Emit** via PR (writePolicy = pr-only). Do not author computed fields
   (coverage/satisfaction recompute from the Tests + results).

## Output Format

```yaml
item: <work item id>
invest:
  independent: pass|fail
  negotiable: pass|fail
  valuable: pass|fail
  estimable: pass|fail
  small: pass|fail
  testable: pass|fail
tests_created:                 # one per acceptance criterion
  - id: <test id>
    method: test|demonstration
    gherkin: |
      Scenario: <behavior>
        Given <context>
        When <action>
        Then <observable, measurable outcome>
    verifies: <work item id>
dor: PASS | NOT_READY
dor_misses:
  - "estimatedPoints not set"
split_proposal:                # only if INVEST-S failed
  - "Story A: <vertical slice>"
  - "Story B: <vertical slice>"
recommend_state: ready | stay  # proposed currentState transition
```

## Examples

### BAD vs GOOD acceptance criterion
- **BAD (text blob on the story):** "Acceptance: password reset works and is
  secure and fast." → anti-patterns #1, #2.
- **GOOD (atomic Test, verify-linked):**
  ```gherkin
  Scenario: Registered user requests a reset link
    Given a registered user with email "ana@example.com"
    When the user submits "ana@example.com" on the reset page
    Then a reset email is sent within 60 seconds
    And the email contains a single-use link valid for 24 hours
  ```
  → `Test T-12 (method=test) verify Story S-4`.

### BAD vs GOOD story (INVEST)
- **BAD:** "Add a users table column for reset tokens." → horizontal slice,
  no user value (#3, fails V). Demote to a Task under a value story.
- **GOOD:** "As a user who forgot my password, I want to reset it via an emailed
  link, so that I can regain access without contacting support." → passes V, N;
  size it, attach the Test above, check independence.

### The representative case — split decision
Story: "As an operator I want a full reporting dashboard." → fails INVEST-S
(epic-sized, many behaviors). **Action:** promote to `Epic`; propose child
stories each a vertical slice ("view daily total", "filter by date range",
"export CSV"), each independently valuable, sizable, and testable. Do not mark
the epic ready.

## Questions This Skill Answers

- "Refine this story"
- "Is this story ready to work on?"
- "Write acceptance criteria for this"
- "Turn these criteria into Gherkin scenarios"
- "Is this story too big? / split this story"
- "Does this story pass INVEST?"
- "Run the Definition of Ready on STORY-9"
- "Should this be an epic?"
- "Make these acceptance criteria testable"
- "What's blocking this from being ready?"
