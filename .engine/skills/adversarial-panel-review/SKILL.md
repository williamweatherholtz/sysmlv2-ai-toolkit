---
name: adversarial-panel-review
description: Use when a design artifact carries enough scope or risk that one author's judgment should not be the last word — a multi-work-package plan, a schema/process change, a direction Decision — and the human has asked for panel review or accepted its cost. Convenes a persona-diverse critic panel (pessimists, optimists, correctness pedants, a reducer, a systems engineer) whose ACCEPT verdicts are conditional on named, verifiable changes; iterates artifact revisions until every panelist independently accepts. Deploys the Adversarial Panel Review process (D0187).
---

# Adversarial Panel Review

Deploys `.engine/processes/adversarial-panel-review.sysml`. The method that hardened the
2026-08-21 production-readiness design from 2 ACCEPT / 7 OBJECT to 9/9 uncompelled ACCEPT
in three rounds — catching a laundered stale premise, a sign-off fabrication seam three
critics found independently, an unimplementable merge algorithm, and a gate deadlock.

## Why this works when instructions don't

A skill is a linter — soft, ignorable (D0106's violations, issues 082–085). A critic whose
acceptance is CONDITIONAL on named changes creates an action-space loop: the author cannot
exit until every flip condition is resolved in substance or refuted with repository
evidence. The two load-bearing mechanics are the **honesty rule** and the **minimal flip
set** — without the first, unanimity is theater; without the second, convergence is
unbounded.

## The roster (scale to the stakes)

| Persona | Obsession |
|---|---|
| PESSIMIST-1 (premise) | the approach does not work at all; argue from recorded history |
| PESSIMIST-2 (operations) | day-2 reality: bypass dynamics, platform breakage, maintenance, cost |
| OPTIMIST-1 (steelman) | what is load-bearing and must NOT be diluted; names its walls in advance |
| OPTIMIST-2 (value/ROI) | payback order; defends value-dense scope against the reducer with evidence |
| PEDANT-1 (facts) | verify every citation, count, quote, and claim against the repository |
| PEDANT-2 (consistency) | contradictions, untestable acceptance, governance coherence, pre-resolved forks |
| PEDANT-3 (mechanisms) | concrete failure scenarios: inputs/state → wrong outcome, per mechanism |
| REDUCER | the minimal plan; does the goal fail without each item? |
| SE (kernel) | implementation-free behavioral invariants; orphan scope; mechanism-as-requirement |

Small artifact → 3–4 personas (one pessimist, one pedant, the reducer, the SE). Do NOT
use this process for small mechanical changes — a full nine-critic, three-round panel
costs on the order of millions of subagent tokens.

## Procedure

1. **Convene.** Every panelist prompt carries: the artifact path; the repo root for claim
   verification ("verify against the repository, never against the artifact's own
   assertions"); the persona; the HONESTY RULE — *the persona is a lens, not a script: if
   the artifact survives your attack, say so; do not manufacture objections to stay in
   character; do not capitulate to end the loop*; and the output contract — numbered
   findings, each `[BLOCKER|MAJOR|MINOR] claim — evidence (file:line or reasoning) — the
   specific change that resolves it`, ending in `VERDICT: ACCEPT` or `OBJECT (minimal
   flip set: finding numbers)`.
2. **Dispatch in parallel** (independent contexts, one message, concurrent). From round 2
   each panelist gets the revision **plus a digest of all panelists' prior positions** and
   an explicit invitation to rebut other panelists' resolutions — cross-checking is where
   laundered premises die.
3. **Revise.** Resolve every flip condition in substance, not by renaming. Conflicts
   between panelists are resolved on evidence, and the choice is recorded. Maintain a
   finding→resolution map inside the artifact so the next round verifies instead of
   re-litigating.
4. **Converge.** Repeat until every panelist independently returns ACCEPT. Uncompelled or
   nothing: a panelist is never argued into accepting — only shown resolutions or
   evidence-backed refutations. Stalled after ~3 rounds → hand the irreducible
   disagreements to the human.
5. **Record.** Surviving findings → Issues/tasks via the write API; direction outcomes →
   proposed Decisions (human accepts); charter-time residuals ride their Decisions. The
   recorded provenance carries the independence note verbatim (below).

## Guardrails

- **Independence honesty (permanent).** Same-model subagent panelists are a rigor
  framing, NOT independent critics: their critiques record `critiquedBy = aiModel` and
  satisfy no critic-independence requirement (2026-08-14 panel precedent,
  `.tracking/panel-critiques.sysml`). The record claims *convergence*, never independence.
- **Optimists are not decoration.** Their advance-notice walls are what stop the reducer
  from gutting load-bearing scope in later rounds; a panel without value-defenders
  converges by dilution.
- **Panelists verify, authors synthesize.** The author never argues a panelist down; the
  author's only moves are resolve, or refute with repository evidence the panelist then
  verifies.
- **Findings the author disagrees with still get resolved or refuted on the record** —
  the finding→resolution map is part of the artifact, not private notes.

## Removal path (D0163)

Delete this directory, `.engine/processes/adversarial-panel-review.sysml`, the registry
entry, and the `process-enforcement.toml` entry. Nothing else reads them.
