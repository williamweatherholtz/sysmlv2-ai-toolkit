---
name: test-design
description: |
  Designs test cases from a DoD procedureText: parses the text into atomic,
  objective pass/fail criteria; maps each to the right VerificationMethod
  (test, analysis, inspection, demonstration, confirmation); generates
  Gherkin scenarios for automatable criteria. Use when asked to "design
  tests for this DoD," "what tests does this story need," "write BDD
  scenarios," or "make this DoD testable." Do NOT use for requirement wording
  (use requirement-quality) or backlog refinement (use backlog-refinement).
metadata:
  version: 0.1.0
  domain: [test-design, BDD, Gherkin, VerificationMethod, DoD, SysMLv2]
  writePolicy: pr-only
  engine: keel-ai-toolkit
---

# test-design

Turns a Definition-of-Done `procedureText` into atomic `Test` items with the
correct `method`, objective pass/fail criteria, and Gherkin scenarios for
automatable cases. Links each `Test` to the parent Story/DoD via `verify`.

## Expert Vocabulary Payload

**VerificationMethod:** test (automated assertion), analysis (analytical
derivation), inspection (human review of an artifact), demonstration
(live show), confirmation (explicit human attestation).

**BDD:** Given/When/Then, one behavior per scenario, declarative steps,
parameterized examples (Scenario Outline), feature-file structure.

**Test anatomy:** precondition, stimulus, observable, expected outcome
(pass threshold), automatable flag.

## Anti-Pattern Watchlist

1. **Compound criterion** — "does X and Y and Z in one test" → split into
   separate `Test` items; one criterion per test.
2. **Non-objective Then** — "looks correct", "is fast" → add measurable
   threshold ("< 200 ms", "HTTP 200", "diff empty").
3. **Wrong method** — using `test` for a human judgment call → use
   `inspection` or `confirmation`; reserve `test` for machine-verifiable.
4. **Missing negative case** — only happy path → always add at least one
   failure/rejection scenario for non-trivial behavior.

## Behavioral Instructions

1. Parse the `procedureText` into individual verifiable claims (split on
   semicolons, bullet points, "AND"/"OR").
2. For each claim: choose `method`, write a pass/fail criterion, determine
   if automatable (Gherkin-ready), assign a new UUID.
3. Author Gherkin scenarios for `method = test` criteria.
4. Emit each claim as a `Test` item with `verify` edge to the Story/DoD.
5. Flag any claim that is ambiguous or non-objective; surface for human
   clarification before authoring.

## Output Format

```yaml
tests:
  - id: <uuid>
    method: test|analysis|inspection|demonstration|confirmation
    criterion: "<objective, measurable pass condition>"
    gherkin: |
      Scenario: <behavior>
        Given <precondition>
        When <action>
        Then <observable outcome with threshold>
    verifies: <story or dod id>
    automatable: true|false
ambiguous_claims:
  - "<original text that needs clarification>"
```
