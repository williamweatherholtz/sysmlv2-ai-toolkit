// =====================================================================
// ProprietyRound2 — the panel over the FEDERATION TEST PLATFORM DESIGN (D0262), convened BEFORE
// implementation at the human's instruction (st040: design, panel, implement, re-panel).
//
// INDEPENDENCE, stated per the process's own rule (cpIndependence): the design under assessment was
// authored by the assessor (claudeOpus5) minutes earlier, so every `sound` verdict here is a
// TESTIMONY and not a receipt (D0080/D0232). The stance is eliminative - argue against before
// crediting - and the round's own record is the evidence it still convicts: FOUR of five lenses
// return a STANDING defeater against a design its author had just finished defending, and one of
// those (pf44) is against a mitigation the design explicitly claimed.
//
// Round 1 assessed shipped controls (pf01-pf40). This round assesses a DESIGN, which is the cheaper
// place to convict: every finding below is discharged by editing a Decision rather than by
// rewriting a test suite that already exists.
// =====================================================================
package ProprietyRound2 {
    private import EngineElement::*;
    private import EnginePropriety::*;

    part pf41 : ProprietyFinding {
        :>> id = "c1a20002-0830-4c1e-9f10-000000000041";
        :>> title = "federation-suite design x overfit";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "design:federation-suite";
        :>> lens = ProprietyLens::overfit;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "F1 is stated generally ('a scaffolded project can make its first commit') but its only case is the instance reconnaissance happened to find: PATH DEPTH. The class is 'a path the host filesystem or git handles differently from the developer's', and depth is one member. A path component carrying a NON-ASCII byte is another member of the same class, and it defeated a control in THIS repository six days ago (issue273, second blindness: git C-quotes such paths and the parse policing them went blind). A path with a SPACE is a third. Testing the found member and calling the class covered is the overfit this lens exists to catch.";
        :>> defeater = "STANDS: a project at a path containing a non-ASCII or spaced component. The design must parameterise F1 over path SHAPE, not only length - the cost is one table, and the precedent for the omission being expensive is in this repo's own issue log.";
    }

    part pf42 : ProprietyFinding {
        :>> id = "c1a20002-0830-4c1e-9f10-000000000042";
        :>> title = "federation-suite design x classCoverage";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "design:federation-suite";
        :>> lens = ProprietyLens::classCoverage;
        :>> verdict = ProprietyVerdict::unsound;
        :>> rationale = "The class is 'a downstream project works in a federation OVER TIME'. The eight claims cover a project at BIRTH - init, first commit, first import, first publish - and not one of them covers the project's SECOND WEEK, when the engine has moved and the project has not. That is not a peripheral member: version skew is the most likely real federation failure by a wide margin, it is the failure keel migrate and the BINDING pin (D0251) exist for, and this repository has already been blocked outright TWICE by exactly it (issue089, issue090, both downstream projects broken by an engine that moved underneath them). A suite that asserts a federation works while never advancing the engine tests the easy day.";
        :>> defeater = "DEMONSTRATED, not merely possible: issue089 and issue090 are the class occurring in the field with no suite to catch them. Routed to work - the design gains F9 (a project inited on engine N is migrated to engine N+1: the pin re-stamps, the tree gates green, and a SKEWED binary refuses writes before migration and accepts them after).";
    }

    part pf43 : ProprietyFinding {
        :>> id = "c1a20002-0830-4c1e-9f10-000000000043";
        :>> title = "federation-suite design x changeRobustness";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "design:federation-suite";
        :>> lens = ProprietyLens::changeRobustness;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "Most claims are assertions about ABSENCE (own decisions empty, no keel Needs in the project's model, guards not yet active), and absence assertions are the ones that rot silently: if the engine legitimately MOVES .engine/decisions/ or renames the reference directory, F3 keeps passing while measuring nothing, because the thing it looks for is still not where it looks. The suite would then report that purpose separation holds on a tree where the question is no longer being asked.";
        :>> defeater = "STANDS wherever a claim is keyed on a FILE PATH. Partially mitigable and the mitigation is cheap: assert on COMMAND OUTPUT (`keel business`, `keel activation`, `keel process show`) rather than directory layout, since the command is the contract and the layout is an implementation detail. Where a path assertion is unavoidable, it must be paired with a positive assertion that the path EXISTS and is populated, so a moved directory fails loudly instead of passing emptily.";
    }

    part pf44 : ProprietyFinding {
        :>> id = "c1a20002-0830-4c1e-9f10-000000000044";
        :>> title = "federation-suite design x circumventability";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "design:federation-suite";
        :>> lens = ProprietyLens::circumventability;
        :>> verdict = ProprietyVerdict::unsound;
        :>> rationale = "The design's own network policy is the hole. F6 and F7 split into a pure half and a live half, with the live half 'SKIPPED-with-a-statement' when no token is present. A statement is not a control - it is the same manual-vigilance answer D0047 rejects - and the predictable steady state is that CI has no token, the live half never runs, and the suite reports GREEN while the only assertion that touches the real GitHub contract has never once executed. The design names this risk in its own text and then adopts the arrangement that realises it, which is worse than not noticing: it converts a known hazard into a documented one.";
        :>> defeater = "STANDS as designed. Discharged only by making the skip a DISTINCT OUTCOME rather than a shade of pass: the suite must report which claims were exercised live, and a context that asserts federation coverage while the live half never ran must FAIL. The cheap honest form is a manifest of executed claims that a check reads - the same shape as the no-op coverage floor (D0261/issue310), which was built precisely because an enumerated table silently outgrown reads as complete.";
    }

    part pf45 : ProprietyFinding {
        :>> id = "c1a20002-0830-4c1e-9f10-000000000045";
        :>> title = "federation-suite design x measurability";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "design:federation-suite";
        :>> lens = ProprietyLens::measurability;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "Seven claims are objectively measurable - a commit succeeds or does not, a number is d0001 or is not, a guard fires or does not. F3 is not. 'Is the project confused about whose purpose it serves' names a conceptual state, and the design operationalises it as two file-level proxies (own model empty, reference namespaced). A project can pass both while its CLAUDE.md, its skills and its authored vocabulary are thoroughly keel's - which is the actual confusion the question is about, and the one the engine-model-versus-deliverable invariant exists to prevent.";
        :>> defeater = "STANDS for the conceptual claim; the proxies measure NAMESPACE HYGIENE and should say so rather than claim more. A strictly stronger observable is available and belongs in the design: keel's own DOMAIN VOCABULARY must not appear in the new project's AUTHORED surface - the scaffold may ship keel's rationale as reference, but a fresh project whose own items talk about guards, sprints-of-keel or the engine's needs has inherited a purpose it was never given. That is text-searchable, so the claim can be made measurable rather than left as judgment.";
    }
}
