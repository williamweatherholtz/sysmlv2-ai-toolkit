---
name: traceability-audit
description: |
  Audits the traceability chain from Needs → Requirements → Tests → TestResults
  for a given scope: finds gaps (no verify edge, no TestResult), suspects
  (result predates last material change), and orphans (element with no upstream
  derivation). Use when asked to "audit traceability," "check coverage," "find
  unverified requirements," "what's missing a test," or "run a trace audit."
  Do NOT use for designing tests (use test-design) or recording results (use
  test-result).
metadata:
  version: 0.1.0
  domain: [traceability, coverage, verification, suspicion, SysMLv2]
  writePolicy: readOnly
  engine: keel-ai-toolkit
---

# traceability-audit

Audits the full traceability chain for a scope, identifying coverage gaps,
suspect items, and orphans. Read-only: emits findings for human action; does
not author or mutate the model.

## Expert Vocabulary Payload

**Traceability chain:** Need → (satisfy) → Requirement → (verify) → Test →
(TestResult). Each link must exist; breaks are coverage gaps.

**Coverage states (decision 0005):** covered (latest pass postdates last
material change), suspect (pass predates a material change), failing (latest
result is fail), uncovered (no verifying Test).

**Orphan:** an element with no upstream derivation edge (no `:>`, `satisfy`,
or `allocate` pointing to it from any requirement or need).

**Material change:** a change to an element's semantic fields, not cosmetic
edits. Suspicion is computed from git ancestry, not wall-clock.

## Anti-Pattern Watchlist

1. **Confusing missing-test with failing-test** — missing = uncovered (no
   Test exists); failing = a Test exists but its latest result is fail.
2. **Wall-clock suspicion** — judging suspect by date comparison instead of
   git ancestry; decisions 0005, 0013.
3. **Conflating orphan with uncovered** — an orphan has no upstream Need/
   Requirement; uncovered has a Requirement but no Test. Distinct findings.

## Behavioral Instructions

1. For each Requirement in scope: check that ≥1 `verify` edge exists from
   a Test. If not: report as **uncovered**.
2. For each verifying Test: check for ≥1 TestResult with `outcome = pass`
   and `judgedAgainst` ≥ last material-change commit. If not: report as
   **suspect** or **failing**.
3. For each element: check for at least one upstream derivation edge. If
   missing: report as **orphan**.
4. Summarize by severity: failing > suspect > uncovered > orphan.
5. Emit findings only. Do not author any TestResult or edge.

## Output Format

```yaml
audit_scope: <package or file>
summary:
  failing: <N>
  suspect: <N>
  uncovered: <N>
  orphans: <N>
findings:
  failing:
    - id: <test-uuid>
      requirement: <req-uuid>
      latest_result: fail
      judged_at: <date>
  suspect:
    - id: <test-uuid>
      requirement: <req-uuid>
      last_pass_commit: <sha>
      material_change_commit: <sha>
  uncovered:
    - id: <req-uuid>
      title: "<requirement title>"
  orphans:
    - id: <element-uuid>
      type: <Requirement|Story|Test>
```
