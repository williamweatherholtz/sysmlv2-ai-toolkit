// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded; a human review
// discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks
// on the file being repaired.
package Obligation3b91c51a {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineRelationships::*;

    part obligation3b91c51a : Issue {
        :>> id = "3b91c51a-77ff-4d8a-a6e4-c0144b99e63d";
        :>> title = "turn gate yielded while red - the tree needs a correction pass";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> description = "The Stop hook's second red pass yielded (loop avoidance). Session: b43aa4fc-84f7-4c3d-bc35-61eccafbbda8. First problem at yield: keel guard: [sprint-closure] sprint617_suiteVerdictIsShellIndependent.sysml: task `storySuiteVerdictIsShellIndependent` has no TestResult, but work has moved on to a later sprint — an unstamped task is served as READY forever, so finished work is indistinguishable from open work (D0260/sprint483) [cli-surface-declared] `reject` is an authored CliCommand fact with no entry in cli_facts.rs - the help cannot describe it [cli-surface-declared] `reject` is declared as a command but nothing dispatches it. Discharge: make the tree green and triage this obligation with a #Resolves edge.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }

    // Discharged by D0395: all three problems at the yield were transient states of one in-progress
    // build - sprint617 stamped storySuiteVerdictIsShellIndependent (the sprint-closure line), and
    // sprint618 added `reject` to cli_facts.rs and the dispatch (the two cli-surface-declared lines).
    // The tree is green at the commit carrying this edge; keel guard is clean.
    #Resolves dependency from d0395 to obligation3b91c51a;
}
