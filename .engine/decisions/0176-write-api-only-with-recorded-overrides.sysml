// D0176 (status: the `status` field below is the truth) — D-P1a: API-owned facts are written only through the write API; override unlocks reach every tier and are recorded
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0176Accept event on sign-off.
package Decision0176 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0176 : Decision {
        :>> id = "010c3b5b-5b94-442d-9ba9-bb05ff605ca9";
        :>> title = "D-P1a: API-owned facts are written only through the write API; override unlocks reach every tier and are recorded";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "In-loop gating today only detects after the fact: no PreToolUse gate exists on writes, Bash writes bypass the edit gate entirely, and the write API covers ~9 fact operations while CLAUDE.md sanctions direct editing for the rest - so a blanket deny would deadlock against the Stop hook's fix-it demand and train the Bash bypass (the recorded issue076/081 dynamic). Reviewed by a 9-member adversarial panel (2 pessimists, 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) across three rounds under an honesty rule; round 3 verdict 9 ACCEPT / 0 OBJECT, uncompelled. Full evidence and finding-to-resolution map: docs/proposals/2026-08-21-production-readiness-design.md sections 10-12. NOTE per the 2026-08-14 panel precedent (.tracking/panel-critiques.sysml): the panelists are subagent contexts of the authoring model, a rigor framing, NOT independent critics - this record claims convergence, never critic-independence.";
        :>> decision = "Three tiers over .tracking writes: (1) hard deny - ABSENT A RECORDED OVERRIDE - on the API-owned fact surfaces (test results, gate results, issues, decisions, backlog task entries), the refusal naming the exact command; the override reaches this tier too, because repairing a corrupted API-owned file is not one of the 9 operations and a tier the unlock cannot reach recreates the deadlock. (2) permissionDecision ask for other .tracking writes - the harness prompt is itself a human channel; in headless launched runs, ask maps to a console proxy when the console is up, else auto-deny plus a recorded obligation on the approve queue. (3) recorded override for everything the API does not cover: single-use, target-path-bound, session-bound, reason required; the auto-recorded obligation fact names the path actually written and uses one-file-per-fact so recording cannot deadlock on the very file being repaired. Path matching normalizes Windows separators. Promotion of any advisory pattern to blocking is evidence-gated per D-PM.";
        :>> rationale = "Discipline lives in the action space, not in instructions (D0109), but the threat model is a cooperative-but-drifting agent, not an adversary: against Bash, action-space controls are friction whose job is to make drift rare, visible, and redirected onto the paved road - the guarantee remains layer-3 tree-derived verification. The tiering honors the repo's own accepted conditionality that blocking fires only where unambiguous (d0128-direction-requirements: deliberately undelivered until the turn-boundary gate has produced evidence).";
        :>> consequences = "The write API becomes the only UNRECORDED-bypass-free write path; every used escape hatch becomes a durable, surfaced signal (D0047 input). Charters p1TieredDeny and feeds D-PM's promotion loop.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0176Accept : Test { :>> id = "5e6c4bc9-bab3-4818-ad1c-da8302c92969"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'accepted all'. Said in response to the presented queue of fifteen proposed Decisions - D0174, D0175, D0176, D0177, D0178, D0179, D0180, D0181, D0182, D0183, D0184, D0185, D0186, D0187, D0188 - so 'all' is that queue and this acceptance covers this Decision as one of the fifteen. The open fork stated in the proposal (section 8-A, strict default for provably empty directories) was named to them alongside the queue with the invitation to object on D0174; they accepted without objection, so D0174 is accepted AS WRITTEN including its stated strict-for-empty default."; }
    part d0176AcceptR1 : TestResult { :>> id = "577dd5e1-8144-4d45-8c65-eaf4cf591b61"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10b66bd"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
}
