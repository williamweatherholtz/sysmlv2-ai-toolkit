// =====================================================================
// D0073 — the per-sitting sprint review (D0049's single human gate) is RECORDED as a
// confirmation TestResult naming the sitting's sprints, written only on the human's explicit
// attestation. Resolves issue023 (the human gate had no auditable artifact).
// Accepted 2026-06-18 by wweatherholtz ("issue023 - yes confirmed").
// =====================================================================
package Decision0073 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0073 : Decision {
        :>> id = "63940516-2c7d-4e8f-a073-516273849653";
        :>> title = "D0073: per-sitting sprint review = a confirmation TestResult naming the sitting's sprints, on explicit human attestation (resolves issue023)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0049 made per-sprint closeOut/retro autonomous and defined the per-SITTING sprint review as the single human gate — but issue023 found that gate has no recorded artifact: it has been satisfied informally (the human reviewing in conversation), never as a tracked confirmation tied to a sitting's content. So there is nothing auditable attesting a sitting was reviewed/accepted.";
        :>> decision = "Record each per-sitting review as a confirmation TestResult (a `sittingReviewR<n>` : TestResult, outcome=pass, judgedBy=<human>, judgedAt, judgedAgainst=<commit>) whose procedureText NAMES the sprints in the sitting it accepts. It is written ONLY on the human's explicit attestation of that specific content (D0016/D0051 — never inferred from 'continue', from the work being done, or from accepting a different thing). A sitting-review record lives in a delivery file (or a dedicated sitting file). OPTIONAL follow-on: a guard flagging a run of committed sprints with no covering sitting-review record (makes the gate auditable, like the other guards).";
        :>> rationale = "The human gate must leave an auditable trace like every other verification (provenance: who/when/against-what); informal in-conversation review is real but unrecorded, so the model cannot show a sitting was accepted. Naming the sprints in the record makes coverage checkable. ALTERNATIVES: (A) keep it informal — rejected: no artifact, fails the audit (issue023); (B) auto-record on any 'continue'/acceptance — rejected: violates D0016/D0051 (confirmation must be explicit + specific); (C CHOSEN) explicit-attestation confirmation TestResult naming the sitting. CRITERIA: auditable x explicit-only x minimal.";
        :>> consequences = "sittingReviewRecord backlog task resolved by this mechanism; implementation = the record form + the optional coverage guard, a small sprint. The CURRENT sitting (sprints 38-44 + D0070/D0071/D0072-D0075) is NOT yet reviewed — its sitting-review record awaits the human's explicit 'I reviewed and accept these sprints' attestation; until then it is correctly outstanding. issue023 closed by this disposition. The audit's sitting-review dimension (query.py audit) can add a precise check once the record form exists (currently tracked, not auto-detected).";
    }

    verification d0073Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a073-516273849654";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0073 (per-sitting review = an explicit-attestation confirmation TestResult naming the sitting's sprints) via 'issue023 - yes confirmed' on 2026-06-18. NOTE: this accepts the MECHANISM; it is NOT the sitting-review attestation for sprints 38-44, which remains outstanding pending an explicit content sign-off.";
    }
    part d0073AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a073-516273849655";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "909764e";
        :>> judgedAt = "2026-06-19";
        :>> judgedBy = "wweatherholtz";
    }
}
