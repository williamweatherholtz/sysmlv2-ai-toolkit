// stpaSelf — STPA run on keel's OWN workflow, repeatably (D0313; st065).
//
// WHY A PROCESS OF ITS OWN. The generic `stpa` skill drives a hazard analysis for a DELIVERABLE
// system. keel is also a system - the human said so (st066: "i'm not sure we have a defined system
// for keel itself. that should be the first step") - and its control structure is now COMPUTED from
// the hook config, git hooks, workflow files, CLI facts and declared deciders (D0284). The human
// paired the verification revamp with "a strong STPA process to identify control gaps, if we're going
// to be that cavalier with AI" (st065). Recurring by their words, so D0040 makes the process itself
// the deliverable: define it, run it once, and let the computed structure say when to run it again.
//
// WHAT IS DIFFERENT FROM THE GENERIC SKILL. Steps 1 and 2 READ - the losses and hazards exist
// (engine-safety.sysml) and the control structure is computed, never authored. Step 3 walks the
// COMPUTED control actions, not a hand-drawn set. Step 5 is the re-run trigger: the run records
// which computed actions it analysed, and the `stpa-currency` guard warns when the structure has
// grown an action no run has looked at - a new hook event, workflow step or write command.
package ProcessStpaSelf {
    private import EngineElement::*;
    private import EngineProcess::*;

    action stpaSelf : Process {
        :>> id = "3f6e2a9c-7d41-4b58-9c0e-5a1b8d2f4e73";
        :>> title = "STPA on keel itself";
        :>> purpose = "Find the control gaps in keel's own workflow the way the workflow finds them in a deliverable: from the computed control structure, for every control action, the four unsafe-control-action types, each traced to a controller constraint bound to an existing control or to an Issue naming the gap - and re-run whenever the structure changes.";
        :>> cadence = Cadence::eventDriven;
        // APPLIES-WHEN: the control structure view has changed (a hook event, a workflow step, a
        // write command, a decider) since the last recorded run, or a control defect (D0278) or a
        // High Issue in the workflow itself asks what else the same controller can do wrong.
    }

    action stpa1Losses : ProcessStep {
        :>> id = "9b2c4d6e-1f3a-4c5b-8d7e-2a4c6e8f0b1d";
        :>> title = "Read the losses and hazards; add only what a new control action exposes";
        :>> triggerCondition = "A run is due (stpa5's trigger, or a control defect asks).";
        :>> actionText = "Read .tracking/architecture/engine-safety.sysml: the losses (EL) and hazards (EHZ) are the accepted frame. Add a hazard only when a control action analysed in stpa3 can lead to a loss no hazard covers; never restate one. Record the run's frame as the hazard ids it used.";
        :>> producedArtifact = "The hazard set the run analyses against (existing ids, plus any new Hazard part).";
        :>> owner = Owner::ai;
    }

    action stpa2Structure : ProcessStep {
        :>> id = "c4e6a8b0-2d4f-4e6a-9f1b-3c5e7a9b1d2f";
        :>> title = "Read the computed control structure back - never draw one";
        :>> triggerCondition = "stpa1 done.";
        :>> actionText = "Run `keel show control-structure [ROOT]` and, if a reader needs the picture, the stpa-diagram process. The controllers, control actions, feedback and process models are computed facts (D0284); the run analyses THAT list. Note the action names verbatim - they are what stpa5 records.";
        :>> producedArtifact = "The computed action list the run will walk.";
        :>> owner = Owner::ai;
    }

    action stpa3Ucas : ProcessStep {
        :>> id = "e6a8c0d2-3f5b-4a7c-9b2d-4e6a8c0d2f3a";
        :>> title = "For every control action, the four unsafe-control-action types";
        :>> triggerCondition = "stpa2 done.";
        :>> actionText = "For each computed control action consider all four UCA types - provided, not provided, wrong timing or order, stopped too soon or applied too long - in worst case, not most likely. Record each that can lead to a hazard as an UnsafeControlAction part (ucaType, context naming the computed action verbatim, statement) under .tracking/architecture/, with a dependency edge to the hazard. A type considered and found harmless is written as one line in the run record, so the reader can tell considered-and-safe from not-considered.";
        :>> producedArtifact = "UnsafeControlAction parts, each edged to a hazard; the considered-safe lines in the run record.";
        :>> owner = Owner::ai;
    }

    action stpa4Constraints : ProcessStep {
        :>> id = "a8c0e2f4-4a6c-4b8e-8c3f-5a7c9e1f3b4c";
        :>> title = "Every UCA gets a controller constraint bound to a control, or an Issue naming the gap";
        :>> triggerCondition = "stpa3 done.";
        :>> actionText = "For each UCA write a ControllerConstraint part (statement) and bind it: a dependency edge to the control in control-map.sysml (ctl* or g*) that enforces it, or - where no control exists - `keel record issue` naming the gap with a resolver, and a dependency edge from the constraint to the Issue. A constraint bound to nothing is the D0254 third failure class (an undeclared control cannot show as a gap) and is refused by the run's own read-back. Loss scenarios are written only where the constraint's mechanism is not obvious from the control it binds to.";
        :>> producedArtifact = "ControllerConstraint parts, each edged to a control or an Issue; LossScenario parts where needed.";
        :>> owner = Owner::ai;
    }

    action stpa5Trigger : ProcessStep {
        :>> id = "c0e2a4b6-5b7d-4c9f-9d4a-6b8d0f2a4c5d";
        :>> title = "Record what was analysed, so the structure can say when to run again";
        :>> triggerCondition = "stpa4 done.";
        :>> actionText = "Record the run as a Test (method analyze) whose procedureText lists `ANALYSED: <action>, <action>, ...` - the computed action names walked in stpa3 - and a passing TestResult judged against HEAD. The stpa-currency guard compares the latest run's ANALYSED list with the current computed structure and WARNS on every action no run has analysed: that warning is the re-run trigger, and it clears only by a new run that covers the action.";
        :>> producedArtifact = "The run record; the guard's silence until the structure grows.";
        :>> owner = Owner::ai;
    }

    first stpa1Losses then stpa2Structure;
    first stpa2Structure then stpa3Ucas;
    first stpa3Ucas then stpa4Constraints;
    first stpa4Constraints then stpa5Trigger;
}
