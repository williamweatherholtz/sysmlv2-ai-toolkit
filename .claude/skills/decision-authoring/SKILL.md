---
name: decision-authoring
description: |
  Deploy the Decision Authoring process (.engine/processes/decision-authoring.sysml, resolves issue058):
  author every Decision so each fact has ONE canonical home — the why in its own context/decision/
  rationale/consequences fields, the acceptance verdict in a discrete human-judged method=confirmation
  TestResult, status/state COMPUTED — and NEVER restate a verdict/status/computed fact as prose in another
  field or a comment. Use whenever recording, accepting, or amending a Decision (CHANGE/RECORD, or
  `keel record decision`). Backstopped by decisionNoVerdictProseRule (`keel rules`) +
  guard:confirmation-authenticity (acceptance must be human-judged).
metadata:
  version: 0.1.0
  domain: [decisions, one-source-of-truth, ADR, MBSE, SysMLv2]
  writePolicy: direct
  engine: keel-ai-toolkit
  deploys: [.engine/processes/decision-authoring.sysml]
---

# decision-authoring — one fact, one home

The dual-truth defect (a verdict restated as prose beside its discrete TestResult) recurred even after
D0106 because "one source of truth" was only guidance. This skill deploys the **Decision Authoring
process** so it is *carried out*, not just hoped (D0059). Bound by D0058 (ADR fields), D0016/D0066
(acceptance = a human-judged confirmation TestResult), §2.1 (text is truth; state is computed).

## The steps (deploy `.engine/processes/decision-authoring.sysml`)

1. **Own facts in their fields.** context/decision/rationale/consequences carry only the decision's own
   what/why/impact (substantive, ≥20 chars). Use `keel record decision` (auto UUID + NNNN). Do not put
   another item's verdict, another decision's governance, or any computed/status fact in these fields.
   **Prose goes through a file: `keel record decision --from DRAFT.md`** (D0224). A Decision's fields are
   paragraphs, and passing paragraphs as double-quoted shell arguments is how tool output gets into a
   governance record — a backtick inside double quotes is command substitution, so the shell RUNS the
   command your prose merely names. It has happened twice here (issue255, and issue256 where 13,136
   characters had already landed in an accepted Decision). The draft file is `key: value` headers plus
   `--- key` sections taken verbatim; explicit flags still override. The write path refuses prose that
   looks like captured tool output either way, and names the mechanism when it does.
2. **Verdict is a discrete TestResult — never prose.** Acceptance = a `method=confirmation` verification +
   a passing TestResult (who/when/commit), **judged by a human `Person`** (guard:confirmation-authenticity).
   Never write an `(ACCEPTED <date> by <who>)` comment or field sentence — the TestResult is the sole source.
3. **State is computed.** Acceptance/coverage/suspicion/resolution are `#View`s (`keel orient`/`decisions`/
   `coverage`). Only the guarded `status` flag is materialized; everything else derivable is not authored.
4. **Backstop.** Before commit, `keel rules` → `decisionNoVerdictProseRule` must be 0; any flag is a defect to fix.

## Why enforceable (not just guidance)

- The process is **non-inert** — the `process-skill` guard requires this skill to name it (D0059).
- Acceptance authenticity is a **hard gate** (guard:confirmation-authenticity, issue059).
- Verdict-prose has an **automated detector** (decisionNoVerdictProseRule).
So "one source of truth" is now a carried-out, checked process — the issue058 fix.
