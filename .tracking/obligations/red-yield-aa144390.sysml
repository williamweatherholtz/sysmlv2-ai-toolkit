// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded; a human review
// discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks
// on the file being repaired.
package Obligationaa144390 {
    private import EngineElement::*;
    // `Issue` is defined in EngineWork (schema/core/work.sysml:70). This import was added by hand
    // here, because the recorder's template omitted it and RECORDING an obligation therefore made
    // the tree INVALID (issue274). The template now carries it, and the property is held by
    // `recording_an_obligation_keeps_the_tree_valid` rather than by this repair.
    private import EngineWork::*;
    private import EngineRelationships::*;

    // TRIAGED: the red state was my own test contamination, not a defect in the tree. A rename
    // verification for issue273 ran in the REPOSITORY ROOT instead of its temp fixture, so
    // `.engine/rules/rules.sysml` was replaced by a `rules2.sysml` carrying 0 blocking rules
    // where HEAD has 8 - a staged downgrade of every blocking rule in this project. Restored
    // from HEAD and the artifact deleted; `git diff HEAD -- .engine/rules/` is empty.
    //
    // WORTH RECORDING: the guard that caught it is the one being repaired in the same commit.
    // With the old `MD` filter the rename would have shown NOTHING; the new `MDR` filter is
    // what made the Stop hook go red and produce this obligation. The control found its own
    // author's contamination minutes after being fixed.
    part obligationaa144390 : Issue {
        :>> id = "aa144390-8ba2-4794-a4d6-2bd7c7387471";
        :>> title = "turn gate yielded while red - the tree needs a correction pass";
        :>> createdAt = "2026-08-26"; :>> createdBy = "claudeFable5";
        :>> description = "The Stop hook's second red pass yielded (loop avoidance). Session: 938f1981-92ec-414a-9681-d119cb28134f. First problem at yield: keel guard: [process-change] locked file(s) changed (.engine/rules/rules.sysml, .engine/rules/rules2.sysml) with NO co-committed process-change Decision (a #ProspectiveChange/#SafetyChange-marked .engine/decisions/*.sysml). HARD LOCK: process definitions (D0070) AND the enforcement surface — guard source, hook config, CI workflows (D0209 clause 2) — may change only with a human-signed Decision, because a silently self-modified control is the issue236 self-modification class. Record one with `k. Discharge: make the tree green and triage this obligation with a #Resolves edge.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcRenameIsAChange to obligationaa144390;
}
