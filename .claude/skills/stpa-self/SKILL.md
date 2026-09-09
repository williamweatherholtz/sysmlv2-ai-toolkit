---
name: stpa-self
description: |
  Runs STPA on keel's OWN workflow (D0313, st065): from the computed control structure, for every
  control action, the four unsafe-control-action types, each traced to a controller constraint bound
  to an existing control or to an Issue naming the gap. Use when the control structure has changed
  (the stpa-currency guard warns), when a control defect asks what else the same controller can do
  wrong, or when someone says "run STPA on keel", "what are the control gaps in our workflow",
  "UCA analysis of the hooks / the gate / auto-accept". For a DELIVERABLE system use `stpa`; this
  skill is the engine analysing itself.
metadata:
  version: 0.1.0
  domain: [STPA, keel, self-analysis, control-gaps]
  writePolicy: api-only
  engine: keel-ai-toolkit
---

# stpa-self — STPA on keel itself

Deploys `.engine/processes/stpa-self.sysml` (D0313). The generic `stpa` skill's SOP
(`.engine/skills/stpa/references/sop.md`) is the method; this skill is the keel-specific run of it,
with two differences that matter:

1. **Nothing is drawn or invented.** Losses and hazards exist (`.tracking/architecture/engine-safety.sysml`);
   the control structure is **computed** (`keel show control-structure`, D0284). The run analyses that
   list, verbatim. Its `stepTwoGate` rows ARE the SOP's step-2 gate for this project - read them, do
   not re-argue them: actuators and responsibilities are derived from the actions, process models
   and the other inputs/outputs (`OtherInputOutput`, D0363) are authored in
   `.tracking/architecture/engine-control-structure.sysml`, and a role nothing wires is in
   `absentRoles`, not in the structure.
2. **The structure says when to run again, and where.** The run records the computed action names it
   analysed (`ANALYSED: ...`); the `stpa-currency` guard warns while any action is uncovered, grouped by
   the controller->process edge each sits on, and names the NEXT TRANCHE - the open edge with the fewest
   unanalysed actions - with the `ANALYSED:` list its record will carry (D0410). **A tranche is one edge
   walked in full.** Take the named edge; do not pick actions by hand.

## Procedure

| Step | Do | Produce |
|---|---|---|
| stpa1 | Read EL/EHZ in `engine-safety.sysml`; add a Hazard only if a control action exposes a loss no hazard covers | the hazard ids used |
| stpa2 | `keel show control-structure .` (and `stpa-diagram` if a human needs the picture); every `stepTwoGate` row must hold, or the run's first record is the clause that does not | the computed action list |
| stpa3 | For each action, all four UCA types in worst case: `provided`, `notProvided`, `wrongTimingOrder`, `stoppedTooSoonAppliedTooLong`. Each that can lead to a hazard → `UnsafeControlAction` part (context names the action verbatim) + dependency edge to the hazard. Considered-and-safe → one line in the run record | UCA parts; the safe lines |
| stpa4 | Each UCA → `ControllerConstraint` + edge to the enforcing control in `control-map.sysml`, or `keel record issue` + edge to the Issue | constraints bound to controls or Issues |
| stpa5 | Run record: a `Test` (analyze) with `ANALYSED: <names>` copied from the guard's NEXT TRANCHE line + passing result at HEAD | the guard's next line |

## Where the records live

`.tracking/architecture/engine-ucas.sysml` — UCAs, constraints, scenarios and the run records for
this project's own workflow. Edges: UCA → hazard, constraint → UCA, constraint → control or Issue.

## What it does not do

It does not judge whether a control is *good* — it establishes that every UCA has a named control or
a named gap. Arming (can the control fire) is `keel show controls` / control-arming.toml; substance is
the sitting review's.
