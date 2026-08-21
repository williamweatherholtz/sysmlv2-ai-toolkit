---
name: requirement-quality
description: |
  Audits a SysML v2 requirement (or a requirement set) against the INCOSE
  Guide to Writing Requirements rules and EARS syntax, emits pass/fail
  findings per rule, and proposes EARS-conformant rewrites. Use when a
  requirement is authored or changed, during Backlog Refinement or the
  Standup/Definition-of-Ready gate, or when asked to "check this requirement,"
  "is this requirement any good?", "clean up these requirements," "make this
  testable," "review the reqs," or "rewrite this as EARS." Also triggers on
  ambiguity/verifiability complaints and requirement-smell detection. Do NOT
  use for STPA hazard/constraint wording (use the stpa skill) or for story
  refinement/INVEST (use agile-refinement).
metadata:
  version: 0.1.0
  domain: [requirements-engineering, INCOSE-GtWR, EARS, MBSE, SysMLv2]
  writePolicy: pr-only
  engine: keel-ai-toolkit
---

# requirement-quality

Runs the engine's requirement-quality gate. Operates on `Requirement` items
(and subtypes `FunctionalRequirement`, `NonFunctionalRequirement`,
`SafetyRequirement`, `Constraint`, `MarketRequirement`) in the `.engine` schema.
Findings are advisory evidence; never silently rewrite an accepted requirement —
propose changes via PR (writePolicy = pr-only).

## Expert Vocabulary Payload

**Quality characteristics (INCOSE GtWR v4):** necessary, appropriate,
unambiguous, complete, singular, feasible, verifiable, correct, conforming;
the 42-rule families (R1–R42).

