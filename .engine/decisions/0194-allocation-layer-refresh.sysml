// D0194 (PROPOSED — NOT YET ACCEPTED) — D0194: retire the June component allocations; charter-chains plus the code registry become the sanctioned allocation mechanism, with a coverage lens
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0194Accept event on sign-off.
package Decision0194 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0194 : Decision {
        :>> id = "c21156dc-bf83-4311-adb6-e82d6202a367";
        :>> title = "D0194: retire the June component allocations; charter-chains plus the code registry become the sanctioned allocation mechanism, with a coverage lens";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "All five R1 panelists (systems engineering, architecture, aerospace, robotics, automotive) independently converged on the same defect: function-to-component allocation is a two-island ruin. The 22-26 allocate edges all date from 2026-06 and cover only the era-1 SRs; the 8 Components they target describe RETIRED implementations (queryStack still calls query.py 'the single source of project state' though it was retired at D0074 and the file is gone); the allocIfaceGate confirmation ('Every SystemRequirement has an allocate edge') was true at abb033a in June and is false at HEAD (26 edges vs 102 live SRs) with nothing re-firing the obligation - the K10 reprocess problem applied to the engine's own architecture phase; and the CodeElement registry (the surface that actually carries risk judgments) participates in zero typed edges. Worst of all, keel trace-need actively SERVES the stale component closure as current truth - a dual-truth defect (EHZ8 in the new engine-safety list).";
        :>> decision = "PROPOSED, option (b) of the panel's stated fork, because the evidence favors it: (1) SUPERSEDE the June Component set and its allocate edges (kept readable as history, #Supersede from this Decision - the D0108 idiom); (2) the sanctioned SR-to-code allocation mechanism becomes the charter chain (SR -> #CharteredBy work -> delivered code) PLUS explicit SR-to-CodeElement allocate edges for the standing high-consequence units (the registry, now 17 elements after the panel's completions); (3) an allocation-coverage leg joins tier-satisfaction (same structural-floor semantics as the satisfy and verify legs), so silent coverage collapse - 100% to 25% unnoticed for two months - cannot recur; (4) trace-need recomputes over the live mechanism or states plainly that its component layer is historical. The rejected alternative (a) - refresh the 8 Components to the realized 34-module architecture and maintain both layers - was rejected because it creates a second hand-maintained component vocabulary alongside the registry, which is the two-island defect reborn.";
        :>> rationale = "Four of five panelists flagged this in their minimal flip sets; the systems engineer named the current state 'worse than absence' because a live view serves the falsehood. The repo's own dead-schema policy (re-add THEN, with an instance) and D0105's one-home rule both point at (b): one allocation surface, computed coverage, history superseded not rewritten.";
        :>> consequences = "IF ACCEPTED: the stale components resolve as superseded; ~15 allocate edges are authored SR->CodeElement for the high-consequence set; tier-satisfaction gains its third leg; trace-need stops serving June as now. COST: the allocation-coverage floor will read LOW at first (honest), and authoring the edges is judgment work chartered to a sprint. IF REJECTED: the human names option (a) or another shape; the stale layer stays recorded as EHZ8's open instance either way.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0194Accept : Test { :>> id = "92dd2ab5-9e17-43fb-aa87-2363ada593a9"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'looks great. proceed to implement' - said in direct response to the panel-consensus report whose closing line asked them to sign D0194/D0195 so the chartered work could execute. Covers this Decision as one of that named pair. D0192 is NOT covered: accepting it requires choosing option A or B, which a general go does not do - it stays proposed until they name one."; }
    part d0194AcceptR1 : TestResult { :>> id = "e298fd04-9f12-4db9-a326-0aec00febf46"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5207cbc"; :>> judgedAt = "2026-08-22"; :>> judgedBy = "wweatherholtz"; }
}
