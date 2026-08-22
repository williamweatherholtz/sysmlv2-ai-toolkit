// D0180 (status: the `status` field below is the truth) — D-PM: enforcement effect is measured; advisory-to-blocking promotions cite the measurements; N defaults to 3 sprints
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0180Accept event on sign-off.
package Decision0180 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0180 : Decision {
        :>> id = "150ccca0-ec49-441f-afe1-0ef97f3e1962";
        :>> title = "D-PM: enforcement effect is measured; advisory-to-blocking promotions cite the measurements; N defaults to 3 sprints";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0128's accepted sequence begins 'prove the in-loop gate (measure)' and D0130's consequences record that measurement as NOT delivered - the plan's later steps were built while step 1 stayed missing. issue093 shows the cost: the Stop hook was dead for a full sitting and nothing in the work product revealed it; if enforcement being silently off is indistinguishable from on, the hypothesis that the hooks buy nothing is live and untested. Reviewed by a 9-member adversarial panel (2 pessimists, 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) across three rounds under an honesty rule; round 3 verdict 9 ACCEPT / 0 OBJECT, uncompelled. Full evidence and finding-to-resolution map: docs/proposals/2026-08-21-production-readiness-design.md sections 10-12. NOTE per the 2026-08-14 panel precedent (.tracking/panel-critiques.sysml): the panelists are subagent contexts of the authoring model, a rigor framing, NOT independent critics - this record claims convergence, never critic-independence.";
        :>> decision = "A machine-local outcome ledger (.keel/metrics/*.jsonl, gitignored class; the emit plumbing ships in P0, this Decision owns the schema freeze, capture breadth, and analysis): hook fires/blocks/overrides/red-yields keyed by session id and event type - this fire-ledger is also the single instrumentation path the P0/P5 hooks-actually-fired checks read - plus launcher-run durations and turn counts. Adherence trend uses the already-tracked ProcessDefect-marked issues. Entirely local until a consumer for tracked summaries is named (D0144). Every advisory-to-blocking promotion (P0 guided-to-strict, P1 pattern promotions) cites at least N sprints of this evidence; N = 3 unless amended here.";
        :>> rationale = "Promotions decided by gut recreate the issue076/081 over-strict dynamic the repo has already paid for; promotions decided by data settle future scope fights structurally. The launcher-adoption ratio-shift claim is a hypothesis this ledger tests (fraction of EXECUTE/RECORD work launcher-initiated per sprint). The PM baseline review also watches the dirty-tree-refusal rate (the launch friction tripwire).";
        :>> consequences = "Delivers D0128 step 1. The production-ready criterion gains a measured clause: metrics exist for >= N sprints and at least one promotion or non-promotion decision cites them. Charters pmMeasure.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0180Accept : Test { :>> id = "90f3869e-29aa-417c-8d66-4ec1c07acda0"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'accepted all'. Said in response to the presented queue of fifteen proposed Decisions - D0174, D0175, D0176, D0177, D0178, D0179, D0180, D0181, D0182, D0183, D0184, D0185, D0186, D0187, D0188 - so 'all' is that queue and this acceptance covers this Decision as one of the fifteen. The open fork stated in the proposal (section 8-A, strict default for provably empty directories) was named to them alongside the queue with the invitation to object on D0174; they accepted without objection, so D0174 is accepted AS WRITTEN including its stated strict-for-empty default."; }
    part d0180AcceptR1 : TestResult { :>> id = "f4650df0-b744-4048-935c-bfe9ea267ace"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10b66bd"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
}
