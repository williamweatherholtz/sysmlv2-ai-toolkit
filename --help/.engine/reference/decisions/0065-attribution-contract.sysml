// =====================================================================
// D0065 — attribution-contract: the CONTRACT step of D0063's expand/migrate/contract.
// Drops the deprecated attribution fields now that the new ones are populated and the
// readers prefer them. Schema sign-off for the frozen-schema removal.
// First decision authored purely in the contracted schema (createdBy + decision +
// rationale; no authoredBy / decisionText).
// Accepted 2026-06-17 by wweatherholtz (pre-authorized via D0063 expand/migrate/contract;
// "continue to the end iteratively" — contract is self-evidencing, no separate gate).
// =====================================================================
package Decision0065 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0065 : Decision {
        :>> id = "63940516-2c7d-4e8f-c065-516273849565";
        :>> title = "D0065: attribution-model CONTRACT — drop authoredBy + decisionText; createdBy stays [0..1]; decision/rationale [1..1]";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0063 staged the attribution migration as expand -> migrate -> contract. Expand (sprint28) added createdBy/decision/rationale [0..1]; migrate (sprint28/32) populated createdBy on 367 sites, split decisionText into decision+rationale across 64 decisions, and pointed the readers (validate_actors, query.py, report.py) at the new fields. With no instance authoring the old fields and no reader requiring them, the contract step removes them. One open multiplicity question: D0063 said 'tighten toward [1..1]', but TestResult (an occurrence def :> Element) carries judgedBy, not a creator, and the kernel validator does not enforce mandatory-attribute presence anyway.";
        :>> decision = "Remove `authoredBy` from Element + the three Tracked* bases and `decisionText` from Decision (schema attribute definitions); strip their 367 + 64 instance assignments (contract_attr_fields.py). Set `decision` and `rationale` to [1..1] on Decision (all 64 decisions have them). Keep `createdBy` at [0..1] on Element — NOT [1..1].";
        :>> rationale = "createdBy [0..1] because some Element subtypes legitimately have no distinct creator: TestResult records judgedBy (the judge) as its actor, and forcing createdBy [1..1] on the Element base would mismodel every TestResult. decision/rationale [1..1] is safe and correct — every Decision is, by definition, a choice with a justification, and all 64 carry both. ALTERNATIVES: (A) createdBy [1..1] per D0063's 'toward [1..1]' — rejected: breaks the TestResult model (no creator distinct from judgedBy) and overstates an unenforced constraint; (B CHOSEN) createdBy [0..1], decision/rationale [1..1]. CRITERIA: model-correctness (TestResult) x faithful-to-the-concept (a Decision always has a decision+rationale).";
        :>> consequences = "schema/core/element.sysml + work.sysml: authoredBy and decisionText attribute definitions removed; createdBy [0..1]; decision/rationale [1..1]. Instances stripped (367 authoredBy + 64 decisionText) via the one-time contract_attr_fields.py. tracking-template.sysml example updated to decision/rationale. The attribution migration (D0060/D0061/D0063) is now COMPLETE through contract; remaining attrModel* tasks (acceptanceEvents, coverageView, guard) are about the acceptance-as-TestResult model, separate from field migration. validate_schema 14/14 + validate_instances + validate . + validate_actors green.";
    }

    verification d0065Accept : Test { :>> id = "9ebb9d3d-cd5b-41b4-b058-0c4a8daa048c"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0065 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0065AcceptR1 : TestResult { :>> id = "5097d375-2b46-4adb-88f9-e5fd5e73ac0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
