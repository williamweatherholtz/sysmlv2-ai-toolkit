// D0195 (PROPOSED — NOT YET ACCEPTED) — D0195: controls get model identities - hazards, invariants, and the guards that discharge them become one computable trace
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0195Accept event on sign-off.
package Decision0195 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0195 : Decision {
        :>> id = "6512342a-ee1b-4d56-bc5a-524c3313ea54";
        :>> title = "D0195: controls get model identities - hazards, invariants, and the guards that discharge them become one computable trace";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Panel R1's aerospace critic (finding 2, in their minimal flip set): the 45 guards are the engine's control layer but exist only as Rust names, with no typed edge from any guard to the K-invariant or hazard it discharges - so the core assurance cross-check, WHICH FAILURE CONDITIONS HAVE NO PREVENTING OR DETECTING CONTROL, cannot be asked of the model. The first half is now authored: .tracking/architecture/engine-safety.sysml carries the severity-ordered loss/hazard list (4 losses, 9 hazards) with #DerivedFrom edges from the constraining K-invariants and each hazard's standing controls named in its statement text - readable, but the guard half of the trace is prose, not edges.";
        :>> decision = "PROPOSED: (1) each enforced guard gains a model identity - extending the D0139(D) constraint-def surface in .engine/rules/guard-constraints.sysml to cover all guards, not only the unit-owned ones; (2) typed edges from each guard's constraint def to the hazard(s)/invariant(s) it discharges (the edge content is exactly what the hazard statements already name in prose); (3) a computed view (keel controls or a hardening axis) reporting hazards with no live control and controls tied to no hazard - the two-way diff, same shape as guard 45's event coverage; (4) chartered as dcControlMapView, delivered under this Decision when signed. ROUND-2 ADDITIONS (the panel unanimously accepted with these folded into this proposal, edited pre-signature by its author - a proposed decision is a draft until signed): (5) the registry control-loop edges completed (the new elements carry roles but no #Controls/#Feedback edges; the arch stpa-inputs flag on ceSyncIntegration gets its disposition) - AERO round-2 finding 2; (6) degraded-mode columns: the view enumerates enforcement-point x failure-mode -> open/closed/announced from the behaviors that already exist in code - ROBO round-2 finding 3; (7) an incident-lineage leg: a new High or dataLoss-class Issue referencing no hazard WARNS, so the loss/hazard list cannot go stale by the very EHZ8 mechanism it documents - AUTO round-2 finding 1; (8) EHZ9 gets its requirement anchor: an SR/invariant (no outward publish leaves the boundary ungated) verify-linked to the sprint-409 pipeline evidence and edged to ehz9, so the release-gate controls trace to a standing requirement rather than to nothing - AERO round-2 finding 1, and the first row the two-way view would otherwise report.";
        :>> rationale = "D0047's spirit at the architecture level: a control that cannot be traced to what it controls is vigilance, not assurance. The panel's aerospace lens is the right one here - this is the single ARP4754A mechanic that transfers to a work-tracking engine at near-zero ceremony, because every ingredient (constraint defs, markers, computed views) already exists; only the edges and one view are missing.";
        :>> consequences = "IF ACCEPTED: guard-constraints.sysml roughly triples (one line per guard), ~20 edges land, one view joins the lens set; the uncovered-hazard question becomes computable. COST: a new guard must add its constraint def and edge - one more authoring step, caught by the same two-way diff if forgotten. IF REJECTED: the loss/hazard list stands alone as readable-but-not-computable, which is still more than existed before the panel.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0195Accept : Test { :>> id = "d17e4cdb-92fc-4033-b140-e8affb057c4a"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'looks great. proceed to implement' - said in direct response to the panel-consensus report whose closing line asked them to sign D0194/D0195 so the chartered work could execute. Covers this Decision as one of that named pair. D0192 is NOT covered: accepting it requires choosing option A or B, which a general go does not do - it stays proposed until they name one."; }
    part d0195AcceptR1 : TestResult { :>> id = "c07748fc-3e46-4fd7-8ad0-d963861effc1"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5207cbc"; :>> judgedAt = "2026-08-22"; :>> judgedBy = "wweatherholtz"; }
}
