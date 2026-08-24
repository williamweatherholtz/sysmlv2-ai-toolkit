---
name: business-architecture
description: |
  The keel-native design skill: runs a design/scoping dialogue by DEPLOYING the Business workflow
  (.engine/workflows/business.sysml: Brief -> Personas -> Needs -> UseCases) and then the Architecture
  workflow (.engine/workflows/architecture.sysml: data/app/tech -> allocation), recording every
  accepted decision as an atomic Decision AT THE POINT OF DECISION, keeping Needs strictly upstream of
  architecture, and emitting AUTHORED FACTS (Brief/Needs/Decisions/SystemRequirements + typed edges),
  never a prose design document. Use whenever a request means "let's design/scope/figure out X,"
  "what should we build," a new capability, or any architectural direction. Replaces the generic,
  keel-blind brainstorming skill for keel projects (issue054 F1 root: the Business/Architecture
  workflows were inert — defined but with no deploying skill, so a keel-blind skill filled the vacuum
  and produced prose + uncaptured decisions in the wrong order).
metadata:
  version: 0.1.0
  domain: [business-workflow, architecture-workflow, requirements, design, MBSE, SysMLv2]
  writePolicy: direct
  engine: keel-ai-toolkit
  deploys: [.engine/workflows/business.sysml, .engine/workflows/architecture.sysml]
---

# business-architecture — design by deploying the workflows, not by chatting

Design/scoping is not free-form conversation that later gets "written up." It is the **Business
workflow** then the **Architecture workflow**, executed as tracked work: each step produces its
declared artifact as authored facts, and each decision is recorded when it is made. This skill is the
deployer for both workflows (D0059: no inert workflow). It is bound by D0106 (strict
process-boundedness) and D0105 (declarative controls).

## The one ordering rule (non-negotiable)

**Needs are upstream of architecture.** You may *explore* architecture early (you need to know a
solution is feasible to write a good Need), but you may not **record** an architecture Decision or
SystemRequirement until the Needs it serves exist and are accepted. Recorded order is always
Business -> Architecture. Recording architecture-before-needs is the issue054 defect; do not repeat it.

## Flow

### Phase 1 — Business (`.engine/workflows/business.sysml`: `brief -> personas -> needs -> useCases`)

0. **Record their words FIRST** — deploy `.engine/processes/business-elicitation.sysml`. Before any
   Brief or Need exists, author a `Statement` per substantive answer with `text` verbatim (the same
   artifact intake authors, deliberately — an elicited utterance and a volunteered one are one kind of
   fact), then a `UserStory` per implication with `#DerivedFrom` to its Statement. **Why this is step
   0 and not a nicety (issue245):** this skill has authored 75 Needs from conversations whose
   utterances were never recorded, so nobody can check a Need against what the stakeholder actually
   said. Once the Need is written the justifying wording is gone and every later reader is comparing
   my paraphrase to my conclusion. Ask about **pain, not features** — never offer a menu and ask
   which they want; a chosen option is evidence about the menu, not about the need.
1. **Brief** — capture the problem/opportunity as a `Brief` (the what/why + constraints). One item.
2. **Personas** — the stakeholders/actors, as `Persona`/actor items.
3. **Needs** — solution-free, *measurable* outcomes as `Need` items (MoSCoW), each traced to the
   Brief AND carrying `#DerivedFrom` to the UserStory that implicated it. A Need like "it's slow" is
   not verifiable; "cold load < 1s for a 2,500-item set" is. A Need with no `#DerivedFrom` to a
   UserStory is **my judgment**, legitimate under D0064 but never to be dressed as elicited — say so
   in the Need. Where the stakeholder did not state the threshold, record that too rather than
   picking a number that will later be read as theirs.
4. **UseCases** — the concrete usage, as `UseCase` items traced to Needs.
   **Gate:** the human **accepts the Needs** (an explicit `method=confirmation` sign-off) before Phase 2.
   Ask specifically **what is WIDER than they meant**, not whether the set is good — the second
   question invites assent. D0157 records the failure this prevents: N-8 was not wrong, it was written
   wider than the demand, and a later reader decomposed the wide half. A Need the stakeholder narrows
   is the process working; a Need nobody narrowed is *unconfirmed*, not confirmed.

### Phase 2 — Architecture (`.engine/workflows/architecture.sysml`: data/app/tech -> allocation)

5. Derive `SystemRequirement`s with **measurable thresholds**, `satisfy`-linked to the Needs and
   `verify`-linked to Tests. Record architecture **Decisions** (axum vs actix, drop-in vs rewrite, …)
   — each with context/rationale/consequences, each `#DerivedFrom`/satisfy-linked to the Need it serves.
6. Allocation & interfaces as the workflows declare.

### Then — refine into a backlog (hand to `backlog-refinement` -> `sprint-planning`).

## Record every decision AT THE POINT OF DECISION (D0106)

The moment the human accepts a direction — scope, tech, a cut — record it **immediately** as a
`Decision` (proposed until its sign-off gate), with the *why*. Do not let decisions accumulate in
conversation "to be written up later" (issue054/issue055). The AI's creative output here is the
*authored model* (items + typed edges + decisions), **never a prose design doc** (§2.1 — that would be
an un-regenerable document; the model IS the spec, and `keel render`/`keel report` compute the
readable views).

## One source of truth (D0105 / issue058)

Each fact has ONE canonical home: a Need's outcome in the `Need`, a decision's why in the Decision's
`rationale`, an acceptance verdict in a `method=confirmation` TestResult. **Never restate a
verdict/status/acceptance as prose** in another item's fields. Sign-off is an explicit gate, not a
narrated aside.

## Guardrails

- **Parse first** (D0106): open with the `Parsed:` block; route the design request here explicitly.
- **Needs before architecture**, always (the ordering rule above).
- **Output = authored facts**, validated green (`keel validate` / `keel guard`), not a document.
- **Human gates are explicit** `method=confirmation` steps: Needs accepted before architecture;
  architecture Decisions accepted before they are `status=accepted`.
- Downstream projects: this is how "design" happens under keel — deploy the workflows, don't chat.
