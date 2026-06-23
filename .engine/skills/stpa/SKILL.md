---
name: stpa
description: |
  Drives a System-Theoretic Process Analysis (STPA, Leveson & Thomas) end to
  end, creating proper SysML v2 safety items in the .engine schema: Losses,
  Hazards, System-level Safety Constraints, the hierarchical control structure
  (Controllers, Control Actions, Feedback, Sensors, Actuators, Process Models),
  Unsafe Control Actions, Controller Constraints, and Loss Scenarios. Use when
  the work is safety-relevant and someone asks to "do a hazard analysis,"
  "run STPA," "identify unsafe control actions," "find loss scenarios," "what
  could go wrong with this controller," "derive safety requirements," or
  "build the control structure." Also triggers for STAMP-based analysis and
  safety-case construction. Do NOT use for HARA/ASIL (out of scope per decision
  0008), generic risk registers (use Risk items), or requirement wording (use
  requirement-quality).
metadata:
  version: 0.1.0
  domain: [STPA, STAMP, system-safety, hazard-analysis, MBSE, SysMLv2]
  writePolicy: pr-only
  engine: keel-ai-toolkit
---

# stpa

Executes the four STPA steps as a tracked, iterative procedure, producing
atomic SysML v2 items from `Engine::Safety` connected by typed edges. STPA is a
**worst-case** method, not average/most-likely. Steps are iterative — revisit
earlier steps when later ones expose gaps. Full SOP, per-element data fields,
and per-step completeness gates are in `references/sop.md` — read it before
running a step.

## Expert Vocabulary Payload

**STAMP / STPA (Leveson):** STAMP accident model, hierarchical control
structure, control loop, process model, control algorithm, unsafe control
action (UCA), loss scenario, system-level hazard, system-level constraint,
worst-case analysis.

**Step 1 wording:** loss (`L-n`), hazard as `<System> & <Unsafe Condition> &
<Link to Losses>` (`H-n`), system boundary, controllable vs uncontrollable,
safety constraint by inversion (`SC-n`), response constraint, sub-hazard
`H-n.m`.

**Step 3 UCA guidewords:** providing causes hazard, not-providing causes hazard,
wrong timing/order (too early/too late/out-of-order), stopped-too-soon /
applied-too-long (continuous actions only); five-part UCA form; controller
constraint (`C-n`).

**Step 4 causality:** unsafe controller behavior, inadequate control algorithm,
inadequate process model, inadequate/missing feedback, control-path failure,
controlled-process failure; causal chain (not a factor list).

## Anti-Pattern Watchlist

These are the most common and most damaging STPA errors. Scan for them in every
item you author.

1. **Hazard-as-failure** — Detection: a "hazard" names a component failure or
   cause ("brake failure", "operator distracted", "sensor fault"). Resolution:
   hazards are SYSTEM STATES to be prevented, not causes. Re-word to a system
   condition; the failure belongs in a Step-4 Loss Scenario.
2. **The word "unsafe" in a hazard** — Detection: hazard text contains "unsafe",
   "unintended", or "accidental" (recursive/ambiguous). Resolution: state the
   concrete unsafe condition ("aircraft violate minimum separation"), not a
   self-referential label.
3. **Component reference in a hazard** — Detection: hazard names a specific part
   (brakes, engine, hydraulic line). Resolution: raise to system-level state;
   components appear only in the control structure and scenarios.
4. **UCA context = belief, not reality** — Detection: a UCA says "...when the
   controller believes X" or "...when it incorrectly thinks Y". Resolution: UCA
   context is the ACTUAL state of the world. The mistaken belief is a Step-4
   process-model flaw, not the UCA context.
5. **UCA context = outcome** — Detection: a UCA ends "...resulting in a
   collision". Resolution: state the context that makes the action unsafe, not
   the consequence. (Outcome may be appended optionally for clarity only.)
6. **Skipping a guideword class** — Detection: a control action has fewer than
   four guideword cells considered, or "stopped-too-soon" marked N/A for a
   continuous action. Resolution: evaluate all four classes for every control
   action; N/A only for genuinely discrete actions, and verify it.
7. **Scenario as a factor list** — Detection: Step-4 output is bullet factors,
   not a causal chain. Resolution: write each scenario as a chain (UCA/context →
   mechanism → why it occurs → resulting hazard); finish every feedback-based
   scenario with WHY the feedback is inadequate.
8. **Orphan UCA / broken trace** — Detection: a UCA links to no hazard, or a
   constraint to no UCA. Resolution: every UCA `dependency`→ ≥1 Hazard; an orphan
   signals a missing hazard — add it. Maintain Loss←Hazard←SC/UCA←C←Scenario.

## Behavioral Instructions

1. **Scan for the anti-patterns above** as you author every item — especially
   #1, #4, #7 (the highest-frequency errors).
