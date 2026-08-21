// D0193 (PROPOSED — NOT YET ACCEPTED) — D0193: every control-relevant event emits a counted record - one coverage control for the family
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0193Accept event on sign-off.
package Decision0193 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0193 : Decision {
        :>> id = "ff89ea3d-0f41-460d-9ff0-79e03c2d2971";
        :>> title = "D0193: every control-relevant event emits a counted record - one coverage control for the family";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::proposed;
        :>> context = "Four instances of one family accumulated this week, all found by verification rather than by failure: issue203 (the build-time spec-pin enforcement point was missing from the inventory, its warn-allow residual unrecorded - and its check had been DEAD its whole life without anything noticing), issue205 (actor rebinding - a weakening srK07 itself names - leaves no durable record), issue208 (successful override consumptions emit no ledger line, so the K14 promotion evidence structurally under-reads override pressure), plus the sr13 sentinel itself (a control shipped in the OFF state, invisible). Sprint 398's retro set the two-instance bar for converting per-instance fixes into one control; the family is at four.";
        :>> decision = "PROPOSED: (a) a declared control-event list in .engine/contracts/control-events.toml - every event class whose occurrence is control-relevant (hook fires per event, override consumption, ask allow/deny, actor rebind, enforcement-point checks), each naming its required record (ledger event name, tracked fact, or inventory entry); (b) ONE warning-tier guard (control-event-coverage) that cross-checks the declaration against the surfaces that should emit - a declared event with no emitter in the enforcement-point inventory, or an emitter whose event name is undeclared, warns; (c) the three open per-instance resolvers (dcEnforcementPointSpecPin, dcActorRebindLedger, dcOverrideCountedOnSuccess) become the first three rows of the declaration and are delivered UNDER this decision rather than piecemeal, so the fixes and the control land as one shape.";
        :>> rationale = "D0047: the fourth instance of a class is past the point where per-instance patches are honest - each fix closes one invisible event while the class keeps minting new ones (this week alone minted three). A declared list makes absence VISIBLE (the D0136 idiom: not-adopted is reported, never silent), and folding the three open resolvers under it prevents three separate mechanisms where one belongs. Warning-tier per the guided profile; promotion cites the ledger like every other control.";
        :>> consequences = "IF ACCEPTED: one contract file, one guard (45), three existing backlog items re-chartered under this decision and delivered together; the K14 counted set becomes complete by construction rather than by sweep. COST: one more declaration to keep truthful, and the guard's cross-check is heuristic where emitters are Rust (it checks names against the inventory, not call sites). IF REJECTED: the three per-instance fixes proceed as chartered - the family stays closed member by member, and the fifth instance re-raises this.";
    }
}
