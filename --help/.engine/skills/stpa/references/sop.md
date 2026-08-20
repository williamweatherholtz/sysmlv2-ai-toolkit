# STPA Standard Operating Procedure

Source of record: STPA Handbook, Leveson & Thomas, MIT PSAS, March 2018, Ch. 2.
STPA is iterative and worst-case. Four steps: Purpose → Control Structure →
UCAs → Loss Scenarios.

## STEP 1 — Define the Purpose
1. List stakeholders; capture what each values.
2. Translate values into **Losses** (`L-n`). A loss is something unacceptable to
   lose. MUST NOT reference components or causes (no "failure", "human error").
   May involve the uncontrolled environment.
3. Define the **system boundary** — include what designers can control;
   uncontrollable things stay in the environment.
4. Author **Hazards** (`H-n`): form `<System> & <Unsafe Condition> & [Links to
   Losses]`. Three criteria: (a) a system state/condition, not a cause/component;
   (b) will lead to a loss in some worst-case environment; (c) a state to be
   PREVENTED (not a normal operating state). Keep ~7–10. Avoid "unsafe /
   unintended / accidental".
5. Author **System-level Safety Constraints** (`SC-n`) by inverting each hazard:
   `<System> & <Condition to Enforce> & [Links to Hazards]`. Add response
   constraints where needed: `If <hazard> occurs, then <what must be done>`.
   Constraints state WHAT, never a solution.
6. (Optional) Refine into sub-hazards `H-n.m` / sub-constraints `SC-n.m`.

**Step-1 gate (all must hold):** every loss traces to a stakeholder value; no
loss names a component/cause; every hazard fits the form and links ≥1 loss; no
hazard names a component; every hazard is a preventable system state; no
"unsafe/unintended/accidental"; ~7–10 hazards; every hazard has ≥1 SC;
constraints are solution-free.

## STEP 2 — Model the Control Structure
A functional model of feedback control loops. Not physical, not executable,
assumes no obedience.
1. Conventions: vertical = authority; **downward arrows = control actions,
   upward = feedback**; horizontal = peer coordination; arrows = info that CAN be
   sent (not guaranteed/obeyed). No required 1:1 controller↔process mapping.
2. Build top-down, abstract first. Identify **Controllers** (functional roles,
   e.g. "Autobrake Controller", not "Single-Board Computer") and **Controlled
   Processes**.
3. Assign **Responsibilities** per controller (`R-n`, traced `[SC-…]`).
4. Define **Control Actions** from responsibilities (specific labels).
5. Define **Process Models** (what the controller must believe) and the
   **Feedback** needed to maintain each belief (specific labels). Document the
   Responsibility→ProcessModel→Feedback mapping.
6. **Defer Sensors and Actuators to Step 4.**

**Step-2 gate:** ≥1 controller, ≥1 control action, ≥1 controlled process; arrows
labeled functionally (no "Command/Status/Computer"); each physical process has
≥1 controller (or justified); responsibilities traced to SCs; process-model
variables identified with the feedback to maintain them; sensors/actuators
deliberately deferred; supporting docs recorded.

## STEP 3 — Identify Unsafe Control Actions
1. For EVERY control action, evaluate all four guideword classes:
   (1) **not providing** causes a hazard; (2) **providing** causes a hazard
   (sub-cases: never safe here / wrong amount / wrong direction); (3) **wrong
   timing/order** (too early, too late, out of order); (4) **stopped too soon /
   applied too long** (continuous actions ONLY; N/A for discrete — verify).
   The four classes are provably complete.
2. Write each **UCA** (`UCA-n`) in five parts: `<Source controller> <Type>
   <Control Action> <Context> [Hazard links]`. Use "when/while/during" for
   context.
3. **Context rules:** context = the ACTUAL state that makes the action unsafe
   (environment, process state, prior actions, parameters). NOT a belief/process-
   model flaw. NOT the outcome.
4. Trace every UCA to ≥1 hazard; an orphan UCA → add/revise a hazard. Record in
   the action × guideword table per controller (human controllers included).
5. Invert each UCA into a **Controller Constraint** (`C-n`, linked `[UCA-…]`).

**Step-3 gate:** all four classes considered per action; class-4 applied to
continuous actions, N/A verified for discrete; parameterized actions checked for
insufficient/excessive/wrong-direction; every UCA has all five parts; context is
the actual state (not belief, not outcome); every UCA traces to ≥1 hazard; each
UCA inverted into a `C-n`.

## STEP 4 — Identify Loss Scenarios
First, **add Sensors and Actuators** to the control structure.
- **Type (a) — why a UCA occurs.** Causes: (1) controller failure; (2) inadequate
  control algorithm (flawed impl / flawed spec / degrades over time; common:
  assumes a prior action succeeded with no confirming feedback); (3) unsafe
  input from another controller; (4) inadequate **process model** (wrong/ignored
  feedback, missing/delayed feedback, or needed feedback doesn't exist). Finish
  any process-model scenario with the feedback/sensor cause.
- **Type (b) — why actions are improperly executed / not executed** (even
  without a UCA): control-path failures (controller→actuator→process: not
  received / actuator no response / not applied) and controlled-process failures
  (missing inputs, disturbances, degradation, conflicting commands).
- Write each as a **causal chain** ending in a `[H-…]`, not a factor list.
- (Optional security extension) consider inject / spoof / tamper / intercept /
  DoS / disclose per feedback/control/process path.
- Convert scenarios into requirements, mitigations, design/architecture
  decisions, and test cases. Maintain Loss←Hazard←SC/UCA←C←Scenario.

**Step-4 gate:** sensors/actuators added; BOTH scenario types covered; type-(a)
covers all four causes plus the feedback cause behind any process-model flaw;
type-(b) covers control-path and controlled-process causes; every feedback-based
scenario explains WHY (no dangling "to be finished"); scenarios are causal
chains; each scenario yields ≥1 requirement/mitigation/test; end-to-end trace
intact.

## Data fields per element
| Element | Fields |
|---|---|
| Loss | id (`L-n`); description; priority (opt); exclusions |
| Hazard | id (`H-n`/`H-n.m`); system; unsafeCondition; lossLinks `[L-…]` |
| SystemSafetyConstraint | id (`SC-n`); system; conditionToEnforce (or response form); hazardLinks `[H-…]` |
| Controller | name (functional role); description; responsibilities (`R-n` `[SC-…]`); processModel vars; type (human/auto/org) |
| ControlAction | name; sourceController; target; discrete\|continuous; parameters |
| Feedback | name; source (process/sensor); destinationController; processModel var(s) supported |
| ControlledProcess | name; controller(s); physical inputs/disturbances |
| Sensor | name; measuredVariable; feedbackProduced; consumingController |
| Actuator | name; controlActionExecuted; controller↔process span |
| ProcessModel | owningController; variables; feedback source per variable |
| UnsafeControlAction | id (`UCA-n`); sourceController; type (1of4); controlAction; context (actual state); hazardLinks `[H-…]`; rationale |
| ControllerConstraint | id (`C-n`); controller; requiredBehavior (inverse of UCA); ucaLink `[UCA-…]` |
| LossScenario | id; type (a/b); associated UCA/action; causalMechanism + why; resultingHazard `[H-…]`; derivedRequirement/mitigation; (opt) securityVector |

## Sources
- STPA Handbook (Leveson & Thomas, 2018): https://www.flighttestsafety.org/images/STPA_Handbook.pdf
- MIT PSAS: http://psas.scripts.mit.edu/home/
- UCA table format (Astah): https://astah.net/support/astah-system-safety/unsafe-control-action-table/