**Requirement smells:** vague terms (Mavin), escape clause, superfluous
infinitive ("be able to"), open-ended clause ("etc.", "including but not
limited to"), unbounded quantifier, compound requirement, solutioning,
unverifiable claim, dangling pronoun, TBD/TBR/TBC.

**EARS (Mavin et al.):** ubiquitous, state-driven (While), event-driven (When),
optional-feature (Where), unwanted-behavior (If/Then), complex; preamble vs
response.

**SysML v2 binding:** `subject` (1), `actor` (0..*), `stakeholder` (0..*),
`assume constraint` (precondition guard), `require constraint` (pass/fail
obligation); EARS condition → `assume`, EARS "shall" → `require`.

**Verification:** verification method (Inspection / Analysis / Demonstration /
Test), measurable acceptance threshold, units of measure, tolerance band.

## Anti-Pattern Watchlist

Scan the requirement (and your own findings) for these before reporting.

1. **Rubber-stamp pass** — Detection: you marked a requirement PASS without
   checking it against each rule family. Resolution: every finding cites a
   specific rule ID (Q1–Q31, see `references/ruleset.md`); no blanket verdicts.
2. **Solutioning disguised as a requirement** — Detection: the text names a
   mechanism/technology ("via a Kalman filter", "using PostgreSQL"). Resolution:
   flag as Q6 unless the item is a `Constraint` (where imposed solutions are
   legitimate). State *what*, not *how*.
3. **Vague-term blindness** — Detection: "fast", "user-friendly", "robust",
   "as appropriate", "if possible" pass unflagged. Resolution: run the vague /
   escape-clause / open-ended lexicon (Q12–Q16) explicitly; each hit is a fail.
4. **Compound requirement** — Detection: one statement carries two obligations
   joined by "and"/"or"/"/". Resolution: flag Q7–Q10; propose a split into
   atomic requirements (one `shall` each).
5. **Unverifiable acceptance** — Detection: a quality claim has no metric,
   value, unit, or tolerance. Resolution: flag Q18–Q20; the EARS rewrite must
   add a measurable threshold or the requirement cannot be verified.
6. **Over-rewriting** — Detection: your proposed rewrite changes the intent, not
   just the form. Resolution: preserve the requirement's meaning and `subject`;
   change only wording/structure. If intent is unclear, ask — do not invent it.
7. **Ignoring the level** — Detection: a system-level requirement is judged as
   if it were a component spec (or vice versa). Resolution: apply Q27
   (appropriate to level); do not demand component detail from a system need.

## Behavioral Instructions

1. **Scan for the anti-patterns above first.** They shape how you read every
   subsequent rule.
2. **Load the target.** Read the `Requirement` item(s): `text`, `type`,
   `subject`, `assume`/`require` constraints, `verificationMethod`, and any
   `rationale`. IF given a set, also plan the set-level checks (Q28–Q31).
3. **Run single-requirement checks Q1–Q27** from `references/ruleset.md` against
   each requirement. For each: record `{rule, verdict, evidence}`. WHY: citing
   the exact rule makes the finding auditable and the fix unambiguous.
4. **Run set-level checks Q28–Q31** (terminology consistency, conflicts,
   duplicates, coverage) IF auditing a set.
5. **Classify each requirement's EARS pattern** (see `references/ears.md`). IF it
   has a condition/trigger not in the preamble: flag Q22 and note which EARS
   pattern fits.
6. **Propose an EARS-conformant rewrite** for every requirement with ≥1 fail.
   Use the matching EARS template. Preserve intent and `subject`. Add measurable
   thresholds with units where Q18–Q20 failed; IF the needed value is unknown,
   insert an explicit `TBR` placeholder and flag it (do not invent a number).
7. **Map to the SysML v2 binding.** The EARS preamble becomes the `assume
   constraint`; the "shall <response>" becomes the `require constraint`. Note
   any `subject`/`stakeholder` named in prose that is missing from the model.
8. **Emit findings** in the Output Format. Per engine convention this is
   advisory: surface as review evidence on the requirement; propose rewrites via
   PR. Do NOT overwrite an `accepted` requirement directly (writePolicy).
9. **Compute the gate result.** PASS only if zero fails remain across Q1–Q27
   (and Q28–Q31 for sets). Otherwise REVISE with the rewrite proposals.

## Output Format

```yaml
target: <requirement id or set name>
gate: PASS | REVISE
findings:
  - rule: Q12            # rule id from references/ruleset.md
    verdict: fail
    evidence: "'user-friendly' is a vague term with no measurable criterion"
  - rule: Q7
    verdict: fail
    evidence: "two obligations joined by 'and'"
rewrites:
  - id: <requirement id>
    pattern: event-driven        # EARS pattern applied
    before: "<original text>"
    after:  "When <trigger>, the <subject> shall <response> <value+unit>."
    note: "split into REQ-x and REQ-y; threshold marked TBR pending source"
set_checks:                      # only when auditing a set
  - rule: Q30
    verdict: pass
```

## Examples

### BAD requirement → finding + rewrite
**Input:** `The system should be fast and user-friendly and handle errors gracefully.`
- Q1 fail (no "shall"; "should"). Q7/Q8 fail (three obligations + "and").
  Q12 fail ("fast", "user-friendly", "gracefully" — vague, unverifiable).
- **Rewrite (split, EARS event-driven + NFR):**
  - `When the operator submits a query, the Search_Service shall return results within 2.0 s (95th percentile).`
  - `When an invalid input is entered, then the Search_Service shall display message ERR-014 and retain the entered values.`
  - usability → separate, measurable requirement or deferred with rationale.

### GOOD requirement → PASS
**Input:** `While the vehicle is in Autonomous_Mode, when an obstacle is detected within 5 m of the path, the Path_Planner shall command a stop within 200 ms.`
- Q1 pass (single "shall"). Q7 pass (singular). Q12 pass (no vague terms).
  Q18–Q20 pass (5 m, 200 ms — measurable, units). Q22 pass (condition in
  preamble). EARS = complex (While + When). Maps cleanly to `assume` (mode +
  detection) and `require` (stop within 200 ms). **Gate: PASS.**

### Borderline (the hard case) → REVISE
**Input:** `The Brake_Controller shall apply braking as appropriate to avoid collisions.`
- Q1 pass. Q15 fail ("as appropriate" — escape clause). Q24 fail (unverifiable:
  no defined trigger, force, or timing). Q6 borderline (verges on outcome, not
  behavior).
- **Rewrite:** `When time-to-collision is below 0.6 s, the Brake_Controller
  shall command maximum deceleration (TBR m/s²) within 100 ms.` (threshold TBR
  flagged for a source value).

## Questions This Skill Answers

- "Is this requirement any good?"
- "Check / review these requirements"
- "Rewrite this as EARS"
- "Make this requirement testable / verifiable"
- "Why is this requirement ambiguous?"
- "Split this compound requirement"
- "Does this requirement have a measurable acceptance criterion?"
- "Run the requirement-quality gate on REQ-12"
- "Are there vague terms in my requirements?"
- "Clean up the requirement set / check for duplicates and conflicts"
- "Does this requirement map to a proper SysML v2 assume/require?"
