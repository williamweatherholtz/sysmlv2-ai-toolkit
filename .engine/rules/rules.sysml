// =====================================================================
// EngineRuleSet — the engine's DECLARED controls (D0105 migration). These run via `keel check`
// ALONGSIDE the Rust guards during EXPAND; a guard retires only once its rule reaches PARITY
// (`keel check` == `keel guard` on the live repo — guardsToRulesMigration). Each rule carries its
// WHY as a typed #JustifiedBy edge to the Decision that justifies it (provenance, not prose).
// =====================================================================
package EngineRuleSet {
    private import EngineElement::*;
    private import EngineRules::*;
    private import EngineRelationships::*;

    // First migrated rule — mirrors guard:requirement-rootedness (D0099). Parity target:
    // view::capability_root_violations (a #Capability with no #DerivedFrom edge to a Need).
    part capabilityRootednessRule : EdgeRule {
        :>> id = "d0105r001-0001-4001-9001-516273841001";
        :>> title = "A user-facing capability must derive from a Need";
        :>> subjectType = "#Capability";
        :>> requiredEdge = EdgeKind::derivedFrom;
        :>> edgeDirection = EdgeDirection::outgoing;
        :>> objectType = "Need";
        :>> cardinality = RuleCardinality::atLeastOne;
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "all";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from capabilityRootednessRule to d0099;

    // Mirrors guard:issues (D0077/D0078). Parity target: view::untriaged_issues (an Issue with no
    // INCOMING #Resolves edge from a resolving action/Decision).
    part issuesTriagedRule : EdgeRule {
        :>> id = "d0105r002-0002-4002-9002-516273841002";
        :>> title = "Every Issue must be triaged (have a resolving edge)";
        :>> subjectType = "Issue";
        :>> requiredEdge = EdgeKind::resolves;
        :>> edgeDirection = EdgeDirection::incoming;
        :>> objectType = "*";
        :>> cardinality = RuleCardinality::atLeastOne;
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "all";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from issuesTriagedRule to d0078;

    // Mirrors guard:decision-rationale (D0103). Parity target: view::decisions_weak_rationale (a Decision
    // whose context OR rationale is blank/trivial — both must be a substantive >=20 chars).
    part decisionRationaleRule : ElementRule {
        :>> id = "d0105r003-0003-4003-9003-516273841003";
        :>> title = "A Decision must carry a substantive context + rationale (the why)";
        :>> subjectType = "Decision";
        :>> predicate = "minLength(context,20) and minLength(rationale,20)";
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "all";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from decisionRationaleRule to d0103;

    // Mirrors guard:acceptance-events (D0066). First rule using the SCOPE sub-language (whereStatus) + a
    // sibling-result predicate. Parity target: view::compute_attestation (a status=accepted Decision with
    // no passing <name>AcceptR1 event).
    part acceptanceEventRule : ElementRule {
        :>> id = "d0105r004-0004-4004-9004-516273841004";
        :>> title = "An accepted Decision must carry a passing acceptance event";
        :>> subjectType = "Decision";
        :>> predicate = "hasPassingResult(Accept)";
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "whereStatus(accepted)";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from acceptanceEventRule to d0066;

    // Mirrors guard:charter (D0068). First rule using the git-temporal `newlyAdded` scope. Parity target:
    // guards::charter (a Story in a newly-added .tracking/delivery file lacking a #CharteredBy edge).
    part charterRule : EdgeRule {
        :>> id = "d0105r005-0005-4005-9005-516273841005";
        :>> title = "A newly-added delivery Story must be chartered";
        :>> subjectType = "Story";
        :>> requiredEdge = EdgeKind::charteredBy;
        :>> edgeDirection = EdgeDirection::outgoing;
        :>> objectType = "*";
        :>> cardinality = RuleCardinality::atLeastOne;
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "newlyAdded";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from charterRule to d0068;
}
