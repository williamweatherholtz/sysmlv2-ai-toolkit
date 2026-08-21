---
name: element-critique
description: |
  Deploys the Antagonistic Element Critique process (D0080/D0079): adversarially verify a
  tracked Need / SystemRequirement / Decision / Architecture element through required
  adversarial LENSES, by an INDEPENDENT critic, recording each critique as a method=critique
  verification + result (#Verify-linked to the target). Findings become severity-carrying
  Issues (the #Resolves loop); coverage + staleness COMPUTE from the assurance substrate.
  Use when asked to "critique / red-team / adversarially review" a requirement, need, or
  decision; to "find weaknesses / gaps / ambiguities" in the model; "what hasn't been
  critiqued"; or to respond to a critique going suspect after a change. Do NOT use for the
  whole-engine architectural audit (use architectural-critique) or for routine code review.
metadata:
  version: 0.1.0
  domain: [critique, verification, assurance, coverage, computed-state, SysMLv2, D0079, D0080]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# element-critique

Runs the engine's Antagonistic Element Critique process (`.engine/processes/element-critique.sysml`).
Its defining move: **antagonistic critique IS a lens-tagged verification** — a `method=critique`
Test + TestResult `#Verify`-linked to the target, by a critic INDEPENDENT of the author. It reuses
*all* the assurance machinery: coverage (`keel coverage`), staleness (`suspect`), the human
confirmation gate, and findings-as-Issues (`#Resolves`, D0077/D0078). No parallel subsystem.

## Expert Vocabulary Payload

**Critique = verification (D0080):** `verification <name> : Test { :>> method = VerificationMethod::critique;
:>> lens = CritiqueLens::<lens>; :>> critiquedBy = CriticKind::<aiModel|human|tool>; ... }` + a
`part <name>R1 : TestResult { :>> outcome = ...; :>> judgedBy = <critic id>; :>> judgedAgainst = <sha>; }`,
`#Verify`-linked to the target (import `EngineRelationships::*`).

**Lenses (CritiqueLens, requirement-quality canon):** `completeness` (gaps vs intent), `correctness`
(sound/right), `ambiguity` (clear/unambiguous), `testability` (verifiable), `feasibility`
(achievable — premortem), `consistency` (no conflicts), `necessity` (needed / in scope). Required
lenses are BROAD (D0079) across needs/requirements/decisions/architecture.

**Severity (Severity, D0079):** `Critical` (defeats purpose/unsafe — must be fixed or risk-accepted),
`High` / `Medium` (need human disposition), `Low` (AI-dispositioned, surfaced not gated). The
human-disposition gate fires at **>= Medium**. Severity lives on the finding **Issue**.

**Independence:** the critic IDENTITY (result `judgedBy`) MUST differ from the target's author. A
self-critique is not a critique.

**Computed, never stored:** critique coverage + staleness are views. A critique goes **suspect**
when its target changes after the critique commit (same git-temporal rule as D0050) — re-critique
at HEAD. Read state from tooling, not memory.

## Anti-Pattern Watchlist

1. **Affirming instead of attacking** — Detection: a critique that explains why the element is fine.
   Resolution: the lens must try to BREAK the element; a clean result is *survived the attack*, not
   *looks good*.
2. **Self-critique** — Detection: result `judgedBy` == target author. Resolution: an independent
   critic must perform it (different identity; ideally a different model/human).
3. **Finding left as prose** — Detection: a weakness described only in the critique notes.
   Resolution: raise an `Issue` (with `severity`) and triage it via `#Resolves` (issue-resolution).
4. **Skipping disposition on >= Medium** — Detection: a Medium/High/Critical finding with no human
   disposition. Resolution: a human must ACT / ACCEPT-RISK (confirmation) / DISMISS (mooting
   Decision). Critical cannot be left open. Never infer the disposition (D0016/D0051).
5. **Hand-refreshing stale critique** — Detection: editing a critique to "still valid" after the
   target changed. Resolution: append a NEW critique TestResult at HEAD; staleness computes.
6. **Storing a critique-coverage field** — Detection: a `critiqued`/`coverage` attribute on an
   element. Resolution: rejected (§2.1) — coverage computes from critique results.

## Behavioral Instructions

1. **Scope:** pick targets — prioritize the `keel coverage` gap set (uncovered requirements/needs
   first), then accepted Decisions / Architecture. Choose the relevant canon lenses per element.
2. **Critique:** for each (element x lens), an independent critic attacks the element through the
   lens. Record the `method=critique` Test (lens, critiquedBy) + TestResult (outcome, judgedBy != author,
   judgedAgainst HEAD), `#Verify`-linked to the target.
3. **Findings:** every `fail` → an `Issue { description; severity }` triaged via the issue-resolution
   skill (`#Resolves` to a resolving action or mooting Decision). Also add the TYPED finding→critique
   link `#DependsOn dependency from <issueNNN> to <critiqueTest>;` (D0102) — so a later ACCEPT-RISK/DISMISS
   disposition clears the target's `critique_suspect` via a typed path (never prose).
4. **Disposition (D0092):** for every >= Medium finding, get the human's TYPED verdict and record it
   via `keel apply-review` (batch verdict `act`/`accept-risk`/`dismiss` on the finding Issue) — a
   `#Dispositions`-linked `method=confirmation` verification carrying `disposition : DispositionKind`,
   never prose. ACCEPT-RISK/DISMISS close the finding; ACT also needs a `#Resolves` resolver. Critical
   must be fixed or risk-accepted. Low is AI-dispositioned. Read state via `keel dispositions`.
5. **Staleness:** if a target changed, its critiques are suspect — re-critique at HEAD and re-open any
   accept-risk disposition whose basis changed.
6. **Verify:** validate green; read the critique coverage + open findings from tooling before committing.

## Output Format

```yaml
target: <element name>
type: Need | SystemRequirement | Decision | Architecture
critiques:
  - lens: <lens>
    critic: <identity>            # != target author
    critic_kind: aiModel | human | tool
    outcome: pass | fail          # pass = survived the lens
    finding: <issueNNN | null>    # Issue raised on fail
    severity: Critical | High | Medium | Low | null
    disposition: act | accept-risk | dismiss | n/a   # required if severity >= Medium
suspect: <true|false>             # critique stale vs HEAD (re-critique if true)
```

## Questions This Skill Answers

- "Critique / red-team / adversarially review this requirement / need / decision"
- "Find the weaknesses / gaps / ambiguities in element X"
- "Which elements haven't been critiqued (and through which lenses)?"
- "Element X changed — its critique is stale, what do I do?"
- "How do I record a critique finding / disposition?"
