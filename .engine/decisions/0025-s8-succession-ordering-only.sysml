package Decision0025 {
    private import EngineWork::*;

    part d0025 : Decision {
        :>> id = "f3a4b5c6-d7e8-4f9a-ab1c-4d5e6f7a8b9c";
        :>> title = "S8 runtimeParser: add is_ordering_only to Succession AST node";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 8 (runtimeParser): replacing orient.rs text-scan with AST-based indexer. The existing Succession AST node does not preserve whether the edge was prefixed with #OrderingOnly. When parse_action_def_body encounters '#OrderingOnly first X then Y;' it loses the marker and stores it as a plain Succession. The text-scan in orient.rs correctly splits these into a separate ordering_only set (used to suppress false-suspect propagation). Without this field, the AST-based indexer cannot replicate that behavior, breaking parity on the actual project (schemaAudit -> testModel edge in backlog.sysml).";
        :>> decisionText = "Add `pub is_ordering_only: bool` to the Succession struct in ast.rs. Update parse_action_def_body to set is_ordering_only = true when '#OrderingOnly first X then Y;' is seen. Update parse_hash_item to parse '#Marker first X then Y;' at package level as Succession { is_ordering_only = (marker == OrderingOnly) } instead of skip. Update the single Succession literal construction. Update compute_orient_state in lib.rs to exclude ordering-only successions from the regular dep set. This is an additive change (new bool field, default false) — no existing semantics change. Accepted under standing sprint instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "Succession struct gains one bool field. All AST-based orient (lib.rs + indexer.rs) correctly distinguishes ordering-only edges. The schemaAudit -> testModel edge is correctly handled. Parity with orient.rs text-scan is achievable.";
    }
}
