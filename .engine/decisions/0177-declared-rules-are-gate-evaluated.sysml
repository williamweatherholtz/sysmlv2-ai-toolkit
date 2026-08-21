// D0177 (PROPOSED — NOT YET ACCEPTED) — D-P1b: declared rules are gate-evaluated - the modularity keystone
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0177Accept event on sign-off.
package Decision0177 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0177 : Decision {
        :>> id = "49789780-ff2b-4fc2-b79f-90d38b891ad2";
        :>> title = "D-P1b: declared rules are gate-evaluated - the modularity keystone";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "keel rules evaluates declared EdgeRule/ElementRule instances generically, but no hook, pre-commit, or CI step runs it - so the ONLY extension point a downstream project has for adding a control without forking the compiled binary (GUARD_NAMES is a compiled array) is advisory-only. Five guards are rule-sourced; a project's own declared rule gates nothing. Reviewed by a 9-member adversarial panel (2 pessimists, 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) across three rounds under an honesty rule; round 3 verdict 9 ACCEPT / 0 OBJECT, uncompelled. Full evidence and finding-to-resolution map: docs/proposals/2026-08-21-production-readiness-design.md sections 10-12. NOTE per the 2026-08-14 panel precedent (.tracking/panel-critiques.sysml): the panelists are subagent contexts of the authoring model, a rigor framing, NOT independent critics - this record claims convergence, never critic-independence. The panel's steelman defender flagged this single item as the highest-leverage line in the plan and demanded it carry its own Decision so it cannot be silently dropped.";
        :>> decision = "Wire keel rules into hook stop, the pre-commit gate, and CI: blocking-severity rules gate, warning-severity rules report. Fix hook_stop bypassing the activation filter (it iterates GUARD_NAMES via run_one, so a deactivated process's guard still blocks turns while keel guard correctly skips it - latent until any project writes an activation.toml).";
        :>> rationale = "This is what makes the modular adjective true: a project adds a blocking control by declaring a rule - no Rust, no fork. It is also the prerequisite for rules traveling in process units (D-P3a/P4b) meaning anything at the destination.";
        :>> consequences = "Downstream extensibility without forking; the activation-filter bug dies before it goes live. Charters p1RulesWired.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0177Accept : Test { :>> id = "4b88c7d6-c3fe-457d-bbcb-4d31b0a1858b"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'accepted all'. Said in response to the presented queue of fifteen proposed Decisions - D0174, D0175, D0176, D0177, D0178, D0179, D0180, D0181, D0182, D0183, D0184, D0185, D0186, D0187, D0188 - so 'all' is that queue and this acceptance covers this Decision as one of the fifteen. The open fork stated in the proposal (section 8-A, strict default for provably empty directories) was named to them alongside the queue with the invitation to object on D0174; they accepted without objection, so D0174 is accepted AS WRITTEN including its stated strict-for-empty default."; }
    part d0177AcceptR1 : TestResult { :>> id = "376a3cbd-38d9-4ea9-943c-a2e50d4607bb"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10b66bd"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
}
