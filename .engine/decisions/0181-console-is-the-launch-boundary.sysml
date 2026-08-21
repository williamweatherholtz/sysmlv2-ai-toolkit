// D0181 (PROPOSED — NOT YET ACCEPTED) — D-P5a: the console is the launch boundary for routine EXECUTE and RECORD work
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0181Accept event on sign-off.
package Decision0181 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0181 : Decision {
        :>> id = "c25e4dd4-a9dd-4c4c-90b9-8a6130cc93cc";
        :>> title = "D-P5a: the console is the launch boundary for routine EXECUTE and RECORD work";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0109 accepted action-space constraints realized as the maturation of keel serve into a process-launcher with generation-time output validation, and the levers analysis (F6) identified the interaction boundary as the ONE place the parse-first residual (issue059) becomes structurally enforceable: input, visible route, approve, execute - un-routed actions simply not offered. The bridge already has the skeleton: closed action set, is_launchable check, approve-before-execute, claude -p spawn with bounded turns. Reviewed by a 9-member adversarial panel (2 pessimists, 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) across three rounds under an honesty rule; round 3 verdict 9 ACCEPT / 0 OBJECT, uncompelled. Full evidence and finding-to-resolution map: docs/proposals/2026-08-21-production-readiness-design.md sections 10-12. NOTE per the 2026-08-14 panel precedent (.tracking/panel-critiques.sysml): the panelists are subagent contexts of the authoring model, a rigor framing, NOT independent critics - this record claims convergence, never critic-independence.";
        :>> decision = "Routine EXECUTE and RECORD work starts as a declared process launch from the console (VIEW needs no launch - reads are free; CHANGE and design work remain interactive). Per-process input forms generate from the model (D0117 substrate: /api/schema, /api/surfaces). This is a RECOMMENDATION contingent on human review, extends D0109/D0152, and leaves D0093's dual-surface invariant unamended: truth and automation stay CLI/JSON, single binary, embedded console. The launcher-first ratio claim is a measured hypothesis (D-PM), not an assertion. The run-branch amendment to commit-directly-to-main is NOT recorded here - it belongs to the worktree trigger Decision, recorded only if that DoR is met.";
        :>> rationale = "Fresh narrow context per task (lever 2), a closed action space at the start of work (lever 1), and run-exit verification (lever 3 at run granularity) are what the boundary buys; what it does not buy is stated honestly - inside a spawned run the agent is a full harness, and mid-run forcing comes from the shipped hooks and denials, which is why D-P0a/D-P1a precede this.";
        :>> consequences = "The console becomes the paved road for routine work with interactive sessions preserved for design. Charters p5Forms.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0181Accept : Test { :>> id = "d164f94f-729e-4b96-896e-5b19e16ebd57"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'accepted all'. Said in response to the presented queue of fifteen proposed Decisions - D0174, D0175, D0176, D0177, D0178, D0179, D0180, D0181, D0182, D0183, D0184, D0185, D0186, D0187, D0188 - so 'all' is that queue and this acceptance covers this Decision as one of the fifteen. The open fork stated in the proposal (section 8-A, strict default for provably empty directories) was named to them alongside the queue with the invitation to object on D0174; they accepted without objection, so D0174 is accepted AS WRITTEN including its stated strict-for-empty default."; }
    part d0181AcceptR1 : TestResult { :>> id = "5595d23d-a1fd-4c38-8d31-7458a0bcf1d6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10b66bd"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
}
