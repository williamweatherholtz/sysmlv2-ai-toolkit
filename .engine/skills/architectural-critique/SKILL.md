---
name: architectural-critique
description: |
  Runs the antagonistic architectural audit of the engine + its processes (D0046
  framework: Claims–Arguments–Evidence backbone + GQM metrics + ATAM scenarios).
  Every step tries to BREAK the architecture, not affirm it. Findings are recorded
  as tracked Issue/Decision items (NOT a prose doc). Use when asked to "audit,"
  "critique the architecture/processes," "where are we weak," "stress-test the
  engine," or on the architectural-critique process (.engine/processes/).
metadata:
  version: 0.1.0
  domain: [audit, critique, assurance-case, CAE, GQM, ATAM, adversarial, SysMLv2]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# architectural-critique

Operationalizes `.engine/processes/architectural-critique.sysml` with the D0046
framework. Adversarial by construction: the goal is to find what's wrong before it
bites. Output = tracked `Issue` items (and `Decision`s for accepted changes), each
with a concrete preventing change — never a prose report in design-history.

## The framework (D0046)

**CAE / assurance-case backbone (GSN: Goal → Strategy → Evidence).** The engine IS a
CAE machine (Requirement=claim, satisfy/verify edges=argument, TestResult=evidence), so
auditing it with CAE dogfoods its own model. Structure each audit area as:

```
GOAL (claim): "<the engine/process property we assert is true>"
 └─ STRATEGY: <how we decompose the claim into checks>
     └─ EVIDENCE: <TestResults / computed views / git facts that support or REFUTE it>
```
A claim with no evidence, or with refuting evidence, is a finding.

**GQM (Goal → Question → Metric)** makes the audit objective. Default metric set
(extend per audit):

| Goal                                   | Question                                  | Metric (how to compute)                              |
|----------------------------------------|-------------------------------------------|------------------------------------------------------|
| Ceremonies are actually run            | % sprints with all gates, in order?       | `keel guard ceremony` + `keel orient` in_progress |
| Needs are covered                      | % Needs with a satisfy edge?              | `traceability-audit` skill                            |
| Done means verified                    | any done task with stale/invalid evidence?| `keel orient` suspect + invalidEvidence            |
| Deliverable matches its claims         | Rust orient == structural truth?          | inherent — Rust is the sole authority (D0048/M4)      |
| Decisions are recorded                 | any CR commit without a Decision file?    | git log `CR:` vs `.engine/decisions/`                 |
| Docs match reality                     | any doc claim contradicted by the model?  | grep doc claims vs schema/registry/tooling            |
| Work is chartered                      | % delivery Stories with a #CharteredBy edge?| `keel audit` (charter_coverage, grandfather-aware) |
| Estimation feedback kept               | sprints recording actualHours?            | `keel audit` (estimation_discipline)               |
| Sitting review current                 | a per-sitting review recorded (D0049)?    | `keel audit` (sitting_review)                      |

**One-shot adherence metrics:** `keel audit` computes the retrospective
adherence set in one call — charter coverage, ceremony completeness, estimation discipline,
sitting-review currency — each split ACTIONABLE vs grandfathered (charter since sprint38; ceremony
post-issue010), so it never cries wolf on historical sprints. Pair it with the per-commit guards
(`validate_ceremony` / `validate_sprint_coverage` / `validate_charter` / `validate_acceptance_events`
/ `validate_process_change`), which enforce the same invariants forward. File red metrics as tracked
Issues (below), never prose.

**ATAM scenario walkthroughs** for architecture risk: build a small quality-attribute
utility tree (e.g. legibility, evolvability, correctness, performance), then play
concrete scenarios end-to-end ("a new downstream project adopts the engine," "an item's
DoD changes after it was verified," "a schema attribute is renamed"). Each scenario that
hits friction, a dead end, or a silent failure is a finding; classify it as a **risk**,
a **sensitivity point**, or a **tradeoff**.

## Behavioral Instructions (autonomous)

Follow the process steps (criScope → criMisuse → criUnused → criUseCases →
criAntiPatterns → criProcesses → criSynthesize → criAccept), applying the framework:

1. **Scope inventory (criScope).** Enumerate what exists: schema packages, workflows,
   processes, skills, tooling, instance data. Fix the surface so nothing hides.
2. **Compute the GQM metrics** above first — they cheaply surface where claims are
   already failing (run the actual tools; capture real numbers).
3. **Attack each claim (CAE).** For every area, state the GOAL as a claim and hunt
   REFUTING evidence: misuse of SysML v2 (criMisuse), unused native leverage (criUnused),
   invariant violations / state that can lie / authored views / embedded procedure
   (criAntiPatterns), process followability + enforcement (criProcesses).
4. **Run ATAM scenarios (criUseCases)** end-to-end; record every gap.
5. **Adversarial discipline:** default to "the claim is FALSE until evidence shows
   otherwise." A finding you can't back with evidence is itself suspect — drop or label it.
6. **Record findings as tracked items (criSynthesize), NOT prose:**
   - Each finding → an `Issue` in `.tracking/issues.sysml` with a concrete
     `preventing change` and a `relatedTask` (existing or new backlog action).
   - Rank by severity × leverage in the issue description.
   - An accepted architectural change → a `Decision` file (capture even "won't do").
   - If a guard is cheap and obvious, build it now (D0047: corrections become guards).
7. **Human disposition (criAccept)** happens at the per-sitting review (D0049) — present
   the ranked findings there; do not block the audit on per-finding sign-off.

## Anti-Patterns

- **Affirming, not attacking** — listing what works is not a critique. Every step must
  try to break something.
- **Prose findings** — a finding that isn't a tracked Issue/Decision can't be resolved or
  re-checked. No design-history report blobs (the 2026-06-11 critique's prose was a HIGH
  finding; D0046 fixed it).
- **Metrics theater** — computing a metric without acting on a red one. A failing metric
  is a finding.
- **Unfalsifiable claims** — "the architecture is clean" with no evidence. State claims so
  they can be refuted.
- **Boiling the ocean** — scale to the ask: a quick check runs the GQM metrics + top
  scenarios; "comprehensive audit" runs the full 8-step process with larger scenario sets.

## Output Format

```yaml
audit: <scope> (date)
gqm:
  - goal: "<claim>"
    metric: "<name>"
    value: "<measured>"
    verdict: green | RED
findings:
  - claim: "<goal that failed>"
    evidence: "<refuting evidence>"
    severity: high | med | low
    leverage: high | med | low
    preventing_change: "<guard / skill / doc / schema>"
    tracked_as: "issueNNN" | "decision:DNNNN" | "backlog:<action>"
atam_scenarios:
  - scenario: "<end-to-end walkthrough>"
    result: ok | risk | sensitivity | tradeoff
    tracked_as: "<id if a finding>"
disposition: presented at per-sitting review (D0049)
```

## Questions This Skill Answers

- "Audit the engine / processes" · "Critique the architecture"
- "Where are we weak?" · "Stress-test this" · "Run the antagonistic critique"
- "What claims can't we back with evidence?"
