---
name: project-onboarding
description: Use when a keel project has never chartered its process set — `keel onboard` reports NOT CHARTERED, a project was just scaffolded, or the author asks which processes their project should run. Elicits what they are building, researches the domain's practice where the engine has no process for it, recommends a set with per-process reasoning, and records the choice as a Decision.
---

# Project onboarding — choose the process set, and record why

Deploys `.engine/processes/project-onboarding.sysml` (D0225).

**The one rule that shapes everything else: do not offer the catalogue.** D0216 forbids a menu because
a chosen option is evidence about the menu, not about the chooser's work. An author shown 24 process
names will pick the familiar ones. Ask about the WORK; recommend afterwards, with the reasoning
visible so they can strike any line of it.

## 1. Ask (deploy `business-elicitation`)

Record each answer **verbatim as a `Statement` before interpreting any of it**. Their words and your
reading are separate, separately auditable facts.

Ask about the work, roughly in this order — adapt the wording, keep the intent:

- What are you building, and who is it for?
- What has gone wrong before on work like this? What would be expensive to get wrong here?
- Who has to be convinced it is right, and what would convince them?
- Who else touches this work, and how do they find out what changed?
- What has to be provable later, and to whom? (a regulator, a client, a court, a future maintainer, nobody)
- What are you doing today that already feels like bookkeeping rather than work?

The last one matters: it finds the friction they will not tolerate, and a process that adds to it will
be abandoned no matter how well justified.

**Never ask "do you want the X process".** If they name one, record that as a Statement and treat it
as evidence about what they think they need — still test it against the elicited facts.

## 2. Match

`keel onboard` prints every process with its declared `// APPLIES-WHEN:` condition. Decide per process
whether the elicited facts satisfy the condition.

Match on the **situation, not the domain word**. "Safety-related" is not a reason to adopt anything.
The reason is that *an element being wrong is expensive*, which is what `element-critique`'s condition
states — and that condition is equally true of a bridge drawing, a dosing algorithm and a contract.
This is what lets the process serve a domain nobody listed.

A condition that is **not** met is a recommendation **against**, and it gets recorded. Silence is not
a decision, and an unmentioned process reads as an oversight later.

## 3. Research the gaps

Where the elicited work implies a discipline no declared process covers — a domain practice, a
regulatory regime, an approval or review step the engine has never modelled — research what
practitioners in that domain actually do.

- Record what you found **with its source**.
- **Never invent a standard**, and never attribute a practice to a body that did not publish it.
- If research is unavailable, **say so plainly**. An unresearched gap recorded as a gap is honest; a
  fabricated best practice is the worst possible output of an onboarding conversation, because the
  author has no way to know it was invented.
- Then either map the practice to an existing process, or **define a new one** — §3 says defining the
  process is the creative output when a request maps to none.

## 4. Recommend

Present it as an argument, not a form. For each recommended process: the elicited fact that
implicates it, and the cost of adopting it. For each one **not** recommended: the same.

State the friction honestly. Every adopted process is a discipline a human or an AI has to carry out,
and D0054 says friction is the top risk — above bad architecture.

Then read it back asking **what is wider than they meant**, not whether it is good (D0157: the finding
was that a Need had been written wider than the demand, not that it was wrong).

## 5. Charter

- `keel record decision --from FILE` (prose goes through a file — D0224): the chosen set, the elicited
  facts behind it, and **what was deliberately not adopted and why**. A rejection is a first-class
  Decision even when it causes no action (invariant 4).
- Write `.engine/contracts/activation.toml` to match, with `charteredBy = "dNNNN"` under `[processes]`.
- An activation file without a charter is a set of switches nobody can account for.

## 6. Read it back — including what it does not cover

- `keel onboard` → CHARTERED by that Decision.
- `keel activation` → the declared set. `keel guard` → green.
- **State the residual out loud**: conditions that were met but whose process the author declined, and
  any researched practice the engine still has no process for.

A project should know what it chose not to do. That is the difference between a scoped adoption and
an unexamined one.

## Do not

- Do not gate anything on onboarding. It is a view and a conversation; a project may legitimately run
  unchartered (D0098 — honest-state gates, never self-assurance gates).
- Do not charter a set the author has not seen the reasoning for.
- Do not record their agreement as a `method=confirmation` result unless they explicitly signed off on
  that specific claim. The Decision itself travels the normal channel.
