---
name: actor-enrollment
description: |
  Deploys the actor enrollment process (D0129): take an uninitiated contributor — human or AI, on a
  machine that has never run the engine — to the point of authoring truthfully. Establish the acting
  identity explicitly, ASK actor kind (human/ai) and role rather than inferring them, register the
  correctly-typed actor, bind the machine so no write path needs a default, and prove the local gate
  runs. Use when a new human or AI joins, when acting from a machine with no identity binding, when
  `keel` reports no acting actor, or when provenance/authority is in question. Do NOT use to onboard
  a fresh PROJECT (that is `introduction`, which runs AFTER this), and do NOT use for steady-state
  work (that is `distributed-collaboration`).
metadata:
  version: 0.1.0
  domain: [enrollment, onboarding, identity, provenance, actor, role, authority, distributed, D0129]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# actor-enrollment (who is acting, and may they attest it?)

Deploys `.engine/processes/actor-enrollment.sysml`. This is the **front door** of distributed
operation. Attribution, acceptance authority, and gate uniformity all presuppose a correctly
registered actor on a working machine — and every remote contributor starts without one.

**Not the same as `introduction`.** That process onboards a fresh *project* (zero to first value)
and never establishes *who is acting*. They compose, enrollment first.

## Why this exists (the two defects it closes)

Both were verified in code on 2026-08-12 and both let an AI hold authority reserved for a human:

- **issue072** — write paths *default* the acting actor. Seven `serve.rs` endpoints default
  `author`/`judged_by` to a named human; `record decision` does the same. An AI-driven call that
  merely **omits** the field records a human attestation silently. `confirmation-authenticity`
  (D0106) doesn't fail loudly — it just stops meaning anything, because what it reads is false.
- **issue073** — `capture_user.py` registered **every** actor as `: Person`, with no kind question.
  An enrolling AI became a human in the registry, and thereafter satisfied the guard *legitimately*.

So: **identity is never defaulted, and kind is never inferred.** Refusing is a correct outcome here.

## The steps

1. **Identify — explicitly, or refuse.** Establish who is acting from an explicit source: an
   existing machine binding, an explicit argument, or an answer to a question. Git committer
   identity may be *offered as a suggestion* — never taken as authority, because on an agent's
   machine it is just whatever that machine was configured with. If identity can't be established,
   **stop and say what's missing.** Never fall back to an existing actor, and never to a human.

2. **Classify — ask, don't infer.** Record two fields that must never be defaulted:
   - **kind** — human or AI. This single field is what protects human-only authority (accepting a
     Decision, dispositioning a ≥Medium finding, adjudicating a conflict). Registering an AI as a
     human `Person` must be *impossible* here, not merely discouraged.
   - **role** — the function filled while doing tracked work (contributor, supervisor, integrator,
     reviewer, or a project-declared role). Role is model data, not tribal knowledge.

3. **Register — without duplicating.** Add the correctly-typed part to `.tracking/actors.sysml`: a
   `Person` for a human, an `Actor` with `kind = ActorKind::ai` for an AI. If already registered,
   confirm and move on — a second entry is its own corruption class (issue074). Registration is what
   makes the actor referenceable by `createdBy` / `authoredBy` / `judgedBy`, which `keel guard
   actors` already enforces referentially.

4. **Bind the machine.** Write the machine-local binding that makes this actor the acting identity
   for every later write, so no write path needs a default and none is offered one. Then **read it
   back** — have the write path report the actor it would attribute to, and confirm.

   > This is the step whose absence *is* issue072. Capture already existed and printed an actor id;
   > nothing consumed it, so the write paths kept defaulting.

5. **Prove the gate runs.** Run the gate and confirm it *executes* rather than skipping:

   ```
   keel validate .        # .tracking semantic validation
   keel guard .           # all enforced honest-state guards
   keel check-engine .    # .engine reference resolution (kernel-free)
   ```

   Confirm the engine version matches what the project declares. A check that cannot run must report
   **NOT READY with the specific remedy** — never pass silently (issue076: the local gate becoming
   nothing while everyone believes it is enforced). Do not author on an unverified gate: a
   contributor whose gate silently does nothing is held to a different standard than everyone else,
   which breaks the uniform-gate guarantee for the *whole team*.

6. **Hand off.** State plainly what this actor may and may not attest given its recorded kind — if
   it's an AI, say outright that it cannot record human acceptance, rather than letting that be
   discovered at a refusal. Then hand off: `introduction` on a fresh project, or
   `distributed-collaboration` on a populated one.

**Done when** the actor can author a fact that is attributed to it truthfully and gated identically
to everyone else's.

## Anti-patterns

| Don't | Why |
|---|---|
| Infer kind from git identity or the machine | The root of issue073 — an AI silently becomes a human |
| Default to an existing actor when identity is unclear | Manufactures false provenance; refuse instead |
| Register a second entry for an existing actor | Duplicate identity, undetected today (issue074) |
| Proceed when the gate can't run | The contributor is then held to a different standard than the team |
| Record a human confirmation on a human's behalf | Never; an AI cannot supply human acceptance (D0106) |
