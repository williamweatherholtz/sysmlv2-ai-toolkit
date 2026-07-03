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
}