2. **Determine the current step.** STPA is iterative; resume where the model
   left off. Read existing `Engine::Safety` items to see what exists.
3. **Step 1 — Purpose.** Identify stakeholders and `Loss` items; define the
   system boundary; author `Hazard` items in the `<System> & <Unsafe Condition>
   & <Links to Losses>` form; derive `SystemSafetyConstraint` items by inverting
   each hazard. Keep ~7–10 system hazards. Link Hazard→Loss via `dependency`.
   Run the Step-1 completeness gate (`references/sop.md`).
4. **Step 2 — Control structure.** Author `Controller`, `ControlledProcess`,
   `ControlAction` (downward), `Feedback` (upward) items with FUNCTIONAL labels
   (never "Command"/"Status"/"Computer"). Capture each controller's
   responsibilities (traced to SCs) and `ProcessModel` variables. Defer
   `Sensor`/`Actuator` to Step 4. Run the Step-2 gate.
5. **Step 3 — UCAs.** For EVERY `ControlAction`, evaluate all four guideword
   classes. Author each `UnsafeControlAction` in the five-part form (source,
   type, action, **actual-state context**, hazard links). Invert each into a
   `ControllerConstraint` (`C-n`) linked to its UCA. Run the Step-3 gate.
6. **Step 4 — Loss scenarios.** Add `Sensor`/`Actuator` items to the control
   structure. For each UCA, author Type-(a) scenarios (why the UCA occurs:
   controller failure, inadequate algorithm, unsafe input, inadequate process
   model + the feedback cause behind it). Also author Type-(b) scenarios (why
   actions are improperly executed: control-path and controlled-process
   failures). Write each as a causal chain ending in a `[H-…]`. Run the Step-4
   gate.
7. **Derive outputs.** From scenarios, propose `SafetyRequirement` items
   (`Engine::Core`) via `:>`/`satisfy` from the relevant constraints, plus
   `Test` items (method = analysis/inspection for analysis closure). WHY: this
   closes the loop from analysis to verifiable requirements.
8. **Maintain traceability and emit.** Ensure Loss←Hazard←SC/UCA←C←Scenario is
   intact. Author/modify via PR (writePolicy = pr-only). Do NOT author computed
   coverage/completeness — that is a view computed by the engine from the items
   and the gate rules.

## Output Format

Per item authored, conform to the data fields in `references/sop.md`. Summarize a
run as:

```yaml
step: 1|2|3|4
created:
  losses: [L-1, L-2]
  hazards: [H-1, H-2]
  constraints: [SC-1, SC-2]
  ucas: [UCA-1, ...]            # step 3+
  scenarios: [S-1, ...]        # step 4
gate: PASS | INCOMPLETE
gate_misses:
  - "H-3 references a component ('brakes') — violates Step-1 criterion"
  - "ControlAction 'Brake' missing wrong-timing UCA assessment"
trace_ok: true|false
```

## Examples

### BAD vs GOOD hazard (Step 1)
- **BAD:** `H-2: Brake actuator fails during landing.` → component + failure
  (anti-patterns #1, #3). This is a cause, not a hazard.
- **GOOD:** `H-2: Aircraft is unable to decelerate on the runway [L-1, L-3].` →
  system state, prevented, links to losses.

### BAD vs GOOD UCA (Step 3)
- **BAD:** `BSCU does not provide Brake when it thinks the plane is airborne,
  resulting in a runway overrun.` → context is a belief (#4) and an outcome (#5).
- **GOOD:** `UCA-1: BSCU does not provide the Brake command during the landing
  roll while the aircraft is on the ground [H-2].` Context = actual state.
  → Controller constraint: `C-1: BSCU must provide the Brake command during the
  landing roll when armed [UCA-1].`

### Loss scenario as a causal chain (Step 4) — the representative case
`S-3 (for UCA-1, type-a inadequate process model): During the landing roll, the
wheel-speed sensor reports near-zero speed because the wheels are locked
(hydroplaning on a wet runway). The BSCU's process model therefore believes the
aircraft is stopped and withholds the Brake command. As a result the aircraft
fails to decelerate [H-2]. → Mitigation: add an independent ground-speed
feedback source; SafetyRequirement SR-7.` Note the chain ends in a hazard and
explains *why* the feedback is wrong — not a bare factor.

## Questions This Skill Answers

- "Run STPA on this system"
- "Do a hazard analysis"
- "Identify the unsafe control actions for this controller"
- "What are the loss scenarios for UCA-3?"
- "Build the control structure"
- "Derive safety constraints from these hazards"
- "Is this a valid hazard, or is it actually a cause?"
- "Turn these scenarios into safety requirements"
- "Check my STPA for completeness"
- "Why is my UCA wording wrong?"
- "What feedback does this controller need?"
