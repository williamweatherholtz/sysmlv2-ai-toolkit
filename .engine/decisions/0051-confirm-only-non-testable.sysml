// =====================================================================
// D0051 — confirm-only-non-testable: the per-sitting review confirms ONLY
// non-test-verifiable items (decisions/direction). Test/inspect/analysis-backed
// programmatic items are self-evidencing and need no human confirmation.
// Accepted 2026-06-16 by wweatherholtz ("why would they need confirmation? do we
// not have real tests for these programmatic items?").
// =====================================================================
package Decision0051 {
    private import EngineWork::*;

    part d0051 : Decision {
        :>> id = "12d3e4f5-0617-4239-376f-1324a5b6c7e2";
        :>> title = "D0051: per-sitting review confirms only non-test-verifiable items; tested items self-evidence";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0049 routed the single human gate to the per-sitting sprint review. At the first such review the human asked: why confirm at all, when the programmatic items have real automated tests? Correct — for method=test/inspect/analysis items the evidence IS the automated run (cargo test, clippy, sysmlv2 validate, ceremony guard, parity_check); a human 'yes' adds nothing and is the kind of empty gate D0049 was already removing. The only confirmation-worthy class is non-test-verifiable judgment: decisions/direction (which framework, whether to promote Rust, ceremony model), where the evidence IS the human's word (D0016).";
        :>> decision = "Sharpens D0049: the per-sitting review requires human confirmation ONLY for non-test-verifiable items — Decisions and direction/acceptance calls (method=confirmation, evidence = the human's attestation, D0016). Test/inspect/analysis-backed items are self-evidencing: their automated runs are the evidence, recorded autonomously (AI), and need NO human confirmation; a sitting whose work is entirely test-backed + whose decisions were accepted inline has NOTHING outstanding to confirm.";
        :>> rationale = "For method=test/inspect/analysis items the evidence IS the automated run (cargo test, clippy, sysmlv2 validate, ceremony guard, parity_check); a human 'yes' adds nothing and is the kind of empty gate D0049 was already removing. The only confirmation-worthy class is non-test-verifiable judgment — decisions/direction (which framework, whether to promote Rust, ceremony model) — where the evidence IS the human's word (D0016). User asked at the first such review: 'why would they need confirmation? do we not have real tests for these programmatic items?'";
        :>> decisionText = "Sharpens D0049: the per-sitting review requires human confirmation ONLY for non-test-verifiable items — Decisions and direction/acceptance calls (method=confirmation, evidence = the human's attestation, D0016). Test/inspect/analysis-backed items are self-evidencing: their automated runs are the evidence, recorded autonomously (AI), and need NO human confirmation. A sitting whose work is entirely test-backed + whose decisions were accepted inline has NOTHING outstanding to confirm — the review is a content recap. The review skill surfaces, separately, (a) the recap of tested work [no confirmation] and (b) any decisions/direction still awaiting acceptance [the only confirmation ask]. Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "sprint-review skill updated: Phase 4 asks for confirmation only on outstanding non-test-verifiable items; if none, it states 'nothing to confirm' and the sitting review is a recap. CLAUDE.md confirmation-discipline updated. Decisions are still accepted explicitly (often inline as they are made); test-backed sprint work never waits on a human. This removes the empty-gate failure mode the human flagged.";
    }
}
