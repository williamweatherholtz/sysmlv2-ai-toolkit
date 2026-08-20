---
name: test-result
description: |
  Records a TestResult for a completed verification: captures outcome (pass/
  fail/inconclusive), judgedAt timestamp, judgedBy actor, and judgedAgainst
  commit SHA. Checks that the result is appended (not updated in place) and
  that the commit SHA is the one tested. Use when asked to "record a test
  result," "append a pass/fail," "log the outcome of this test," or "close
  out this verification." Do NOT use for designing tests (use test-design)
  or backlog refinement (use backlog-refinement).
metadata:
  version: 0.1.0
  domain: [test-result, verification, VerdictKind, provenance, SysMLv2]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# test-result

Appends a `TestResult` to the correct tracking file with full provenance:
`outcome`, `judgedAt` (ISO-8601 authored timestamp), `judgedBy` (actor id),
and `judgedAgainst` (commit SHA of the artifact under test).

## Expert Vocabulary Payload

**VerdictKind:** `pass`, `fail`, `inconclusive`.

**Provenance triad:** `judgedAt` (when — authored timestamp, not commit date),
`judgedBy` (who — actor id, never "AI" without a model id), `judgedAgainst`
(what revision — short SHA of the commit containing the artifact tested).

**Append-only invariant (decision 0001):** TestResults are NEVER edited or
deleted. A superseded pass is left in place; the new result appended after it
is the latest verdict. The query layer always takes the most-recent result.

## Anti-Pattern Watchlist

1. **Editing an existing TestResult** — always forbidden; append a new one.
2. **Missing commit SHA** — `judgedAgainst` must be the actual HEAD at test
   time, not a placeholder or a future commit.
3. **method=confirmation without explicit human sign-off** — record ONLY on
   the human's explicit attestation of that specific claim (decision 0016).
4. **Wall-clock as the validity signal** — `judgedAt` is display; `judgedAgainst`
   commit ancestry is the validity signal (decision 0005).

## Behavioral Instructions

1. Identify the `Test` being closed out (UUID, file, procedureText).
2. Confirm the outcome is supported by evidence (run output, human attestation,
   inspection artifact). For `method=confirmation`: hold until explicit sign-off.
3. Author the `TestResult` part in the same file as its `Test`, appended AFTER
   it.
4. Fill all three provenance fields: `judgedAt` = today ISO-8601, `judgedBy`
   = actor id, `judgedAgainst` = current HEAD short SHA.
5. Validate the file (validate_tracking.py); the result must parse clean before
   commit.

## Output Format

```sysml
part <TestId>R<N> : TestResult {
    :>> id = "<new-uuid>";
    :>> outcome = VerdictKind::pass;
    :>> judgedAt = "<YYYY-MM-DD>";
    :>> judgedBy = "<actorId>";
    :>> judgedAgainst = "<short-sha>";
}
```

Where `<N>` is the next sequential result number for this Test (R1, R2, …).
