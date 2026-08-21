// D0184 (PROPOSED — NOT YET ACCEPTED) — D-P3a: rules are attributable to process units; the keystone lock covers skills and contracts
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0184Accept event on sign-off.
package Decision0184 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0184 : Decision {
        :>> id = "8706a2eb-f415-4d5f-9b25-aa1dd6d59761";
        :>> title = "D-P3a: rules are attributable to process units; the keystone lock covers skills and contracts";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "unit.rules is hardcoded empty (activation.rs) because rules live in shared files with no rule-to-owning-process association - so a process unit can never carry its declared rules across export/import, which is exactly the prose-lands-teeth-missing failure the portable goal names. Separately, the D0070 keystone lock covers only .engine/processes/ and .engine/workflows/ - a skill body or an activation contract is as much process definition as a process file, and both are ungoverned. Reviewed by a 9-member adversarial panel (2 pessimists, 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) across three rounds under an honesty rule; round 3 verdict 9 ACCEPT / 0 OBJECT, uncompelled. Full evidence and finding-to-resolution map: docs/proposals/2026-08-21-production-readiness-design.md sections 10-12. NOTE per the 2026-08-14 panel precedent (.tracking/panel-critiques.sysml): the panelists are subagent contexts of the authoring model, a rigor framing, NOT independent critics - this record claims convergence, never critic-independence.";
        :>> decision = "Two items only (the full declarative-controls migration stays deferred as P3b): (1) a per-process rule attribution - the declared-rule-to-owning-unit association - which is P4b's actual prerequisite; (2) extend the keystone lock to .engine/skills/ and .engine/contracts/.";
        :>> rationale = "The panel's reducer tested whether guard-names-only handshaking could cover the portable adjective and found it cannot once D-P1b makes declared rules the extension mechanism: a unit exporting rules=[] ships prose without teeth by construction. The keystone extension is cheap and closes a real governance hole.";
        :>> consequences = "P4b becomes buildable; skill and contract changes join the same change discipline as process files. Charters p3aRuleAttribution and p3aKeystoneExtension.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0184Accept : Test { :>> id = "60c20504-77b2-47ad-ac13-ea75c21647c0"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'accepted all'. Said in response to the presented queue of fifteen proposed Decisions - D0174, D0175, D0176, D0177, D0178, D0179, D0180, D0181, D0182, D0183, D0184, D0185, D0186, D0187, D0188 - so 'all' is that queue and this acceptance covers this Decision as one of the fifteen. The open fork stated in the proposal (section 8-A, strict default for provably empty directories) was named to them alongside the queue with the invitation to object on D0174; they accepted without objection, so D0174 is accepted AS WRITTEN including its stated strict-for-empty default."; }
    part d0184AcceptR1 : TestResult { :>> id = "d890043a-34ab-45ab-9d09-6c131df43ca6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10b66bd"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
}
