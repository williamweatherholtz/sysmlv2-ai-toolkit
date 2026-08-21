// D0175 (PROPOSED — NOT YET ACCEPTED) — D-P0b: the scaffold points at the existing release channel; distribution scope is fenced to the documented install path
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0175Accept event on sign-off.
package Decision0175 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0175 : Decision {
        :>> id = "390276a2-4d83-42b7-8cbf-078967794d18";
        :>> title = "D-P0b: the scaffold points at the existing release channel; distribution scope is fenced to the documented install path";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "A working release channel EXISTS: v0.1.0-v0.2.3 are tagged, sprint259 verified the workflow building and attaching three platform assets, sprint264 verified the v0.2.3 asset downloaded, executed, and version-matched. But nothing downstream points at it - the scaffolded pre-commit remedy says only install keel to enforce, naming no path, and keel init prints no install source. (The panel's fact-checker caught and corrected an earlier draft of this decision that repeated the stale one-stale-release claim from d0128-direction-requirements, authored 2026-08-13 and falsified the next day.) Reviewed by a 9-member adversarial panel (2 pessimists, 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) across three rounds under an honesty rule; round 3 verdict 9 ACCEPT / 0 OBJECT, uncompelled. Full evidence and finding-to-resolution map: docs/proposals/2026-08-21-production-readiness-design.md sections 10-12. NOTE per the 2026-08-14 panel precedent (.tracking/panel-critiques.sysml): the panelists are subagent contexts of the authoring model, a rigor framing, NOT independent critics - this record claims convergence, never critic-independence.";
        :>> decision = "Document the downstream install path; every scaffolded remedy text points at it; state the release-cadence/latest-version expectation. The distribution scope is FENCED: no package managers, no auto-update, no install scripting beyond documentation - each is a D0157-pattern future trigger requiring the human to reraise.";
        :>> rationale = "A loud-fail gate whose remedy names no working install path is K13 violated at the install layer - the refusal must name a path that works, or downstream users uninstall. Conversely, distribution scope famously metastasizes; the fence keeps this a documentation task, which is all the verified channel actually lacks.";
        :>> consequences = "P0's loud-fail pre-commit becomes safe to ship. The scaffold remedy text and README install pointer are the deliverables; the fence is normative for future work.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0175Accept : Test { :>> id = "617f185d-0fdd-4d8e-9e9c-35ebb519d70c"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'accepted all'. Said in response to the presented queue of fifteen proposed Decisions - D0174, D0175, D0176, D0177, D0178, D0179, D0180, D0181, D0182, D0183, D0184, D0185, D0186, D0187, D0188 - so 'all' is that queue and this acceptance covers this Decision as one of the fifteen. The open fork stated in the proposal (section 8-A, strict default for provably empty directories) was named to them alongside the queue with the invitation to object on D0174; they accepted without objection, so D0174 is accepted AS WRITTEN including its stated strict-for-empty default."; }
    part d0175AcceptR1 : TestResult { :>> id = "3f5e86d2-63a1-4071-9842-f7f7d6f8b55c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10b66bd"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
}
