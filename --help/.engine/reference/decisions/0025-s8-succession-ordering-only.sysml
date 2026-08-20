package Decision0025 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0025 : Decision {
        :>> id = "f3a4b5c6-d7e8-4f9a-ab1c-4d5e6f7a8b9c";
        :>> title = "S8 runtimeParser: add is_ordering_only to Succession AST node";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 8 (runtimeParser): replacing orient.rs text-scan with AST-based indexer. The existing Succession AST node does not preserve whether the edge was prefixed with #OrderingOnly. When parse_action_def_body encounters '#OrderingOnly first X then Y;' it loses the marker and stores it as a plain Succession. The text-scan in orient.rs correctly splits these into a separate ordering_only set (used to suppress false-suspect propagation). Without this field, the AST-based indexer cannot replicate that behavior, breaking parity on the actual project (schemaAudit -> testModel edge in backlog.sysml).";
        :>> decision = "Add `pub is_ordering_only: bool` to the Succession struct in ast.rs; set is_ordering_only = true in parse_action_def_body when '#OrderingOnly first X then Y;' is seen; parse '#Marker first X then Y;' at package level in parse_hash_item as Succession { is_ordering_only = (marker == OrderingOnly) } instead of skip; update the single Succession literal construction; and update compute_orient_state in lib.rs to exclude ordering-only successions from the regular dep set.";
        :>> rationale = "The AST-based indexer replacing orient.rs's text-scan must preserve the #OrderingOnly marker, which the text-scan splits into a separate ordering_only set to suppress false-suspect propagation; without the field, parity breaks on the schemaAudit -> testModel edge in backlog.sysml. This is an additive change (new bool field, default false) — no existing semantics change. Accepted under standing sprint instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "Succession struct gains one bool field. All AST-based orient (lib.rs + indexer.rs) correctly distinguishes ordering-only edges. The schemaAudit -> testModel edge is correctly handled. Parity with orient.rs text-scan is achievable.";
    }

    verification d0025Accept : Test { :>> id = "c982c64f-9dd6-4bea-914e-18b5b7b6c814"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0025 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0025AcceptR1 : TestResult { :>> id = "c6599b41-3eec-4ed6-bb90-0bf59d3a83e8"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
