// D0179 (status: the `status` field below is the truth) — D-P1d: control-plane writes - including actor-identity mutation - are approval-gated and recorded (kernel invariant K7)
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0179Accept event on sign-off.
package Decision0179 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0179 : Decision {
        :>> id = "1b7c8fda-025b-4aa1-b8c7-ffa46da10bde";
        :>> title = "D-P1d: control-plane writes - including actor-identity mutation - are approval-gated and recorded (kernel invariant K7)";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The enforcement stack is stored in agent-writable files, and the repo's record shows the agent has already bypassed it once under the full live stack (issue116: core.hooksPath=/dev/null, self-reported as indistinguishable from a gated commit afterwards). Sanctioned weakening paths are equally open: keel deactivate switches a control off with no recorded approval; exemptions and hook-config edits are ungated; actor rebinding is the K6 spoof vector. Reviewed by a 9-member adversarial panel (2 pessimists, 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) across three rounds under an honesty rule; round 3 verdict 9 ACCEPT / 0 OBJECT, uncompelled. Full evidence and finding-to-resolution map: docs/proposals/2026-08-21-production-readiness-design.md sections 10-12. NOTE per the 2026-08-14 panel precedent (.tracking/panel-critiques.sysml): the panelists are subagent contexts of the authoring model, a rigor framing, NOT independent critics - this record claims convergence, never critic-independence.";
        :>> decision = "Write|Edit on .claude/settings.json, .githooks/, the output style, and Bash git config core.hooksPath, keel deactivate, exemption writes, and actor-identity mutation (keel actor set, Person enrollment, KEEL_ACTOR) route through permissionDecision ask plus an auto-recorded, orient-visible fact. Redirection-class Bash writes into .claude/ join the unambiguous block list. keel audit-history runs in CI over the pushed range - the tree-derived, hook-independent verification (kernel invariant K15) that catches control-plane vandalism even when every local hook was dead when it happened.";
        :>> rationale = "A control the controlled agent can legally disarm silently is not a control; but hard-denying control-plane writes would break legitimate maintenance, so the shape is human-approval-at-action plus durable record plus tree-derived backstop. The ask prompt is itself the human channel.";
        :>> consequences = "Guard-the-guards becomes real: every weakening is either human-approved or loudly recorded, and CI re-derives verdicts from the tree regardless. CLAUDE.md's rebase claim is softened to what is actually enforced instead of adding a pre-push hook. Charters p1ControlPlane and p1AuditHistoryCi.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0179Accept : Test { :>> id = "5acb36ee-a829-4263-b640-8f5104ac913e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'accepted all'. Said in response to the presented queue of fifteen proposed Decisions - D0174, D0175, D0176, D0177, D0178, D0179, D0180, D0181, D0182, D0183, D0184, D0185, D0186, D0187, D0188 - so 'all' is that queue and this acceptance covers this Decision as one of the fifteen. The open fork stated in the proposal (section 8-A, strict default for provably empty directories) was named to them alongside the queue with the invitation to object on D0174; they accepted without objection, so D0174 is accepted AS WRITTEN including its stated strict-for-empty default."; }
    part d0179AcceptR1 : TestResult { :>> id = "dc363abb-176a-488b-b82c-5c24dab83c6f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10b66bd"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
}
