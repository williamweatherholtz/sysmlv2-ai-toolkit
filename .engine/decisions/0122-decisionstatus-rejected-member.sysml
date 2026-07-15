// =====================================================================
// D0122 (PROPOSED — NOT YET ACCEPTED) — CR: add `rejected` to the DecisionStatus enum (schema/core).
// Enables the review loop's REJECT-with-rationale path (D0121): a rejected proposed Decision flips
// status -> rejected + records a {d}Reject judgment. schema/core is FROZEN, so this needs explicit
// human sign-off. Human-directed 2026-07-15 ("we have an accept button, but not a reject (with
// rationale)"). Additive; no migration.
// =====================================================================
package Decision0122 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0122 : Decision {
        :>> id = "10a1b0c1-4122-42a3-ab5c-516273849722";
        :>> title = "D0122 (CR): add DecisionStatus::rejected so the review loop can record a rejection (not just acceptance)";
        :>> createdAt = "2026-07-15";
        :>> createdBy = "claudeOpus";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human, reviewing the D0121 review-queue surface, noted (2026-07-15): 'we have an accept button, but not a reject (with rationale).' A review loop needs symmetric REJECT. Rejecting a proposed Decision should flip its status to rejected — but the DecisionStatus enum had only {proposed; accepted; superseded}, so a rejection could not be recorded as status. schema/core is FROZEN (D0002), so adding an enum member is a Change Request requiring explicit human sign-off. (Gate rejection needs NO schema change — it is a failing gate TestResult + rationale.)";
        :>> decision = "Add `rejected` to the DecisionStatus enum: {proposed; accepted; rejected; superseded}. A rejected PROPOSED Decision flips status proposed->rejected and gains a `{d}Reject : Test` (method=confirmation, procedureText = the human's rationale) + a `{d}RejectR1 : TestResult` (outcome=fail, judgedBy = the human Person) — symmetric with the accept event (D0121). The review queue lists only status=proposed Decisions + UNJUDGED confirmation gates, so accepted OR rejected items drop out (computed). Written via write::reject_decision behind POST /api/decision/reject; gate rejection via POST /api/gate-result verdict=fail + rationale (no schema change).";
        :>> rationale = "ALTERNATIVES: (i) record a rejection event but leave status=proposed (no new enum member) — REJECTED: dual-truth (the Decision IS rejected but status says proposed); the queue would need a side-channel to hide it. (ii) model rejection as a superseding Decision — REJECTED: `supersede` is for replacing an ACCEPTED commitment, not declining a PROPOSAL; far too heavy for a one-click UI reject. CHOSEN: add `rejected` — it is the natural proposal lifecycle (proposed -> accepted | rejected), minimal + additive (no existing instance uses it, so NO migration), and the codebase ALREADY referred to 'rejected / proposed Decisions are not active commitments' (view.rs) — the enum simply catches up to intent. schema/core frozen -> recorded as a CR with human sign-off.";
        :>> consequences = "schema/core change (frozen, D0002) — requires this Decision's explicit human acceptance BEFORE commit (CR: prefix). Additive enum member; no migration (grep confirms no instance authored DecisionStatus::rejected before this). Enables decision-Reject in the review queue (D0121). doc-sync: DecisionStatus members are not enumerated in prose docs, so no doc claim is invalidated. Follow-on (optional): a downstream project inherits the richer lifecycle."; }

    #DependsOn dependency from d0122 to d0121;

    verification d0122Accept : Test { :>> id = "10a1b0c1-4122-42a3-bb5c-516273849722"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz explicitly accepted the frozen-schema/core CR (2026-07-15, 'accepted'): add DecisionStatus::rejected to enable Reject-with-rationale in the review loop. The human's word IS the attestation (D0016/D0106)."; }
    part d0122AcceptR1 : TestResult { :>> id = "10a1b0c1-4122-42a3-cb5c-516273849722"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "16b106c"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "wweatherholtz"; }
}
