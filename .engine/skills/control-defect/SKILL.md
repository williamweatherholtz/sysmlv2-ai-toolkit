# control-defect — when the guard itself is wrong

Deploys `.engine/processes/control-defect.sysml` (D0278). Use the moment a guard, gate, hook or lens
gives a verdict you have to explain away.

## Why this isn't just a High-severity bug

An ordinary defect affects what it touched. **A broken control invalidates its own history** — every
verdict it gave while broken is unverified, and nobody knows which. The blast radius is retroactive
and unbounded until someone assesses it. That is why the human's word was *ASAP*: every hour a broken
control keeps running, it adds verdicts nobody can trust.

## The tell

You are about to write, or have just thought, one of these:

- "the guard is wrong here"
- "that's a false positive"
- "it passed, but it shouldn't have"
- **"that's the guard working correctly"** — about a verdict you had to think about

Stop there. That sentence is the only detector that exists; no gate can read surprise.

## Six steps

1. **Notice** — catch the explaining-away.
2. **Establish** against the **source**, not the output. Name `file:line`, and state in one sentence
   what the predicate *actually* tests versus what it is trusted to test. That gap is the defect. If
   you can't state it, you have a disagreement, not a finding.
3. **Classify the direction.** They are not the same defect:

   | | | |
   |---|---|---|
   | **over** | flags what is fine | Loud. Someone hits it. Damages trust — people route around it |
   | **under** | passes what is broken | **Silent.** Found by accident or by auditing the control |

   An under-reporting defect is **never below High**, whatever the case looks like: its severity is
   not the case, it is the unknown population behind it.

   Found an over-report? Go looking for the matching under-report before moving on. One wrong
   predicate usually does both, on different inputs.

4. **Ask what it PASSED while broken.** The step no ordinary defect has. Three honest answers, and
   only these: **re-checked** (say how — usually `keel reverify`), **accepted as-is** (say why, and
   make it read like the judgment it is), or **not re-checked** (say so). Silence is the one
   unacceptable answer, because silence reads as *re-checked* to everyone who comes later.

5. **Record and register, in the same act.** The Issue with a resolver, *and* a section in
   `.engine/contracts/control-defects.toml` keyed by the control's name. Both, together.

6. **Fix now, or carry deliberately and say which.** Two reasons are good enough to carry: the fix is
   a keystone change needing a signed Decision, or it would bury a change in flight. "I was busy" is
   not one. The fix must be **armed** against the broken predicate — a control-defect fix never
   observed to change a verdict hasn't been shown to fix anything.

## What the registry buys

`keel guard` prints each registered defect **beside the verdict of the control it belongs to**. A
green from a control known to under-report arrives already qualified, at the moment you are reading
it, without anyone having to remember. That's the hook.

An entry whose Issue doesn't exist or is already closed fails `control-defect-registry` — so the
announcement can't rot into noise, and the entry is removed in the same commit that resolves the
Issue, never before.

## The honest limit

**Noticing cannot be enforced.** You find a control defect by being surprised, and no gate reads
surprise — the same class as the response contract's verify-before-asserting clause. Everything
*after* the noticing is enforced. This process makes the response reliable, not the detection.

## Removal path

Delete this skill + registry + the process file + the contract. `keel guard` stops printing defect
notes; the guards keep running.
