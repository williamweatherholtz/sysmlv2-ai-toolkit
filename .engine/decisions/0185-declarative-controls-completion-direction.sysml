// D0185 (PROPOSED — NOT YET ACCEPTED) — D-P3b (direction, not scope): complete the declarative-controls migration when a trigger fires
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0185Accept event on sign-off.
package Decision0185 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0185 : Decision {
        :>> id = "7ae56b2e-c314-4d85-bddf-26dcdc632d15";
        :>> title = "D-P3b (direction, not scope): complete the declarative-controls migration when a trigger fires";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::proposed;
        :>> context = "D0105 accepted the declarative-controls direction and the 2026-06-30 spike validated the collapse (3 rule kinds, a bounded scope sub-language, declared exemptions, a parity gate). But migrating 39 compiled guards to rules changes no downstream-observable behavior, D0054 names framework upkeep the number-one risk, and D-P1b already delivers the modularity goal without it. Reviewed by a 9-member adversarial panel (2 pessimists, 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) across three rounds under an honesty rule; round 3 verdict 9 ACCEPT / 0 OBJECT, uncompelled. Full evidence and finding-to-resolution map: docs/proposals/2026-08-21-production-readiness-design.md sections 10-12. NOTE per the 2026-08-14 panel precedent (.tracking/panel-critiques.sysml): the panelists are subagent contexts of the authoring model, a rigor framing, NOT independent critics - this record claims convergence, never critic-independence.";
        :>> decision = "Recorded as DIRECTION with named triggers, D0157 pattern - no scheduled slot. Triggers: the first downstream project that needs a built-in control changed; OR P5 requires typed artifact contracts; OR the human reraises. Scope when triggered: guard-to-rule parity migration behind the D0105 parity gate (guards without a rule expression remain compiled - the bar is declared-rules-gate and migrated-guards-match-parity, never 100 percent migration); GATE_ORDER derived from workflow topology; the succession authoring pass; typed producedArtifact and typed evaluability plus retirement of process-enforcement.toml and its namespace mismatch with activation.toml.";
        :>> rationale = "Deferring behind a fire-able trigger is honest scope discipline: the value is real but its consumer does not exist yet, and building framework ahead of demand is the JPL warning D0054 encodes.";
        :>> consequences = "The direction is preserved and cheap to act on; nothing in the committed plan strands it (P4b's actual prerequisite lives in D-P3a).";
    }
}
